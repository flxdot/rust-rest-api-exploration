use aide::axum::ApiRouter;

mod hello;

pub fn build_router() -> ApiRouter {
    let router = ApiRouter::new().nest("/hello", hello::routes());
    return router;
}
