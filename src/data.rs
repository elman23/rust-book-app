use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Book {
    pub id: i32,
    pub author: String,
    pub title: String,
}

#[derive(Deserialize)]
pub struct BookRequest {
    pub author: String,
    pub title: String,
}

#[derive(Serialize)]
pub struct BookResponse {
    pub id: i32,
    pub author: String,
    pub title: String,
}

impl BookResponse {
    pub fn of(book: Book) -> BookResponse {
        BookResponse {
            id: book.id,
            author: book.author,
            title: book.title,
        }
    }
}
