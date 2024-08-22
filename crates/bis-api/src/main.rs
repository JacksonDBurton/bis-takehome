mod api;
mod error;
use std::{env, io, sync::Mutex};

use actix_web::{
    middleware::Logger,
    web::{self},
    App, HttpServer,
};
use bis_in_memory::{establish_connection, models::NewBook};
use dotenvy::dotenv;

// Name of the application
const APP_NAME: &str = env!("CARGO_PKG_NAME");

// TODO: Declare store pool type
// TODO: Something about a session signing key

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    // Setup logger
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // TODO: work on database and pool
    // let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    // let pool = db::init_pool

    let conn = web::Data::new(api::MutStore {
        mtx: Mutex::new(establish_connection()),
    });
    conn.mtx.lock().unwrap().create_book(NewBook {
        title: "Jack Burton",
        author: "Also Jack Burton",
        date_published: &chrono::offset::Utc::now().naive_utc().date(),
    });

    //TODO: Fix start location
    log::info!("Starting HTTP server at http://localhost:8080");
    HttpServer::new(move || {
        log::debug!("Constructing {}", APP_NAME);

        // TODO: Review how I'm handling errors
        // let error_handlers = ErrorHandlers::new()
        //     .handler(
        //         http::StatusCode::INTERNAL_SERVER_ERROR,
        //         api::internal_server_error,
        //     )
        //     .handler(http::StatusCode::BAD_REQUEST, api::bad_request)
        //     .handler(http::StatusCode::NOT_FOUND, api::not_found);

        App::new()
            .app_data(conn.clone())
            .wrap(Logger::default())
            // TODO: Replace this with some sort of landing page or documentation
            .service(web::resource("/").route(web::get().to(api::get_books)))
            .service(
                web::scope("/bis")
                    .route("", web::get().to(api::get_books))
                    .route("/{book_id}", web::get().to(api::get_book))
                    .route("", web::post().to(api::create_book))
                    .route("", web::put().to(api::update_book))
                    .route("", web::delete().to(api::delete_book)),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
