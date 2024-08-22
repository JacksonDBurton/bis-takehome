use std::sync::Mutex;

use actix_web::{error, web, HttpRequest, HttpResponse, Responder, Result};
use bis_in_memory::models::{simple_date_format, Book, NewBook, Store};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

// TODO: Handle serde errors

#[derive(Deserialize)]
pub struct ReqNewBook {
    title: String,
    author: String,
    #[serde(with = "simple_date_format")]
    date_published: chrono::NaiveDate,
}

pub async fn create_book(
    pool: web::Data<Store>,
    info: web::Json<ReqNewBook>,
) -> Result<impl Responder> {
    let book = pool.create_book(NewBook {
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

pub async fn update_book(pool: web::Data<Store>, book: web::Json<Book>) -> Result<impl Responder> {
    let book_id = book.id;
    if let Some(id) = pool.update_book(book_id, book.into_inner()) {
        Ok(HttpResponse::Ok().json(Id { id }))
    } else {
        Err(error::ErrorNotFound(format!(
            "No book with id: {}",
            book_id
        )))
    }
}

pub async fn get_books(pool: web::Data<Store>, _: HttpRequest) -> Result<impl Responder> {
    // TODO: Fix unwrap on mutex
    let book_list = pool.get_books();
    if book_list.is_empty() {
        Ok(HttpResponse::Ok().body("Book Store is empty"))
    } else {
        Ok(HttpResponse::Ok().json(book_list))
    }
}

// TODO: Should not return an option
#[utoipa::path(
    get,
    path = "/bis/{book_id}",
    responses(
        (status = 200, description = "Book found successfully", body = Book),
        (status = 404, description = "No book found with matching id", body = String),
    ),
    params(
        ("id" = i32, Path, description = "Book ID to retrieve from Database")
    )
)]
pub async fn get_book(pool: web::Data<Store>, path: web::Path<(i32,)>) -> Result<impl Responder> {
    if let Some(book) = pool.get_book(&path.0) {
        Ok(web::Json(book))
    } else {
        Err(error::ErrorNotFound(format!("No book with id: {}", path.0)))
    }
}

#[derive(Deserialize)]
pub struct ListIds {
    ids: Vec<i32>,
}

pub async fn delete_book(
    pool: web::Data<Store>,
    del_list: web::Json<ListIds>,
) -> Result<impl Responder> {
    let del_count = pool.delete_book(&del_list.into_inner().ids);
    if del_count == 0 {
        Err(error::ErrorNotFound("No books were found for deletion"))
    } else {
        Ok(HttpResponse::Ok().body(format!("{} books were deleted", del_count)))
    }
}
