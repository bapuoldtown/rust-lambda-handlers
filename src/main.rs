use lambda_runtime::{service_fn, Error, LambdaEvent};
use serde_json::{json, Value};

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_runtime::run(service_fn(handler)).await
}

async fn handler(event: LambdaEvent<Value>) -> Result<Value, Error> {
    let name = event.payload["name"].as_str().unwrap_or("world");
    Ok(json!({ "greeting": format!("Hello, {name}") }))
}
