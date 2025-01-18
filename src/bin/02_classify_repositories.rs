use dotenv::dotenv;
use experiment_github_star::usecases::classify_repositories;

use std::env;

#[tokio::main]
async fn main() {
    let result = classify_repositories::execute().await;
    match result {
        Ok(_) => println!("Success"),
        Err(e) => println!("Error: {:?}", e),
    }
}
