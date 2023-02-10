use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Request {
    name: String,
}

#[derive(Serialize)]
struct Response {
    req_id: String,
    msg: String,
}

async fn function_handler(event: LambdaEvent<Request>) -> Result<Response, Error> {
    let name = event.payload.name;
    let mut result = String::new();
    for i in 1..11 {
        tracing::info!("Starting computation {}", i);
        // Perform a heavy computation
        let computation = (0..1000000).sum::<i32>();
        tracing::info!("Finished computation {}", i);

        result.push_str(&format!("Iteration {}: {}\n", i, computation));
    }
    // Prepare the response
    let resp = Response {
        req_id: event.context.request_id,
        msg: format!("{} says {}", name, result),
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
