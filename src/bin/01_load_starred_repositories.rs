use dotenv::dotenv;
use experiment_github_star::usecases::load_starred_repositories;

use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let github_token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN must be set");
    let result = load_starred_repositories::execute(github_token).await;
    match result {
        Ok(_) => println!("Success"),
        Err(e) => println!("Error: {:?}", e),
    }
}
