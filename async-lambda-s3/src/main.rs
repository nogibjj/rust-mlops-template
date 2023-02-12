use lambda_runtime::{run, service_fn, Error, LambdaEvent};
//use lib.rs
use async_lambda_s3::{client, list_bucket_sizes};
use humansize::{format_size, DECIMAL};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Request {
    name: String,
}

#[derive(Serialize)]
struct Response {
    req_id: String,
    msg: String,
    size: String,
}

// a helper function to create human readable buckets sizes
async fn human_readable_size() -> String {
    let client = client().await.unwrap();
    //turn off verbose mode
    let verbose = Some(false);
    let bucket_sizes = list_bucket_sizes(&client, verbose).await.unwrap();
    let mut total_size = 0;
    for size in bucket_sizes {
        total_size += size;
    }
    let size = format_size(total_size as u64, DECIMAL);
    let result = format!("Total size of all buckets: {}", size);
    result
}

async fn function_handler(event: LambdaEvent<Request>) -> Result<Response, Error> {
    let name = event.payload.name;
    // get human readable bucket sizes
    let result = human_readable_size();

    // Prepare the response
    let resp = Response {
        req_id: event.context.request_id,
        size: result.await,
        msg: format!(r#"Event Payload {}."#, name),
    };

    // Return `Response` (it will be serialized to JSON automatically by the runtime)
    Ok(resp)
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

    run(service_fn(function_handler)).await
}
