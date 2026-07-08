use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, BufWriter, Write};
use std::path::{Path, PathBuf};

const DATASET_BASE: &str = "https://huggingface.co/datasets/lgi2p/finefreq/resolve/main";
const CACHE_DIR: &str = "data/finefreq";

/// Maximum pseudo-observation count when seeding a BitContext.
/// Keeping this small (≤4) lets the adaptive coder update quickly on real data
/// while still starting at a more informed probability than 50%.
const MAX_COUNT: u32 = 4;

fn download_file(url: &str, path: &Path) -> io::Result<()> {
    if path.exists() {
        return Ok(());
    }
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let response = ureq::get(url)
        .call()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    let mut tmp_path = path.to_path_buf();
    tmp_path.set_extension("tmp");
    {
        let mut file = BufWriter::new(File::create(&tmp_path)?);
        io::copy(&mut response.into_reader(), &mut file)?;
    }
    fs::rename(&tmp_path, path)?;
    Ok(())
}

fn process_csv(path: &Path, global_freqs: &mut HashMap<char, u64>) -> io::Result<()> {
    let mut rdr =
        csv::Reader::from_path(path).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    for result in rdr.records() {
        let record = result.map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        let (Some(char_str), Some(count_str)) = (record.get(4), record.get(7)) else {
            continue;
        };
        let mut chars = char_str.chars();
        let (Some(c), None) = (chars.next(), chars.next()) else {
            continue;
        };
        let Ok(count) = count_str.parse::<u64>() else {
            continue;
        };
        *global_freqs.entry(c).or_insert(0) += count;
    }
    Ok(())
}

struct LangEntry {
    rel_path: String,
    total_frequency: u64,
}

/// Returns the `True{n}False{m}` name for the BitContext with smallest n+m
/// whose probability (n+1)/(n+m+2) is closest to `p_true`.
fn best_bit_context(p_true: f64) -> String {
    let mut best = (0u32, 0u32);
    let mut best_err = (p_true - 0.5_f64).abs();
    for total in 0..=MAX_COUNT {
        for n in 0..=total {
            let m = total - n;
            let p = (n + 1) as f64 / (n + m + 2) as f64;
            let err = (p - p_true).abs();
            if err < best_err {
                best_err = err;
                best = (n, m);
            }
        }
    }
    format!("True{}False{}", best.0, best.1)
}

/// For a power-of-two frequency table, compute P(bit=1) at each internal node
/// of the binary tree used by BitsContext<N>. Node layout is heap-ordered:
/// node k covers [lo, hi), left child is 2k+1, right child is 2k+2.
fn tree_probs(freq: &[u64]) -> Vec<f64> {
    assert!(freq.len().is_power_of_two());
    let n = freq.len();
    let mut probs = vec![0.5_f64; n];
    fn fill(node: usize, lo: usize, hi: usize, freq: &[u64], probs: &mut Vec<f64>) {
        if hi - lo <= 1 {
            return;
        }
        let mid = (lo + hi) / 2;
        let right: u64 = freq[mid..hi].iter().sum();
        let total: u64 = freq[lo..hi].iter().sum();
        probs[node] = if total > 0 {
            right as f64 / total as f64
        } else {
            0.5
        };
        fill(2 * node + 1, lo, mid, freq, probs);
        fill(2 * node + 2, mid, hi, freq, probs);
    }
    fill(0, 0, n, freq, &mut probs);
    probs
}

fn write_bits_context<W: Write>(out: &mut W, probs: &[f64], indent: &str) -> io::Result<()> {
    writeln!(out, "BitsContext([")?;
    for p in probs {
        writeln!(out, "{indent}    BitContext::{},", best_bit_context(*p))?;
    }
    write!(out, "{indent}])")?;
    Ok(())
}

