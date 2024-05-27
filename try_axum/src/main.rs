use crate::routes::router::make_router;
use axum::serve;
use tokio::net::TcpListener;
mod routes;

#[tokio::main]
async fn main() {
    let app = make_router();
    let listener = TcpListener::bind("0.0.0.0:9000").await.unwrap();
    serve(listener, app).await.unwrap();
}
