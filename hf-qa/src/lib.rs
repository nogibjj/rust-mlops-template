/*A question answer Rust library using Hugging Face */

use rust_bert::pipelines::question_answering::{QaInput, QuestionAnsweringModel};

//export a function to answer a question
pub fn answer_question(question: &str, context: &str) -> Vec<String> {
    //load a model
    let qa_model = QuestionAnsweringModel::new(Default::default()).unwrap();
    let question = QaInput {
        question: question.to_string(),
        context: context.to_string(),
    };
    //return output as a vector of strings
    let output = qa_model.predict(&[question], 1, 32);
    output[0].iter().map(|x| x.answer.to_string()).collect()
}
