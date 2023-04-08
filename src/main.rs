use aide::{openapi::{OpenApi, Tag}, transform::TransformOpenApi};
use axum::{Extension, http::StatusCode};
use std::net::{Ipv4Addr, SocketAddr};
use std::sync::Arc;
use uuid::Uuid;
use crate::docs::docs_routes;
use errors::AppError;
use extractors::Json;

mod docs;
mod errors;
mod extractors;
mod services;

#[tokio::main]
async fn main() {
    let mut api = OpenApi::default();
    let app = services::build_router()
        .nest_api_service("/docs", docs_routes())
        .finish_api_with(&mut api, api_docs)
        .layer(Extension(Arc::new(api)));

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
            aide::openapi::SecurityScheme::ApiKey {
                location: aide::openapi::ApiKeyLocation::Header,
                name: "X-Auth-Key".into(),
                description: Some("A key that is ignored.".into()),
                extensions: Default::default(),
            },
        )
        .default_response_with::<Json<AppError>, _>(|res| {
            res.example(AppError {
                error: "some error happened".to_string(),
                error_details: None,
                error_id: Uuid::nil(),
                // This is not visible.
                status: StatusCode::IM_A_TEAPOT,
            })
        })
}
