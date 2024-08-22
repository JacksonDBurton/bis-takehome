use models::{Book, NewBook, Store};

pub mod models;

pub fn establish_connection() -> Store {
    Store::default()
}

pub fn create_book(
    conn: &Store,
    title: &str,
    author: &str,
    date_published: &chrono::NaiveDate,
) -> Book {
    let new_book = NewBook {
        title,
        author,
        date_published,
    };
    conn.create_book(new_book)
}

pub fn delete_book(conn: &Store, ids: &[i32]) -> i32 {
    conn.delete_book(ids)
}

pub fn get_book(conn: &Store, id: &i32) -> Option<Book> {
    conn.get_book(id)
}

pub fn get_books(conn: &Store) -> Vec<Book> {
    conn.get_books()
}

pub fn update_book(conn: &Store, id: i32, book: Book) -> Option<i32> {
    conn.update_book(id, book)
}
