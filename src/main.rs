// mod github;

// use crate::github::Github;
use dotenv::dotenv;
use experiment_github_star::tools::{
    agent_classifier::{self, RepositoryClassifier},
    github::Github,
};
use futures::{pin_mut, stream, StreamExt};
use itertools::Itertools;
use rig::completion::Prompt;
use std::{collections::HashMap, env, fs::OpenOptions, io::Write};
use tokio;
#[tokio::main]
async fn main() {
    dotenv().ok();

    let github_token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN must be set");
    // print!("Token: {}", github_token);
    let chunk_size = 1;
    let total_page = 100;
    let categories = vec![
        "ai".to_string(),
        "rust".to_string(),
        "webdev".to_string(),
        "fullstack".to_string(),
        "frontend".to_string(),
    ];

    struct RepositoryResult {
        name: String,
        category: String,
        url: String,
    }
    let github = Github::new(github_token);
    // let agent = agent_classifier::create_agent_classifier_simple();
    let agent_extractor: rig::extractor::Extractor<
        rig::providers::openai::CompletionModel,
        RepositoryClassifier,
    > = agent_classifier::create_agent_classifier_extractor(categories.clone());
    let stream = github.get_starred_repos_stream(total_page);

    pin_mut!(stream);
    let mut polling = stream.as_mut().chunks(chunk_size);
    let mut results: Vec<RepositoryResult> = Vec::new();

    let mut data_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("README.md")
        .expect("cannot open file");

    let mut contents: Vec<String> = Vec::new();
    contents.push("Experiment for llm classification to categorize github repository".to_string());

    while let Some(repos) = polling.next().await {
        for repo in repos {
            let desc = repo.description.unwrap_or("none".to_string());

            let repo_infomation = format!(
                "Name: {}\nDescription: {}\nTopics: {:?}\nID: {}\n",
                repo.name, desc, repo.topics, repo.id
            );
            let result = agent_extractor.extract(&repo_infomation).await;
            match result {
                Ok(result) => {
                    // println!("Result: {:?}", result);
                    // println!("Name: {}", result.name);
                    // println!("Category: {}", result.category);
                    // println!("Confidence: {}", result.confidence);
                    // println!("----------------------");
                    results.push(RepositoryResult {
                        name: result.name,
                        category: result.category,
                        url: repo.repo_url,
                    });
                }
                Err(err) => {
                    println!("Error: {}", err);
                }
            }
        }
        println!("each chunk ----------------------");
    }
    let formatted = results
        .iter()
        .filter(|result| categories.contains(&result.category))
        .fold(
            HashMap::<String, Vec<&RepositoryResult>>::new(),
            |mut grouped, result| {
                grouped
                    .entry(result.category.to_string())
                    .or_default()
                    .push(result);
                grouped
            },
        );

    for (category, repos) in formatted {
        println!("Category: {}", category);
        contents.push(format!("### {}", category));
        for repo in repos {
            println!("Name: {}", repo.name);
            println!("----------------------");
            contents.push(format!("- [{}]({})", repo.name, repo.url));
        }
        println!("----------------------");
    }

    let content_str = contents.join("\n");
    data_file
        .write_all(content_str.as_bytes())
        .expect("cannot write to file");

    println!("Done");
}
