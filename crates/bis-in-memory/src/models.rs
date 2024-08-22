use std::collections::{hash_map, HashMap};

#[derive(Default)]
pub struct Store {
    book_store: HashMap<i32, Book>,
}

#[derive(Debug, Clone)]
pub struct Book {
    pub id: i32,
    pub title: String,
    pub author: String,
    pub date_published: time::Date,
}

#[derive(Debug)]
pub struct NewBook<'a> {
    pub title: &'a str,
    pub author: &'a str,
    pub date_published: &'a time::Date,
}

impl Store {
    pub fn new() -> Self {
        Store {
            book_store: HashMap::new(),
        }
    }

    pub fn create_book(
        &mut self,
        NewBook {
            title,
            author,
            date_published,
        }: NewBook,
    ) -> Book {
        let raw_id = self.max_id() + 1;
        let book = Book {
            id: raw_id,
            title: title.to_string(),
            author: author.to_string(),
            date_published: *date_published,
        };
        self.book_store.insert(raw_id, book.clone());
        book
    }

    pub fn update_book(&mut self, id: i32, book: Book) -> Option<i32> {
        if let hash_map::Entry::Occupied(mut e) = self.book_store.entry(id) {
            e.insert(Book { id, ..book });
            return Some(id);
        }
        None
    }

    pub fn get_book(&mut self, id: &i32) -> Option<Book> {
        self.book_store.get(id).cloned()
    }

    pub fn get_books(&mut self) -> Vec<Book> {
        self.book_store.clone().into_values().collect()
    }

    pub fn delete_book(&mut self, ids: &[i32]) -> i32 {
        let mut deleted = 0;
        for id in ids {
            if self.book_store.remove(id).is_some() {
                deleted += 1;
            }
        }

        deleted
    }

    pub fn max_id(&self) -> i32 {
        let mut max_id: i32 = 0;
        for id in self.book_store.keys() {
            if max_id < *id {
                max_id = *id;
            }
        }

        max_id
    }
}
