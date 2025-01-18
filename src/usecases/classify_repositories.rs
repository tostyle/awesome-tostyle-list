use csv::ReaderBuilder;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use crate::tools::agent_classifier::{self, RepositoryClassifier};
use crate::tools::csv_repository::CsvRepository;

pub async fn execute() -> Result<(), Box<dyn Error>> {
    let file_name = "starred_repos.csv";
    let output_file_name = "classified_repos.csv";
    let categories = vec![
        "ai".to_string(),
        "rust".to_string(),
        "webdev".to_string(),
        "fullstack".to_string(),
        "frontend".to_string(),
        "bevy".to_string(),
    ];
    let path = Path::new(file_name);
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_path(&path)
        .expect("Could not create CSV reader");

    let output_path = Path::new(output_file_name);
    let mut wtr = csv::Writer::from_path(&output_path)?;

    let agent_extractor: rig::extractor::Extractor<
        rig::providers::openai::CompletionModel,
        RepositoryClassifier,
    > = agent_classifier::create_agent_classifier_extractor(categories.clone());

    // Write headers
    for result in rdr.deserialize() {
        let repo: CsvRepository = result?;

        println!("{:?}", &repo);
        let repo_infomation = format!(
            "Name: {}\nDescription: {}\nTopics: {:?}\nID: {}\n",
            repo.name, repo.description, repo.topics, repo.id
        );
        println!("{:?}", repo_infomation);
        let result = agent_extractor.extract(&repo_infomation).await;
        match result {
            Ok(result) => {
                let classified_repo = CsvRepository {
                    id: repo.id,
                    name: repo.name,
                    topics: repo.topics,
                    description: repo.description,
                    repo_url: repo.repo_url,
                    category: Some(result.category),
                };
                wtr.serialize(classified_repo)?;
            }
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }
        println!("----------------------");
    }
    wtr.flush()?;
    Ok(())
}
