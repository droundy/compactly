use compactly::v2::Ans;

static BOOK: &str = include_str!("../../comparison/src/pride-and-prejudice.txt");

#[derive(compactly::v2::Encode, PartialEq, Debug)]
struct Book {
    title: String,
    author: String,
    #[compactly(compactly::Compressible)]
    text: String,
}

fn main() {
    let book = Book {
        title: "Pride and Prejudice".to_string(),
        author: "Jane Austen".to_string(),
        text: BOOK.to_string(),
    };
    let start = std::time::Instant::now();
    let encoded = std::hint::black_box(Ans::encode(&book));
    let elapsed = start.elapsed();
    println!("Encoded {} bytes in {:.2}s", encoded.len(), elapsed.as_secs_f64());
    let decoded = Ans::decode::<Book>(&encoded).expect("decode failed");
    assert_eq!(decoded, book, "round-trip failed");
    println!("Round-trip OK");
}
