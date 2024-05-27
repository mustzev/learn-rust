use crate::routes::test::test_router::make_test_router;
use crate::routes::test1::test1_router::make_test1_router;
use axum::Router;

pub fn make_router() -> Router {
    return Router::new()
        .nest("/test", make_test_router())
        .nest("/test1", make_test1_router());
}
