// use std::error::Error;

use futures::{pin_mut, StreamExt};
use serde::{Deserialize, Serialize};

use crate::tools::github::{Github, Repository};

#[derive(Serialize, Deserialize, Debug)]
pub struct CsvRepository {
    pub id: i32,
    pub name: String,
    #[serde(
        serialize_with = "serialize_topics",
        deserialize_with = "deserialize_topics"
    )]
    pub topics: Vec<String>,
    pub description: String,
    pub repo_url: String,
    pub category: Option<String>,
}

impl From<Repository> for CsvRepository {
    fn from(repo: Repository) -> Self {
        CsvRepository {
            id: repo.id,
            name: repo.name,
            topics: repo.topics,
            description: repo.description.unwrap_or_else(|| "none".to_string()),
            repo_url: repo.repo_url,
            category: repo.category,
        }
    }
}

fn serialize_topics<S>(topics: &Vec<String>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let joined_topics = topics.join(",");
    serializer.serialize_str(&joined_topics)
}

fn deserialize_topics<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let topics = String::deserialize(deserializer)?;
    Ok(topics.split(", ").map(String::from).collect())
}
