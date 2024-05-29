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

    let mut result: Option<f64> = None;

    match operation.as_str() {
        "add" => result = Some(a + b),
        "subtract" => result = Some(a - b),
        "multiply" => result = Some(a * b),
        "divide" => result = Some(a / b),
        _ => {}
    };

    if result != None {
        Ok(Json(json!({ "result": Some(result) })))
    } else {
        Err(StatusCode::BAD_REQUEST)
    }
}
