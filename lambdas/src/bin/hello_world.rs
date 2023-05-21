use anyhow::Result;
use lambda_runtime::{service_fn, Error, LambdaEvent};
use serde::{Deserialize, Serialize};
use serde_dynamo::to_item;
use std::alloc::System;
use std::collections::HashMap;
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

#[derive(Serialize, Deserialize)]
pub struct DbItem {
    pub id: String,
    pub timestamp: String,
}

impl DbItem {
    fn to_item(&self) -> Result<HashMap<String, AttributeValue>> {
        let item = to_item(&self)?;

        Ok(item)
    }

    fn to_some_item(&self) -> Result<Option<HashMap<String, AttributeValue>>> {
        let item = self.to_item()?;

        Ok(Some(item))
    }
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
            .set_item(
                DbItem {
                    id: id.clone(),
                    timestamp: system_time_epoch_ms.clone(),
                }
                .to_some_item()?,
            )
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
