use aide::axum::{routing::get, ApiRouter, IntoApiResponse};

async fn hello() -> impl IntoApiResponse {
    "Hello, world!"
}

pub fn routes() -> ApiRouter {
    ApiRouter::new().api_route("/hello", get(hello))
}
