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

pub fn docs_routes() -> ApiRouter {
    // We infer the return types for these routes
    // as an example.
    //
    // As a result, the `serve_redoc` route will
    // have the `text/html` content-type correctly set
    // with a 200 status.
    aide::gen::infer_responses(true);

    let router = ApiRouter::new()
        .route("/openapi.json", get(serve_docs))
        .route("/docs/*path", get(swagger_ui));

    // Afterwards we disable response inference because
    // it might be incorrect for other routes.
    aide::gen::infer_responses(false);

    return router;
}

async fn serve_docs(Extension(api): Extension<Arc<OpenApi>>) -> impl IntoApiResponse {
    return Json(api);
}
