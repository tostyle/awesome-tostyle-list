use rig::providers::{self, openai};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, JsonSchema, Serialize)]
/// A record representing a person
struct Person {
    /// The person's first name, if provided (null otherwise)
    pub first_name: String,
    /// The person's last name, if provided (null otherwise)
    pub last_name: String,
    /// The person's job, if provided (null otherwise)
    pub job: String,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Create OpenAI client
    let client = providers::openai::Client::from_url("ollama", "http://localhost:11434/v1");

    // Create extractor
    let data_extractor = client.extractor::<Person>("llama3.2").build();

    let person = data_extractor
        .extract("Hello my name is John Doe! I am a software engineer.")
        .await?;

    println!("GPT-4: {}", serde_json::to_string_pretty(&person).unwrap());

    Ok(())
}
