use csv::ReaderBuilder;
use std::collections::HashMap;
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, Write};
use std::path::Path;

use crate::tools::agent_classifier::{self, RepositoryClassifier};
use crate::tools::csv_repository::CsvRepository;

pub async fn execute() -> Result<(), Box<dyn Error>> {
    let file_name = "classified_repos.csv";
    let output_file_name = "README.md";
    let categories = vec![
        "ai".to_string(),
        "rust".to_string(),
        "webdev".to_string(),
        "fullstack".to_string(),
        "frontend".to_string(),
    ];
    let path = Path::new(file_name);
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_path(&path)
        .expect("Could not create CSV reader");

    let mut data_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(output_file_name)
        .expect("cannot open file");

    let mut classified_repos: Vec<CsvRepository> = Vec::new();

    // Write headers
    for result in rdr.deserialize() {
        let repo: CsvRepository = result?;

        println!("{:?}", &repo);
        let repo_infomation = format!(
            "Name: {}\nDescription: {}\nTopics: {:?}\nID: {}\n",
            repo.name, repo.description, repo.topics, repo.id
        );
        println!("{:?}", repo_infomation);
        classified_repos.push(repo);
    }

    let grouped_repositoies = classified_repos
        .iter()
        .filter(|result| match &result.category {
            Some(category) => categories.contains(category),
            None => false,
        })
        .fold(
            HashMap::<String, Vec<&CsvRepository>>::new(),
            |mut grouped, result| {
                grouped
                    .entry(result.category.clone().unwrap_or("other".to_string()))
                    .or_default()
                    .push(result);
                grouped
            },
        );

    for (category, repos) in grouped_repositoies {
        data_file.write_all(format!("### {}\n", category).as_bytes())?;

        for repo in repos {
            data_file.write_all(format!("- [{}]({})\n", repo.name, repo.repo_url).as_bytes())?;
        }
    }
    data_file.flush()?;
    Ok(())
}
