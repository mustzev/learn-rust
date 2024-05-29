use std::time::Duration;

use crate::routes::test::test_router::make_test_router;
use crate::routes::test1::test1_router::make_test1_router;
use crate::utilities::handle_error::handle_timeout_error;
use axum::{error_handling::HandleErrorLayer, Router};
use tower::ServiceBuilder;

pub fn make_router() -> Router {
    Router::new()
        .nest("/test", make_test_router())
        .nest("/test1", make_test1_router())
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(handle_timeout_error))
                .timeout(Duration::from_secs(5)),
        )
}
