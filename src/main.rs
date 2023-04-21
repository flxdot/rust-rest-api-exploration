use crate::docs::docs_routes;
use aide::{
    openapi::{OpenApi, Tag},
    transform::TransformOpenApi,
};
use axum::{http::StatusCode, Extension};
use errors::AppError;
use extractors::Json;
use std::net::{Ipv4Addr, SocketAddr};
use std::sync::Arc;
use tower_http::{compression::CompressionLayer, trace::TraceLayer};

use uuid::Uuid;
mod docs;
mod errors;
mod extractors;
mod services;

#[tokio::main]
async fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "tower_http=debug");
    };

    tracing_subscriber::fmt().init();

    let mut api = OpenApi::default();
    let app = services::build_router()
        .nest_api_service("", docs_routes())
        .finish_api_with(&mut api, api_docs)
        .layer(Extension(Arc::new(api)))
        .layer(CompressionLayer::new())
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 8080));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn api_docs(api: TransformOpenApi) -> TransformOpenApi {
    api.title("Aide axum Open API")
        .summary("An example Todo application")
        .description("test")
        .tag(Tag {
            name: "todo".into(),
            description: Some("Todo Management".into()),
            ..Default::default()
        })
        .security_scheme(
            "ApiKey",
            aide::openapi::SecurityScheme::OAuth2 {
                flows: aide::openapi::OAuth2Flows {
                    implicit: None,
                    password: Some(aide::openapi::OAuth2Flow::Password {
                        refresh_url: Some(String::from("https://sso.test.com/refresh")),
                        token_url: String::from("https://sso.test.com/login"),
                        scopes: indexmap::IndexMap::new(),
                    }),
                    authorization_code: None,
                    client_credentials: None,
                },
                description: Some(String::from(
                    "Defines the way you may authorize against the API.",
                )),
                extensions: indexmap::IndexMap::new(),
            },
        )
        .default_response_with::<Json<AppError>, _>(|res| {
            res.example(AppError {
                error: "some error happened".to_string(),
                error_details: None,
                error_id: Uuid::nil(),
                status: StatusCode::BAD_REQUEST,
            })
        })
}
