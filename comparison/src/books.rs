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
