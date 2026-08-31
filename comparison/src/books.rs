use serde::{Deserialize, Serialize};

#[derive(
    Debug, Serialize, Deserialize, compactly::v1::Encode, compactly::v2::Encode, Clone, PartialEq,
)]
pub struct Book {
    pub title: String,
    pub author: String,
    #[compactly(Compressible)]
    pub text: String,
}

pub fn books() -> Vec<Book> {
    vec![Book {
        title: "Pride and Prejudice".to_string(),
        author: "Jane Austen".to_string(),
        text: include_str!("pride-and-prejudice.txt").to_string(),
    }]
}

/// 三國演義 (Romance of the Three Kingdoms), a densely CJK (3-byte UTF-8)
/// corpus. Source: Project Gutenberg ebook 23950, body only.
pub fn three_kingdoms() -> Book {
    Book {
        title: "三國演義".to_string(),
        author: "羅貫中".to_string(),
        text: include_str!("three-kingdoms.txt").to_string(),
    }
}
