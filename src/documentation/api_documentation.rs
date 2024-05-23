use utoipa::OpenApi;

use super::api_security_addon::SecurityAddon;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Mountain Backpackers CMS API Documentation",
        version = "1.0.0",
        description = "The API documentation for the Mountain Backpackers CMS.",
        contact(
            name = "Chairman",
            email = "chairman@mountainbackpackers.co.za"
        ),
        license(
            name = "GPL-3.0"
        )
    ),
    paths(),
    components(
        schemas()
    ),
    modifiers(&SecurityAddon),
    tags(),
    servers(
        (
            url = "http://localhost:4000/api",
            description = "Development Server"
        )
    )
)]
pub struct ApiDoc;
