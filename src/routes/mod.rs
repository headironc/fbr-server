use axum::{routing::post, Router};
use tower_http::trace::TraceLayer;

use crate::controllers::test::{test_get, test_post};

pub fn router() -> Router {
    Router::new()
        .route("/test", post(test_post).get(test_get))
        .layer(TraceLayer::new_for_http())
}