fn main() -> io::Result<()> {
    let mut args = std::env::args().skip(1);
    let top: Option<usize> = match (args.next().as_deref(), args.next().as_deref()) {
        (Some("--top"), Some(n)) => Some(n.parse().map_err(|_| {
            io::Error::new(io::ErrorKind::InvalidInput, "expected integer after --top")
        })?),
        (Some(a), _) => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("unknown argument: {a}\nUsage: char-freq [--top N]"),
            ));
        }
        _ => None,
    };

    // --- Download & cache ---

    let manifest_url = format!("{}/manifest/languages.csv", DATASET_BASE);
    let manifest_path = PathBuf::from(format!("{}/manifest/languages.csv", CACHE_DIR));
    eprintln!("Fetching manifest...");
    download_file(&manifest_url, &manifest_path)?;

    let mut manifest_rdr = csv::Reader::from_path(&manifest_path)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    let headers = manifest_rdr
        .headers()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?
        .clone();

    let find_col = |name: &str| {
        headers.iter().position(|h| h == name).ok_or_else(|| {
            eprintln!("Manifest columns: {:?}", headers.iter().collect::<Vec<_>>());
            io::Error::new(
                io::ErrorKind::Other,
                format!("{name} column not found in manifest"),
            )
        })
    };
    let path_col = find_col("relative_path")?;
    let freq_col = find_col("total_frequency")?;

    let all_langs: Vec<LangEntry> = manifest_rdr
        .records()
        .filter_map(|r| r.ok())
        .filter_map(|r| {
            let rel_path = r.get(path_col)?.to_string();
            let total_frequency = r.get(freq_col)?.parse().unwrap_or(0);
            Some(LangEntry {
                rel_path,
                total_frequency,
            })
        })
        .collect();

    let grand_total: u64 = all_langs.iter().map(|e| e.total_frequency).sum();

    let langs: &[LangEntry] = match top {
        Some(n) => {
            let n = n.min(all_langs.len());
            eprintln!(
                "Using top {} of {} languages (by total frequency)",
                n,
                all_langs.len()
            );
            &all_langs[..n]
        }
        None => {
            eprintln!("Using all {} languages", all_langs.len());
            &all_langs
        }
    };

    let selected_total: u64 = langs.iter().map(|e| e.total_frequency).sum();
    let coverage = selected_total as f64 / grand_total as f64 * 100.0;
    eprintln!(
        "Coverage: {:.2}% of all characters in the dataset",
        coverage
    );

    let mut global_freqs: HashMap<char, u64> = HashMap::new();

    for (i, entry) in langs.iter().enumerate() {
        let lang_code = entry
            .rel_path
            .split('/')
            .next_back()
            .unwrap_or(entry.rel_path.as_str());
        let file_rel = format!("{}/{}.csv", entry.rel_path, lang_code);
        let url = format!("{}/{}", DATASET_BASE, file_rel);
        let local_path = PathBuf::from(format!("{}/{}", CACHE_DIR, file_rel));
        eprint!("\r[{}/{}] {:<20}", i + 1, langs.len(), lang_code);
        if let Err(e) = download_file(&url, &local_path) {
            eprintln!("\nWarning: failed to download {}: {}", url, e);
            continue;
        }
        if let Err(e) = process_csv(&local_path, &mut global_freqs) {
            eprintln!("\nWarning: failed to process {:?}: {}", local_path, e);
        }
    }
    eprintln!();

    if global_freqs.is_empty() {
        eprintln!("No data accumulated — something went wrong.");
        std::process::exit(1);
    }

    // --- Aggregate frequency tables for each part of CharContext ---

    let mut ascii_freq = [0u64; 128];
    let mut chunk1_freq = [0u64; 32];
    let mut chunks0_freq = [0u64; 64];
    let mut chunks1_freq = [0u64; 64];
    let mut chunks2_freq = [0u64; 64];
    let mut ascii_total = 0u64;
    let mut non_ascii_total = 0u64;
    let mut n_chunks_ge1 = 0u64; // cp >= 2048
    let mut n_chunks_ge2 = 0u64; // cp >= 131072

    for (&c, &freq) in &global_freqs {
        let cp = c as u32;
        if cp < 128 {
            ascii_total += freq;
            ascii_freq[cp as usize] += freq;
        } else {
            non_ascii_total += freq;
            chunk1_freq[(cp & 0x1F) as usize] += freq;
            chunks0_freq[((cp >> 5) & 0x3F) as usize] += freq;
            if cp >= 2048 {
                n_chunks_ge1 += freq;
                chunks1_freq[((cp >> 11) & 0x3F) as usize] += freq;
                if cp >= 131072 {
                    n_chunks_ge2 += freq;
                    chunks2_freq[((cp >> 17) & 0x3F) as usize] += freq;
                }
            }
        }
    }

    // --- Compute probabilities ---

    let is_ascii_prob = if ascii_total + non_ascii_total > 0 {
        ascii_total as f64 / (ascii_total + non_ascii_total) as f64
    } else {
        0.5
    };

    // ULessThan<3> context uses only indices 0 and 3 (see ulessthan.rs trace).
    // Index 0: P(n_chunks >= 1)
    // Index 3: P(n_chunks >= 2 | n_chunks >= 1)
    let n_chunks_ctx0 = if non_ascii_total > 0 {
        n_chunks_ge1 as f64 / non_ascii_total as f64
    } else {
        0.5
    };
    let n_chunks_ctx3 = if n_chunks_ge1 > 0 {
        n_chunks_ge2 as f64 / n_chunks_ge1 as f64
    } else {
        0.5
    };

    let ascii_probs = tree_probs(&ascii_freq);
    let chunk1_probs = tree_probs(&chunk1_freq);
    let chunks0_probs = tree_probs(&chunks0_freq);
    let chunks1_probs = tree_probs(&chunks1_freq);
    let chunks2_probs = tree_probs(&chunks2_freq);

    // --- Generate src/v2/ulessthan/char_init.rs ---

    let out_path = "src/v2/ulessthan/char_init.rs";
    let mut out = BufWriter::new(File::create(out_path)?);

    writeln!(
        out,
        "// Auto-generated by `cargo run --bin char-freq --features csv,ureq -- --top {}`",
        top.unwrap_or(all_langs.len())
    )?;
    writeln!(
        out,
        "// Coverage: {:.2}% of FineFreq dataset. Do not edit by hand.",
        coverage
    )?;
    writeln!(out, "//")?;
    writeln!(
        out,
        "// Each BitContext is the lowest-count state whose P(true) best matches the"
    )?;
    writeln!(
        out,
        "// observed frequency. MAX_COUNT={MAX_COUNT} pseudo-observations keeps adaptation fast."
    )?;
    writeln!(out)?;
    writeln!(out, "use super::super::bit_context::BitContext;")?;
    writeln!(out, "use super::super::bits::BitsContext;")?;
    writeln!(out, "use super::super::string::CharContext;")?;
    writeln!(out, "use super::ULessThanContext;")?;
    writeln!(out)?;
    writeln!(
        out,
        "pub(crate) const INITIAL_CHAR_CONTEXT: CharContext = CharContext {{"
    )?;
    // is_ascii
    writeln!(out, "        // P(ASCII) = {:.1}%", is_ascii_prob * 100.0)?;
    writeln!(
        out,
        "        is_ascii: BitContext::{},",
        best_bit_context(is_ascii_prob)
    )?;
    // ascii tree
    writeln!(out, "        ascii: ")?;
    write!(out, "        ")?;
    write_bits_context(&mut out, &ascii_probs, "        ")?;
    writeln!(out, ",")?;
    // n_chunks (ULessThan<3>): array[split-1], so idx0=P(>=1), idx1=P(>=2|>=1), idx2=unused
    let nc0 = best_bit_context(n_chunks_ctx0);
    let nc1 = best_bit_context(n_chunks_ctx3);
    writeln!(
        out,
        "        // n_chunks: idx0=P(>=1)={:.1}%, idx1=P(>=2|>=1)={:.1}%",
        n_chunks_ctx0 * 100.0,
        n_chunks_ctx3 * 100.0
    )?;
    writeln!(out, "        n_chunks: ULessThanContext {{")?;
    writeln!(out, "            bits: [")?;
    writeln!(
        out,
        "                BitContext::{nc0},  // idx 0: P(n_chunks >= 1)"
    )?;
    writeln!(
        out,
        "                BitContext::{nc1},  // idx 1: P(n_chunks >= 2 | >= 1)"
    )?;
    writeln!(
        out,
        "                BitContext::True0False0,  // idx 2: unused (N-1)"
    )?;
    writeln!(out, "            ],")?;
    writeln!(out, "        }},")?;
    // chunk1
    writeln!(out, "        chunk1: ")?;
    write!(out, "        ")?;
    write_bits_context(&mut out, &chunk1_probs, "        ")?;
    writeln!(out, ",")?;
    // chunks[0..3]
    writeln!(out, "        chunks: [")?;
    for (probs, label) in [
        (&chunks0_probs, "bits 5-10 of codepoint"),
        (&chunks1_probs, "bits 11-16 (cp >= 2048)"),
        (&chunks2_probs, "bits 17-22 (cp >= 131072)"),
    ] {
        write!(out, "            // {label}\n            ")?;
        write_bits_context(&mut out, probs, "            ")?;
        writeln!(out, ",")?;
    }
    writeln!(out, "        ],")?;
    writeln!(out, "}};")?;

    drop(out);

    println!("Wrote {out_path}");
    println!(
        "Coverage: {:.2}% of all characters in the dataset",
        coverage
    );
    println!(
        "P(ASCII) = {:.1}%  →  {}",
        is_ascii_prob * 100.0,
        best_bit_context(is_ascii_prob)
    );
    println!("Top 10 chars by frequency:");
    let mut sorted: Vec<_> = global_freqs.iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(a.1));
    for (c, count) in sorted.iter().take(10) {
        println!("  {:?}: {}", c, count);
    }

    Ok(())
}
