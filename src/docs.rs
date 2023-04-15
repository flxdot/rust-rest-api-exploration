use std::io::Read;
use std::sync::Arc;

use aide::{
    axum::{routing::get, ApiRouter, IntoApiResponse},
    openapi::OpenApi,
};
use axum::{
    body::{self, Empty, Full},
    extract::Path,
    http::{header, HeaderMap, HeaderValue, StatusCode},
    response::Response,
    Extension, Json,
};
use flate2::bufread::GzDecoder;

use mime_guess::mime;

pub fn docs_routes() -> ApiRouter {
    return ApiRouter::new()
        .route("/openapi.json", get(serve_docs))
        .route("/docs/*path", get(swagger_ui))
        .route(
            "/docs",
            get(|header: HeaderMap| swagger_ui(Path(String::from("index.html")), header)),
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
    favicon_16x16: include_bytes!("static/swagger-ui/favicon-16x16.png.gz"),
    favicon_32x32: include_bytes!("static/swagger-ui/favicon-32x32.png.gz"),
    index_css: include_bytes!("static/swagger-ui/index.css.gz"),
    index_html: include_bytes!("static/swagger-ui/index.html.gz"),
    oauth2_redirect_html: include_bytes!("static/swagger-ui/oauth2-redirect.html.gz"),
    swagger_initializer_js: include_bytes!("static/swagger-ui/swagger-initializer.js.gz"),
    swagger_ui_bundle_js: include_bytes!("static/swagger-ui/swagger-ui-bundle.js.gz"),
    swagger_ui_css: include_bytes!("static/swagger-ui/swagger-ui.css.gz"),
    swagger_ui_standalone_preset_js: include_bytes!(
        "static/swagger-ui/swagger-ui-standalone-preset.js.gz"
    ),
};

async fn swagger_ui(Path(path): Path<String>, header: HeaderMap) -> impl IntoApiResponse {
    let path = path.trim_start_matches('/');

    // Looks a bit silly. But it also ensures, no other files from an actual directory can be
    // accessed.
    let (mime_type, content) = match path {
        "favicon-16x16.png" => (mime::IMAGE_PNG, SWAGGER_FILE_CONTENTS.favicon_16x16),
        "favicon-32x32.png" => (mime::IMAGE_PNG, SWAGGER_FILE_CONTENTS.favicon_32x32),
        "index.css" => (mime::TEXT_CSS_UTF_8, SWAGGER_FILE_CONTENTS.index_css),
        "index.html" => (mime::TEXT_HTML_UTF_8, SWAGGER_FILE_CONTENTS.index_html),
        "swagger-initializer.js" => (
            mime::APPLICATION_JAVASCRIPT,
            SWAGGER_FILE_CONTENTS.swagger_initializer_js,
        ),
        "swagger-ui-bundle.js" => (
            mime::APPLICATION_JAVASCRIPT,
            SWAGGER_FILE_CONTENTS.swagger_ui_bundle_js,
        ),
        "swagger-ui-standalone-preset.js" => (
            mime::APPLICATION_JAVASCRIPT,
            SWAGGER_FILE_CONTENTS.swagger_ui_standalone_preset_js,
        ),
        "swagger-ui.css" => (mime::TEXT_CSS_UTF_8, SWAGGER_FILE_CONTENTS.swagger_ui_css),
        "oauth2-redirect.html" => (
            mime::TEXT_HTML_UTF_8,
            SWAGGER_FILE_CONTENTS.oauth2_redirect_html,
        ),
        _ => {
            return Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(body::boxed(Empty::new()))
                .unwrap()
        }
    };

    let response_builder = Response::builder().status(StatusCode::OK).header(
        header::CONTENT_TYPE,
        HeaderValue::from_str(mime_type.as_ref()).unwrap(),
    );

    // if the client accepts gzip, we can save some bandwidth
    if header
        .get(header::ACCEPT_ENCODING)
        .map_or(false, |v| v.to_str().unwrap_or("").contains("gzip"))
    {
        return response_builder
            .header(header::CONTENT_ENCODING, "gzip")
            .body(body::boxed(Full::from(content)))
            .expect("failed to build compressed swagger ui response");
    }

    // otherwise we just send the uncompressed file
    return response_builder
        .body(body::boxed(Full::from(
            decompress_gzip(content).expect("gzip decompression failed"),
        )))
        .expect("failed to uncompressed build swagger ui response");
}

fn decompress_gzip(input: &[u8]) -> Result<Vec<u8>, std::io::Error> {
    let mut decoder = GzDecoder::new(input);
    let mut output = Vec::new();
    decoder.read_to_end(&mut output)?;
    Ok(output)
}

async fn serve_docs(Extension(api): Extension<Arc<OpenApi>>) -> impl IntoApiResponse {
    return Json(api);
}
