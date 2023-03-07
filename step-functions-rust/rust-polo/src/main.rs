use lambda_runtime::{run, service_fn, Error, LambdaEvent};

use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Request {
    payload: String,
}

#[derive(Serialize)]
struct Response {
    req_id: String,
    payload: String,
}

async fn function_handler(event: LambdaEvent<Request>) -> Result<Response, Error> {
    // Extract useful information from the request
    let name = event.payload.payload;
    let payload_body = if name == "Polo" {
        "YouWin!".to_string()
    } else {
        "YouLose".to_string()
    };
    tracing::info!("name {}", name);

    // Prepare the response
    let resp = Response {
        req_id: event.context.request_id,
        payload: payload_body,
    };

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
