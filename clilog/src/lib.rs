/*Random fruit generator
*/
use rand::Rng;

//array of fruits
const FRUITS: [&str; 5] = ["apple", "banana", "orange", "pear", "strawberry"];

//function returns a random fruit and logs it to the console
pub fn random_fruit() -> String {
    //randomly select a fruit
    let fruit = FRUITS[rand::thread_rng().gen_range(0..5)];
    //log the fruit
    log::info!("fruit-info: {}", fruit);
    log::trace!("fruit-trace: {}", fruit);
    log::warn!("fruit-warn: {}", fruit);
    fruit.to_string()
}
