use aide::axum::{routing::get, ApiRouter, IntoApiResponse};

async fn hello() -> impl IntoApiResponse {
    return "Hello, world!";
}

pub fn routes() -> ApiRouter {
    return ApiRouter::new().api_route("/hello", get(hello));
}
