use axum::{routing::post, Router};
use tower_http::trace::TraceLayer;

use crate::controllers::{
    not_found,
    test::{test_get, test_post},
};

pub fn router() -> Router {
    Router::new()
        .route("/test", post(test_post).get(test_get))
        .fallback(not_found)
        .layer(TraceLayer::new_for_http())
}
