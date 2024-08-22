use utoipa::OpenApi;

use crate::api;

#[derive(OpenApi)]
#[openapi(
    paths(
        super::api::create_book,
        super::api::update_book,
        super::api::get_books,
        super::api::get_book,
        super::api::delete_book,
    ),
    components(schemas(
        bis_in_memory::models::Book,
        bis_in_memory::models::NewBook,
        api::Id,
        api::ListIds,
    ))
)]
pub struct ApiDoc;
