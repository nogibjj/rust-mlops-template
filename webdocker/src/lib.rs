/*A library that returns back random fruit */

use rand::Rng;

//create an const array of 10 fruits
pub const FRUITS: [&str; 10] = [
    "Apple",
    "Banana",
    "Orange",
    "Pineapple",
    "Strawberry",
    "Watermelon",
    "Grapes",
    "Mango",
    "Papaya",
    "Kiwi",
];

//create a function that returns a random fruit
pub fn random_fruit() -> &'static str {
    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0..FRUITS.len());
    FRUITS[random_index]
}
