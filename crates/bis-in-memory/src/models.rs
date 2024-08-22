use std::{
    collections::{hash_map, HashMap},
    sync::Mutex,
};

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

// TODO: Consider your use of unwrap on the mutex here

#[derive(Default)]
pub struct Store {
    book_store: Mutex<HashMap<i32, Book>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Book {
    pub id: i32,
    pub title: String,
    pub author: String,
    #[serde(with = "simple_date_format")]
    #[schema(value_type = String, format = "%Y-%m-%d", example = "2024-08-22")]
    pub date_published: chrono::NaiveDate,
}

#[derive(Debug)]
pub struct NewBook<'a> {
    pub title: &'a str,
    pub author: &'a str,
    pub date_published: &'a chrono::NaiveDate,
}

impl Store {
    pub fn new() -> Self {
        Store {
            book_store: Mutex::new(HashMap::new()),
        }
    }

    pub fn create_book(
        &self,
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
        self.book_store.lock().unwrap().insert(raw_id, book.clone());
        book
    }

    pub fn update_book(&self, id: i32, book: Book) -> Option<i32> {
        if let hash_map::Entry::Occupied(mut e) = self.book_store.lock().unwrap().entry(id) {
            e.insert(Book { id, ..book });
            return Some(id);
        }
        None
    }

    pub fn get_book(&self, id: &i32) -> Option<Book> {
        self.book_store.lock().unwrap().get(id).cloned()
    }

    pub fn get_books(&self) -> Vec<Book> {
        self.book_store
            .lock()
            .unwrap()
            .clone()
            .into_values()
            .collect()
    }

    pub fn delete_book(&self, ids: &[i32]) -> i32 {
        let mut deleted = 0;
        for id in ids {
            if self.book_store.lock().unwrap().remove(id).is_some() {
                deleted += 1;
            }
        }

        deleted
    }

    pub fn max_id(&self) -> i32 {
        let mut max_id: i32 = 0;
        for id in self.book_store.lock().unwrap().keys() {
            if max_id < *id {
                max_id = *id;
            }
        }

        max_id
    }
}

pub mod simple_date_format {
    use chrono::NaiveDate;
    use serde::{Deserialize, Deserializer, Serializer};

    const FORMAT: &str = "%Y-%m-%d";

    pub fn serialize<S>(date: &NaiveDate, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let dt = NaiveDate::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)?;
        Ok(dt)
    }
}
