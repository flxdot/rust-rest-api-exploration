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

use mime_guess::mime;

pub fn docs_routes() -> ApiRouter {
    return ApiRouter::new()
        .route("/openapi.json", get(serve_docs))
        .route("/docs/*path", get(swagger_ui))
        .route(
            "/docs",
            get(|| swagger_ui(Path(String::from("index.html")))),
        );
}

struct SwaggerUiFiles<'a> {
    favicon_16x16: &'a [u8],
    favicon_32x32: &'a [u8],
    index_css: &'a [u8],
    index_html: &'a [u8],
    oauth2_redirect_html: &'a [u8],
    swagger_initializer_js: &'a [u8],
    swagger_ui_bundle_js: &'a [u8],
    swagger_ui_css: &'a [u8],
    swagger_ui_standalone_preset_js: &'a [u8],
}

const SWAGGER_FILE_CONTENTS: SwaggerUiFiles = SwaggerUiFiles {
    favicon_16x16: include_bytes!("static/swagger-ui/favicon-16x16.png"),
    favicon_32x32: include_bytes!("static/swagger-ui/favicon-32x32.png"),
    index_css: include_bytes!("static/swagger-ui/index.css"),
    index_html: include_bytes!("static/swagger-ui/index.html"),
    oauth2_redirect_html: include_bytes!("static/swagger-ui/oauth2-redirect.html"),
    swagger_initializer_js: include_bytes!("static/swagger-ui/swagger-initializer.js"),
    swagger_ui_bundle_js: include_bytes!("static/swagger-ui/swagger-ui-bundle.js"),
    swagger_ui_css: include_bytes!("static/swagger-ui/swagger-ui.css"),
    swagger_ui_standalone_preset_js: include_bytes!("static/swagger-ui/swagger-ui-standalone-preset.js"),
};


async fn swagger_ui(Path(path): Path<String>) -> impl IntoApiResponse {
    let path = path.trim_start_matches('/');

    // Looks a bit silly. But it also ensures, no other files from an actual directory can be
    // accessed.
    let (mime_type, content) = match path {
        "favicon-16x16.png" => (mime::IMAGE_PNG, SWAGGER_FILE_CONTENTS.favicon_16x16),
        "favicon-32x32.png" => (mime::IMAGE_PNG, SWAGGER_FILE_CONTENTS.favicon_32x32),
        "index.css" => (mime::TEXT_CSS_UTF_8, SWAGGER_FILE_CONTENTS.index_css),
        "index.html" => (mime::TEXT_HTML_UTF_8, SWAGGER_FILE_CONTENTS.index_html),
        "swagger-initializer.js" => (mime::APPLICATION_JAVASCRIPT, SWAGGER_FILE_CONTENTS.swagger_initializer_js),
        "swagger-ui-bundle.js" => (mime::APPLICATION_JAVASCRIPT, SWAGGER_FILE_CONTENTS.swagger_ui_bundle_js),
        "swagger-ui-standalone-preset.js" => (mime::APPLICATION_JAVASCRIPT, SWAGGER_FILE_CONTENTS.swagger_ui_standalone_preset_js),
        "swagger-ui.css" => (mime::TEXT_CSS_UTF_8, SWAGGER_FILE_CONTENTS.swagger_ui_css),
        "oauth2-redirect.html" => (mime::TEXT_HTML_UTF_8, SWAGGER_FILE_CONTENTS.oauth2_redirect_html),
        _ => return Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(body::boxed(Empty::new()))
            .unwrap(),
    };

    return Response::builder()
        .status(StatusCode::OK)
        .header(
            header::CONTENT_TYPE,
            HeaderValue::from_str(mime_type.as_ref()).unwrap(),
        )
        .body(body::boxed(Full::from(content)))
        .unwrap();

}

async fn serve_docs(Extension(api): Extension<Arc<OpenApi>>) -> impl IntoApiResponse {
    return Json(api);
}
