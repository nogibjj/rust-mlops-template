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
    files: String,
}

//helper function that lists the files all the files in the EFS volume
async fn list_files() -> Result<String, Error> {
    let mut files = String::new();
    for entry in std::fs::read_dir("/mnt/efs")? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            files.push_str(&format!(
                "

{}",
                path.display()
            ));
        }
    }
    Ok(files)
}

async fn function_handler(event: LambdaEvent<Request>) -> Result<Response, Error> {
    // Extract some useful info from the request
    let name = event.payload.name;
    let files = list_files().await?;
    // Prepare the response
    let resp = Response {
        req_id: event.context.request_id,
        msg: format!("Hello, {}!", name),
        files,
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
