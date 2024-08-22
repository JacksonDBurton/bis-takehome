use std::net::TcpListener;

use actix_web::{dev::Server, middleware::Logger, web, App, HttpServer};
use bis_in_memory::models::Store;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{
    api::{self},
    docs::ApiDoc,
};

pub fn run(listener: TcpListener, pool: Store) -> Result<Server, std::io::Error> {
    let pool = web::Data::new(pool);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(web::resource("/").route(web::get().to(api::get_books)))
            .service(
                web::scope("/bis")
                    .route("", web::get().to(api::get_books))
                    .route("/{book_id}", web::get().to(api::get_book))
                    .route("", web::post().to(api::create_book))
                    .route("", web::put().to(api::update_book))
                    .route("", web::delete().to(api::delete_book)),
            )
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
            .app_data(pool.clone())
        // You can overwrite actix default behavior for validation errors (Serde) within json
        // here I chose to forgo improving this behavior but I want to acknowledge that the
        // default is not realisticly sufficient for production.
        // .app_data(web::JsonConfig::default().error_handler(|_, _| error::ErrorBadRequest("")))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
