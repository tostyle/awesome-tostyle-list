use dotenv::dotenv;
use experiment_github_star::usecases::create_readme;

use std::env;

#[tokio::main]
async fn main() {
    let result = create_readme::execute().await;
    match result {
        Ok(_) => println!("Success"),
        Err(e) => println!("Error: {:?}", e),
    }
}
