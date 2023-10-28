use tracing::info;
use tracing::instrument;
use tracing::Level;

#[instrument(level = Level::INFO, name = "test_post")]
pub async fn test_post() -> String {
    info!("test_post");

    "Hello, world!".into()
}

#[instrument(level = Level::INFO, name = "test_get")]
pub async fn test_get() -> String {
    info!("test_get");

    "Hello, world!".into()
}

#[cfg(test)]
mod tests {
    use crate::routes::router;
    use axum::http::{self, StatusCode};
    use hyper::{Body, Request};
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_test_post() {
        let router = router();

        let response = router
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/test")
                    .body(Body::from("Hello, world!"))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body = String::from_utf8(body.to_vec()).unwrap();

        assert_eq!(body, "Hello, world!");
    }

    #[tokio::test]
    async fn test_test_get() {
        let router = router();

        let response = router
            .oneshot(
                Request::builder()
                    .method(http::Method::GET)
                    .uri("/test")
                    .body(Body::from("Hello, world!"))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body = String::from_utf8(body.to_vec()).unwrap();

        assert_eq!(body, "Hello, world!");
    }
}
