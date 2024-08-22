use models::{Book, NewBook, Store};

pub mod models;

pub fn establish_connection() -> Store {
    Store::new()
}

pub fn create_book(
    conn: &mut Store,
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

pub fn delete_book(conn: &mut Store, ids: &[i32]) -> i32 {
    conn.delete_book(ids)
}

pub fn get_book(conn: &mut Store, id: &i32) -> Option<Book> {
    conn.get_book(id)
}

pub fn get_books(conn: &mut Store) -> Vec<Book> {
    conn.get_books()
}

pub fn update_book(conn: &mut Store, id: i32, book: Book) -> Option<i32> {
    conn.update_book(id, book)
}
