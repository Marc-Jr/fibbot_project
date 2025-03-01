mod config;
mod extract;
mod fibonacci;
mod github;
mod utils;
mod tests;

use config::Config;
use extract::extract_numbers;
use fibonacci::Fibonacci;
use github::post_comment;
use utils::format_fibonacci_response;
use std::env;

#[tokio::main]
async fn main() {
    let _config = Config::from_env();
    let sample_pr_text = "Numbers found: 2, 5, and 10.";

    let numbers = extract_numbers(sample_pr_text);
    let mut fib_calc = Fibonacci::new();
    let fib_results: Vec<u32> = numbers.iter().map(|&n| fib_calc.compute(n)).collect();

    let formatted_message = format_fibonacci_response(&numbers, &fib_results);

    println!("Computed Fibonacci Numbers: {}", formatted_message);

    let repo_owner = env::var("GITHUB_REPOSITORY_OWNER").unwrap_or_else(|_| "owner".to_string());
    let repo_name = env::var("GITHUB_REPOSITORY_NAME").unwrap_or_else(|_| "repo".to_string());
    let pr_number = 1; // Assume PR number 1 for testing

    if let Err(e) = post_comment(&repo_owner, &repo_name, pr_number, &formatted_message).await {
        eprintln!("Error posting comment: {}", e);
    }
}
