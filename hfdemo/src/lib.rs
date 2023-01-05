//build a function that grabs the content from a wikipedia page
//Using wikipedia-rs crate
extern crate wikipedia;
use rust_bert::pipelines::summarization::SummarizationModel;

pub fn get_wiki_content(page: &str) -> String {
    let wiki = wikipedia::Wikipedia::<wikipedia::http::default::Client>::default();
    let page = wiki.page_from_title((page).to_owned());
    page.get_content().unwrap()
}

//build a function that summarizes the content from a wikipedia page
pub fn summarize_content(content: &str) -> String {
    let model = SummarizationModel::new(Default::default()).unwrap();
    //clean up input to max 500 characters
    let input = content
        .chars()
        .take(500)
        .collect::<String>()
        // remove newlines with spaces
        .replace('\n', " ");
    //convert to a vector of strings
    let input = vec![input];
    //summarize the content
    let output = model.summarize(&input);
    //return the first element of the vector
    output[0].clone()
}
