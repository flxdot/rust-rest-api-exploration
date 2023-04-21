use aide::axum::ApiRouter;

mod hello;

pub fn build_router() -> ApiRouter {
    
    ApiRouter::new().nest("/hello", hello::routes())
}
