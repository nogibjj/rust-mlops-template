/*
A library for a Microservice written in Actix
*/

//Build a function that reverses a string
pub fn reverse(input: &str) -> String {
    input.chars().rev().collect()
}

//build a function that creates pig latin from a string
pub fn pig_latin(input: &str) -> String {
    let mut pig_latin = String::new();
    let mut chars = input.chars();
    if let Some(first_char) = chars.next() {
        pig_latin.push_str(&first_char.to_string());
        pig_latin.push_str("ay");
        pig_latin.push_str(&chars.collect::<String>());
    }
    pig_latin
}

//build a function that converts a string to ones and zeros
pub fn binary(input: &str) -> String {
    let mut binary = String::new();
    for c in input.chars() {
        binary.push_str(&format!("{:08b}", c as u8));
    }
    binary
}
