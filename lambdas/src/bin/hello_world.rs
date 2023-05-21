use lambda_runtime::{service_fn, Error, LambdaEvent};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Deserialize)]
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
        .init();

    lambda_runtime::run(service_fn(|event: LambdaEvent<Request>| async {
        let request = event.payload;
        let response = Response {
            message: format!("Hello worlds, {}!", request.name),

            timestamp: SystemTime::now(),
        };
        Ok::<_, Error>(response)
    }))
    .await
}
