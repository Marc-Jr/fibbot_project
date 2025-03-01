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
    let config = Config::from_env();
    let pr_body = env::var("INPUT_PR_BODY").unwrap_or_else(|_| "Numbers found: 2, 5, and 10.".to_string());

    let numbers = extract_numbers(&pr_body);
    println!("Extracted numbers are: {:?}", numbers);

    if config.enable_fib {
        let mut fib_calc = Fibonacci::new();
        let fib_results: Vec<u32> = numbers
            .iter()
            .filter(|&&n| n <= config.max_threshold)
            .map(|&n| fib_calc.compute(n))
            .collect();
        println!("Fibonacci results are: {:?}", fib_results);

        let formatted_message = format_fibonacci_response(&numbers, &fib_results);
        println!("Computed Fibonacci Numbers: {}", formatted_message);

        // Fetching the GitHub repository owner, name, and PR number
        let repo_owner = env::var("GITHUB_REPOSITORY_OWNER").unwrap_or_else(|_| "Marc-Jr".to_string());
        let repo_name = env::var("GITHUB_REPOSITORY_NAME").unwrap_or_else(|_| "fibbot_project".to_string());
        let pr_number: u64 = env::var("PR_NUMBER")
            .unwrap_or_else(|_| "1".to_string()) // default to 1 if not found
            .parse()
            .unwrap_or(1); // handle parsing failure

        println!("Using repository: {} / {}", repo_owner, repo_name);
        println!("Pull Request number: {}", pr_number);

        // Ensure PR_NUMBER is correctly retrieved and passed
        if let Err(e) = post_comment(&repo_owner, &repo_name, pr_number, &formatted_message).await {
            eprintln!("Error posting comment: {}", e);
        }
    } else {
        println!("Fibonacci computation is disabled.");
    }
}

