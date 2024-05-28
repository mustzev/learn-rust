use axum::{
    extract::{Json, Path},
    http::StatusCode,
};
use serde::Deserialize;
use serde_json::{json, Value};

#[derive(Deserialize)]
pub struct Input {
    a: f64,
    b: f64,
}

pub async fn calculate(
    Path(operation): Path<String>,
    Json(payload): Json<Input>,
) -> Result<Json<Value>, StatusCode> {
    let a = payload.a;
    let b = payload.b;

    let mut result;

    match operation {
        add => result = a + b,
        subtract => result = a - b,
        multiply => result = a * b,
        divide => result = a / b,
        _ => Err(StatusCode::BAD_REQUEST),
    }

    Ok(Json(json!({ "result": result })))
}
