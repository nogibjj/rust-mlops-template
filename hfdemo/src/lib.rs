// Add required imports
extern crate wikipedia;
use wikipedia::Wikipedia;
use wikipedia::http::default::Client;

use rust_bert::pipelines::summarization::SummarizationModel;

// Function to get the content from a Wikipedia page
pub fn get_wiki_content(page: &str) -> String {
    // Change to the correct type parameter for Wikipedia
    let wiki = Wikipedia::<Client>::default();
    let page = wiki.page_from_title(page.to_owned());

    // Check if the page exists before trying to get the content
    match page.get_content() {
        Ok(content) => content,
        Err(_) => String::from("Error: Could not retrieve the content."),
    }
}

// Function to summarize the content from a Wikipedia page
pub fn summarize_content(content: &str) -> String {
    let model = SummarizationModel::new(Default::default()).unwrap();

    // Remove the character limit
    let input = content
        .replace('\n', " ");

    // Convert to a vector of strings
    let input = vec![input];

    // Summarize the content
    let output = model.summarize(&input);

    // Check if there's a summary and return the first element of the vector
    output.get(0).unwrap_or(&String::from("Error: Could not generate a summary.")).clone()
}
