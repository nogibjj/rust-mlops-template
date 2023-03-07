use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

//const
const FILENAME: &str = "/efs/testfiles/";

#[derive(Deserialize)]
struct Request {
    command: String,
}

#[derive(Serialize)]
struct Response {
    req_id: String,
    msg: String,
}

/* Checksums all files in the given directory using Rayon and md5
Use the const FILENAME to set the directory to search
*/
fn checksum_parallel(files: Vec<String>) -> Result<HashMap<String, Vec<String>>, Error> {
    let checksums = std::sync::Mutex::new(HashMap::new());
    files.par_iter().for_each(|file| {
        let checksum = md5::compute(std::fs::read(file).unwrap());
        let checksum = format!("{:x}", checksum);
        let mut checksums = checksums.lock().unwrap();
        checksums
            .entry(checksum)
            .or_insert_with(Vec::new)
            .push(file.to_string());
    });
    Ok(checksums.into_inner().unwrap())
}
/*
Find all the files with more than one entry in the HashMap
*/
async fn find_duplicates(checksums: HashMap<String, String>) -> Vec<Vec<String>> {
    let mut duplicates = Vec::new();
    for (_checksum, files) in checksums {
        if files.len() > 1 {
            duplicates.push(files);
        }
    }
    duplicates
}

//run the duplicate search
async fn run_duplicate_search() -> Result<Vec<Vec<String>>, Error> {
    let files = std::fs::read_dir(FILENAME)?
        .map(|res| res.map(|e| e.path().to_str().unwrap().to_string()))
        .collect::<Result<Vec<_>, std::io::Error>>()?;
    let checksums = checksum_parallel(files)?;
    let duplicates = find_duplicates(checksums).await;
    Ok(duplicates)
}


async fn function_handler(event: LambdaEvent<Request>) -> Result<Response, Error> {
    // Extract some useful info from the request
    let command = event.payload.command;

    // Run the duplicate search
    let duplicates = run_duplicate_search().await?;

    // Prepare the response
    let resp = Response {
        req_id: event.context.request_id,
        msg: format!("Command {}.", command),
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
