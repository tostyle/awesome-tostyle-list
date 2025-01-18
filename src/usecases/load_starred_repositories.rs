use std::error::Error;

use futures::{pin_mut, StreamExt};
use serde::Serialize;

use crate::tools::{
    csv_repository::CsvRepository,
    github::{Github, Repository},
};

pub async fn execute(github_token: String) -> Result<(), Box<dyn Error>> {
    let chunk_size = 1;
    let total_page = 100;
    let file_name = "starred_repos.csv";
    let github = Github::new(github_token);
    let file = std::fs::File::create(file_name).expect("Could not create file");
    let mut writer = csv::Writer::from_writer(file);

    let stream = github.get_starred_repos_stream(total_page);

    pin_mut!(stream);
    let mut polling = stream.as_mut().chunks(chunk_size);
    while let Some(repos) = polling.next().await {
        for repo in repos {
            let csv_repo: CsvRepository = repo.into();
            writer.serialize(csv_repo)?;
        }
    }
    writer.flush()?;
    Ok(())
}
