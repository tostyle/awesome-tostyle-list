use std::path::Iter;

use futures::{stream, StreamExt};
// use futures::stream::{self, StreamExt};
use serde::{Deserialize, Serialize, Serializer};

use reqwest::{self, Response};

pub struct Github {
    apiKey: String,
}
pub fn print() {
    println!("Hello, world!");
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Repository {
    pub id: i32,
    pub name: String,
    #[serde(serialize_with = "serialize_topics")]
    pub topics: Vec<String>,
    pub description: Option<String>,
    #[serde(rename = "html_url")]
    pub repo_url: String, // pub description: String,
    pub category: Option<String>,
}
fn serialize_topics<S>(topics: &Vec<String>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let joined_topics = topics.join(",");
    serializer.serialize_str(&joined_topics)
}

impl Github {
    pub fn new(apiKey: String) -> Github {
        Github { apiKey }
    }
    fn getRepos(&self) {
        println!("Getting repos");
    }

    pub async fn get_starred_repos(
        &self,
        page: Option<i16>,
        max_page: Option<i16>,
    ) -> Result<Vec<Repository>, reqwest::Error> {
        let page = page.unwrap_or(1);
        let max_page = max_page.unwrap_or(10);
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::USER_AGENT,
            reqwest::header::HeaderValue::from_static("request"),
        );
        headers.insert(
            reqwest::header::AUTHORIZATION,
            reqwest::header::HeaderValue::from_str(&format!("Bearer {}", self.apiKey)).unwrap(),
        );
        println!("page {:?}", page);
        let client = reqwest::Client::new();
        let res = client
            .get("https://api.github.com/user/starred")
            .headers(headers)
            .query(&[("page", page), ("per_page", 10)])
            .send()
            .await?
            .json::<Vec<Repository>>()
            .await;

        return res;
    }
    pub fn get_starred_repos_stream(
        &self,
        total_page: i16,
    ) -> impl futures::Stream<Item = Repository> + '_ {
        stream::unfold(
            (self, 1, total_page),
            |(github, page, max_page)| async move {
                if page > max_page {
                    return None;
                }
                match github.get_starred_repos(Some(page), Some(max_page)).await {
                    Ok(repos) if repos.is_empty() => None,
                    // Ok(repos) => Some((stream::iter(repos.into_iter().map(Ok)), (github, page + 1))),
                    Ok(repos) => Some((stream::iter(repos), (github, page + 1, max_page))),
                    Err(err) => {
                        println!("Error: {}", err);
                        // Some(stream::once(async { None }), (github, page + 1))
                        Some((stream::iter(vec![]), (github, page + 1, max_page)))
                        // Some((stream::empty(), (github, page + 1)))
                    }
                }
            },
        )
        .flatten()
    }
}
