/* *///Information about the AWS S3 service
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::{Client, Error};

// Create a new AWS S3 client
pub async fn client() -> Result<Client, Error> {
    let region_provider = RegionProviderChain::first_try(None)
        .or_default_provider()
        .or_else("us-east-1");
    let shared_config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&shared_config);
    Ok(client)
}

/* return a list of all buckets in an AWS S3 account
*/

pub async fn list_buckets(client: &Client) -> Result<Vec<String>, Error> {
    //create vector to store bucket names
    let mut bucket_names: Vec<String> = Vec::new();
    let resp = client.list_buckets().send().await?;
    let buckets = resp.buckets().unwrap_or_default();
    //store bucket names in vector
    for bucket in buckets {
        bucket_names.push(bucket.name().unwrap().to_string());
    }
    Ok(bucket_names)
}

// Get the size of an AWS S3 bucket by summing all the objects in the bucket
// return the size in bytes
async fn bucket_size(client: &Client, bucket: &str) -> Result<i64, Error> {
    let resp = client.list_objects_v2().bucket(bucket).send().await?;
    let contents = resp.contents().unwrap_or_default();
    //store in a vector
    let mut sizes: Vec<i64> = Vec::new();
    for object in contents {
        sizes.push(object.size());
    }
    let total_size: i64 = sizes.iter().sum();
    println!("Total size of bucket {} is {} bytes", bucket, total_size);
    Ok(total_size)
}

/* Use list_buckets to get a list of all buckets in an AWS S3 account
return a vector of all bucket sizes.
If there is an error continue to the next bucket only print if verbose is true
Return the vector
*/
pub async fn list_bucket_sizes(client: &Client, verbose: Option<bool>) -> Result<Vec<i64>, Error> {
    let verbose = verbose.unwrap_or(false);
    let buckets = list_buckets(client).await.unwrap();
    let mut bucket_sizes: Vec<i64> = Vec::new();
    for bucket in buckets {
        match bucket_size(client, &bucket).await {
            Ok(size) => bucket_sizes.push(size),
            Err(e) => {
                if verbose {
                    println!("Error: {}", e);
                }
            }
        }
    }
    Ok(bucket_sizes)
}
