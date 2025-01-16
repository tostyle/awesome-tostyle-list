mod github;

use crate::github::Github;
use dotenv::dotenv;
use experiment_github_star::usecases::agent_classifier::{self, RepositoryClassifier};
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
    let categories = vec![
        "ai".to_string(),
        "rust".to_string(),
        "webdev".to_string(),
        "fullstack".to_string(),
        "frontend".to_string(),
    ];
    let github = Github::new(github_token);
    let agent = agent_classifier::create_agent_classifier_simple();
    let agent_extractor = agent_classifier::create_agent_classifier_extractor(categories.clone());
    let stream = github.get_starred_repos_stream();

    pin_mut!(stream);
    let mut polling = stream.as_mut().chunks(chunk_size);
    let mut results: Vec<RepositoryClassifier> = Vec::new();

    let mut data_file = OpenOptions::new()
        .read(true)
        .append(false)
        .open("README.md")
        .expect("cannot open file");

    let mut contents: Vec<String> = Vec::new();
    contents.push("value".to_string());

    while let Some(repos) = polling.next().await {
        for repo in repos {
            let desc = repo.description.unwrap_or("none".to_string());
            // println!("Name: {}", repo.name);
            // println!("Description: {}", &desc);
            // // println!("Description: {}", repo.description);
            // println!("Topics: {:?}", repo.topics);
            // println!("ID: {}", repo.id);
            println!("----------------------");

            let repo_infomation = format!(
                "Name: {}\nDescription: {}\nTopics: {:?}\nID: {}\n",
                repo.name, desc, repo.topics, repo.id
            );
            // let result = agent.extract(&repo_infomation).await;
            // let result = agent.prompt(&repo_infomation).await;
            // match result {
            //     Ok(result) => {
            //         println!("Result: {}", result);
            //         // println!("Name: {}", result.name);
            //         // println!("Category: {}", result.category);
            //         // println!("Confidence: {}", result.confidence);
            //         println!("----------------------");
            //     }
            //     Err(err) => {
            //         println!("Error: {}", err);
            //     }
            // }
            let result = agent_extractor.extract(&repo_infomation).await;
            match result {
                Ok(result) => {
                    println!("Result: {:?}", result);
                    println!("Name: {}", result.name);
                    println!("Category: {}", result.category);
                    println!("Confidence: {}", result.confidence);
                    println!("----------------------");
                    results.push(result);
                }
                Err(err) => {
                    println!("Error: {}", err);
                }
            }
            // match repo {
            //     Some(repo) => {
            //         println!("Name: {}", repo.name);
            //         println!(
            //             "Description: {}",
            //             repo.description.unwrap_or("none".to_string())
            //         );
            //         println!("Topics: {:?}", repo.topics);
            //         println!("ID: {}", repo.id);
            //         println!("----------------------");
            //     }
            //     None => {
            //         println!("None");
            //     }
            // }
        }
        println!("each chunk ----------------------");
    }
    let formatted = results
        .iter()
        .filter(|result| categories.contains(&result.category))
        .fold(
            HashMap::<String, Vec<&RepositoryClassifier>>::new(),
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
        contents.push(format!("\n### {}", category));
        for repo in repos {
            println!("Name: {}", repo.name);
            println!("Confidence: {}", repo.confidence);
            println!("----------------------");
            contents.push(format!("- {}", repo.name));
        }
        println!("----------------------");
    }

    let content_str = contents.join("\n");
    data_file
        .write_all(content_str.as_bytes())
        .expect("cannot write to file");

    println!("Done");
    // let mut stream2 = stream::iter(vec![17, 19]);
    // while let Some(x) = stream2.next().await {
    //     println!("{:?}", x)
    // }
    //https://stackoverflow.com/questions/64005557/why-usage-of-async-block-in-then-makes-my-stream-unpin
    // stream
    //     .for_each(|repo| async {
    //         match repo {
    //             Ok(repo) => {
    //                 println!("Name: {}", repo.name);
    //                 println!("Description: {}", repo.description);
    //                 println!("Topics: {:?}", repo.topics);
    //                 println!("ID: {}", repo.id);
    //                 println!("----------------------");
    //             }
    //             Err(err) => {
    //                 println!("Error: {}", err);
    //             }
    //         }
    //     })
    //     .await;

    // stream.next().await;
    // while let Some(repo) = stream.next().await {
    //     match repo {
    //         Ok(repo) => {
    //             println!("Name: {}", repo.name);
    //             println!("Description: {}", repo.description);
    //             println!("Topics: {:?}", repo.topics);
    //             println!("ID: {}", repo.id);
    //             println!("----------------------");
    //         }
    //         Err(err) => {
    //             println!("Error: {}", err);
    //         }
    //     }
    // }
    // let res = github.getStarredRepos(None).await;
    // if let Ok(res) = res {
    //     // let text = res.text().await.unwrap();
    //     // println!("{:?}", text);
    //     let body = res.json::<Vec<github::Repository>>().await.unwrap();
    //     body.iter().for_each(|repo| {
    //         println!("Name: {}", repo.name);
    //         println!("Description: {}", repo.description);
    //         println!("Topics: {:?}", repo.topics);
    //         println!("ID: {}", repo.id);
    //         println!("----------------------");
    //     });
    //     // println!("{:?}", body);
    //     // println!("{:?}", res.);
    // } else {
    //     let err = res.err();
    //     println!("Error {}", err.unwrap());
    // }
}
