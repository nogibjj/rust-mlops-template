/* Randomly recommend a popular Rust crate to the user.
*/

// Import the rand crate.
extern crate rand;
const BOTNAME: &str = "User-Agent: rust-ideas";

use crates_io_api::SyncClient;

// get most popular crates from crates.io using crates_io_api
//accept a number of crates to return
pub fn get_popular_crates(count_crates: usize) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let client = SyncClient::new(BOTNAME, std::time::Duration::from_millis(1000)).unwrap();
    let mut popular_crates = Vec::new();
    let summary = client.summary()?;
    //pass in number of crates to return
    for c in summary.most_downloaded.iter().take(count_crates) {
        popular_crates.push(c.name.clone());
    }
    Ok(popular_crates)
}

//randomly select a crate from the list of popular crates
pub fn get_random_crate() -> Result<String, Box<dyn std::error::Error>> {
    let popular_crates = get_popular_crates(10)?;
    let random_crate = rand::random::<usize>() % popular_crates.len();
    Ok(popular_crates[random_crate].clone())
}
