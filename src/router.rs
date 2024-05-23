use axum::Router;
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
use tracing::Level;
use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;
use utoipa_swagger_ui::SwaggerUi;

use crate::{documentation::api_documentation::ApiDoc, AppState};

pub async fn api_router(app_state: AppState) -> Router {
    Router::new()
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
        )
        .merge(SwaggerUi::new("/v1/swagger-ui").url("/api/v1/openapi.json", ApiDoc::openapi()))
        .merge(RapiDoc::new("/api/v1/openapi.json").path("/v1/rapidoc"))
        .with_state(app_state.clone())
}
