use rig::{
    agent::Agent,
    completion::Prompt,
    extractor::Extractor,
    providers::{self, openai::CompletionModel},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, JsonSchema, Serialize)]
pub struct RepositoryClassifier {
    pub category: String,
    pub id: String,
    pub name: String,
    // pub topics: Vec<String>,
    pub description: String,
    pub confidence: f32,
}

pub fn create_agent_classifier_extractor(
    categories: Vec<String>,
) -> Extractor<CompletionModel, RepositoryClassifier> {
    let client = providers::openai::Client::from_url("ollama", "http://localhost:11434/v1");
    let category_str = categories.join(" -");
    let prompt = format!(
        "You are an AI assistant specialized in classifying repository information into predefined categories. \
        The categories are: -{} \
        Use only category that I mentions before  \
        Only one category should be selected. \
        If information doesn't fit into these categories, use the 'other' category. \
        Summary result into single category and give confidence score rate 1-10. \
        If cannot get confidence score assign score to 0. \
        ",
        category_str
    );
    let agent = client
        .extractor::<RepositoryClassifier>("llama3.2")
        .preamble(&prompt)
        .build();
    agent
}

pub fn create_agent_classifier_simple() -> Agent<CompletionModel> {
    let client = providers::openai::Client::from_url("ollama", "http://localhost:11434/v1");
    let agent = client
        .agent("llama3.2")
        .preamble("You are an AI assistant specialized in classifying repository information into predefined categories. \
            The categories are: ai, rust, webdev, fullstack, frontend \
            Use only category I provided do not create new category. \
            Only one category should be selected. \
            If information doesn't fit into these categories, use the 'other' category. \
            If the information is not clear, use the 'unknown' category. \
            Provide a confidence score and a brief summary for each classification.")
        .build();
    agent
}
