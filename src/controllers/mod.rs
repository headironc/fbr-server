pub(crate) mod test;

pub async fn not_found() -> String {
    "Not found".into()
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
                    .uri("/not-found")
                    .body(Body::from("Not found"))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body = String::from_utf8(body.to_vec()).unwrap();

        assert_eq!(body, "Not found");
    }
}
