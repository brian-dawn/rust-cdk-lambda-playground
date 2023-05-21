use lambda_runtime::{service_fn, Error, LambdaEvent};
use serde::{Deserialize, Serialize};
use std::alloc::System;
use std::time::SystemTime;

use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_dynamodb::Client;

#[derive(Deserialize, Debug)]
struct Request {
    name: String,
}

#[derive(Serialize)]
struct Response {
    message: String,
    timestamp: std::time::SystemTime,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        // disable ANSI colors because CloudWatch doesn't support them.
        .with_ansi(false)
        .init();

    // Place things here to have them persist between invocations.
    let shared_config = aws_config::load_from_env().await;
    let dynamo = Client::new(&shared_config);

    let table_name = std::env::var("TABLE_NAME")?;
    let id = "hello_world".to_string();

    lambda_runtime::run(service_fn(|event: LambdaEvent<Request>| async {
        let request = event.payload;

        // Log the request.
        tracing::info!("request: {:?}", request);

        let system_time_epoch_ms = SystemTime::UNIX_EPOCH
            .elapsed()
            .unwrap()
            .as_millis()
            .to_string();

        // Update the database.
        dynamo
            .put_item()
            .table_name(table_name.clone())
            .item("id", AttributeValue::S(id.to_string()))
            .item("timestamp", AttributeValue::S(system_time_epoch_ms))
            .send()
            .await?;

        let response = Response {
            message: format!("Hello worlds, {}!", request.name),

            timestamp: SystemTime::now(),
        };
        Ok::<_, Error>(response)
    }))
    .await
}
