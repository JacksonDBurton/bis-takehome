use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        super::api::get_book,
    ),
    components(
        schemas(
            bis_in_memory::models::Book,
        )
    ),
    tags((name = "BasicAPI", description = "A very Basic API")),
)]
pub struct ApiDoc;
