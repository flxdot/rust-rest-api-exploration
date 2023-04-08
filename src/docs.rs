use std::sync::Arc;

use aide::{
    axum::{routing::get, ApiRouter, IntoApiResponse},
    openapi::OpenApi,
};
use axum::{
    body::{self, Empty, Full},
    extract::Path,
    http::{header, HeaderValue, StatusCode},
    response::Response,
    Extension, Json,
};

use include_dir::{include_dir, Dir};

static SWAGGER_UI_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/static/swagger-ui");

pub fn docs_routes() -> ApiRouter {
    return ApiRouter::new()
        .route("/openapi.json", get(serve_docs))
        .route("/docs/*path", get(swagger_ui))
        .route("/docs", get(|| swagger_ui(Path(String::from("index.html")))));
}

async fn swagger_ui(Path(path): Path<String>) -> impl IntoApiResponse {
    let path = path.trim_start_matches('/');
    let mime_type = mime_guess::from_path(path).first_or_text_plain();

    return match SWAGGER_UI_DIR.get_file(path) {
        None => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(body::boxed(Empty::new()))
            .unwrap(),
Some(file) => Response::builder()
            .status(StatusCode::OK)
            .header(
                header::CONTENT_TYPE,
                HeaderValue::from_str(mime_type.as_ref()).unwrap(),
            )
            .body(body::boxed(Full::from(file.contents())))
            .unwrap(),
    };
}

async fn serve_docs(Extension(api): Extension<Arc<OpenApi>>) -> impl IntoApiResponse {
    return Json(api);
}
