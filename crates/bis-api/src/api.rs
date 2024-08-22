use std::sync::Mutex;

use actix_web::{error, web, HttpRequest, HttpResponse, Responder, Result};
use bis_in_memory::models::{simple_date_format, Book, NewBook, Store};
use serde::{Deserialize, Serialize};

// TODO: Handle serde errors

pub struct MutStore {
    pub mtx: Mutex<Store>,
}

#[derive(Deserialize)]
pub struct ReqNewBook {
    title: String,
    author: String,
    #[serde(with = "simple_date_format")]
    date_published: chrono::NaiveDate,
}

pub async fn create_book(
    pool: web::Data<MutStore>,
    info: web::Json<ReqNewBook>,
) -> Result<impl Responder> {
    let book = pool.mtx.lock().unwrap().create_book(NewBook {
        title: &info.title,
        author: &info.author,
        date_published: &info.date_published,
    });

    Ok(web::Json(book))
}

#[derive(Serialize)]
struct Id {
    id: i32,
}

pub async fn update_book(
    pool: web::Data<MutStore>,
    book: web::Json<Book>,
) -> Result<impl Responder> {
    let book_id = book.id;
    if let Some(id) = pool
        .mtx
        .lock()
        .unwrap()
        .update_book(book_id, book.into_inner())
    {
        Ok(HttpResponse::Ok().json(Id { id }))
    } else {
        Err(error::ErrorNotFound(format!(
            "No book with id: {}",
            book_id
        )))
    }
}

pub async fn get_books(pool: web::Data<MutStore>, _: HttpRequest) -> Result<impl Responder> {
    // TODO: Fix unwrap on mutex
    let book_list = pool.mtx.lock().unwrap().get_books();
    if book_list.is_empty() {
        Ok(HttpResponse::Ok().body("Book Store is empty"))
    } else {
        Ok(HttpResponse::Ok().json(book_list))
    }
}

pub async fn get_book(
    pool: web::Data<MutStore>,
    path: web::Path<(i32,)>,
) -> Result<impl Responder> {
    let book = pool.mtx.lock().unwrap().get_book(&path.0);
    Ok(web::Json(book))
}

#[derive(Deserialize)]
pub struct ListIds {
    ids: Vec<i32>,
}

pub async fn delete_book(
    pool: web::Data<MutStore>,
    del_list: web::Json<ListIds>,
) -> Result<impl Responder> {
    let del_count = pool
        .mtx
        .lock()
        .unwrap()
        .delete_book(&del_list.into_inner().ids);
    if del_count == 0 {
        Err(error::ErrorNotFound("No books were found for deletion"))
    } else {
        Ok(HttpResponse::Ok().body(format!("{} books were deleted", del_count)))
    }
}
