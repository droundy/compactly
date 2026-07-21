use serde::{Deserialize, Serialize};

/// One English↔Arabic dictionary entry. The English side is ASCII (1-byte
/// UTF-8) and the Arabic side is 2-byte UTF-8, so the dictionary densely
/// interleaves 1- and 2-byte characters — a non-ASCII corpus for exercising
/// the string/char encoding.
///
/// Source: ArabEyes word list
/// (github.com/usefksa/engAraDictionaryFrom_ArabEyes), extracted to a
/// tab-separated `english<TAB>arabic` list.
#[derive(
    Debug, Clone, PartialEq, Serialize, Deserialize, compactly::v1::Encode, compactly::v2::Encode,
)]
pub struct Entry {
    pub english: String,
    pub arabic: String,
}

const TSV: &str = include_str!("en-ar-dictionary.tsv");

/// The full English–Arabic dictionary (~87k entries).
pub fn dictionary() -> Vec<Entry> {
    TSV.lines()
        .filter_map(|line| {
            let (english, arabic) = line.split_once('\t')?;
            Some(Entry {
                english: english.to_string(),
                arabic: arabic.to_string(),
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_and_round_trips() {
        let d = dictionary();
        assert!(
            d.len() > 80_000,
            "expected the full dictionary, got {}",
            d.len()
        );
        assert_eq!(d[0].english, "10th");
        let sample = d[..1000].to_vec();
        let bytes = compactly::v2::encode(&sample);
        let back: Vec<Entry> = compactly::v2::decode(&bytes).unwrap();
        assert_eq!(sample, back);
    }
}
