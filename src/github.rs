use octocrab::models::issues::Comment;
use octocrab::Octocrab;
use std::env;

pub async fn post_comment(
    repo_owner: &str, 
    repo_name: &str, 
    pr_number: u64, 
    message: &str
) -> Result<(), Box<dyn std::error::Error>> {
    // Get the GitHub token from environment
    let token = env::var("GITHUB_TOKEN")?;

    // Initialize Octocrab with the token
    let octocrab = Octocrab::builder().personal_token(token).build()?;

    // Fetch the pull request to ensure it's valid
    let pull_request = octocrab.pulls(repo_owner, repo_name).get(pr_number).await;
    match pull_request {
        Ok(pr) => println!("Pull Request fetched: {:?}", pr),
        Err(e) => {
            eprintln!("Error fetching PR: {}", e);
            return Err(e.into());
        }
    }

    // Try to create a comment
    match octocrab.issues(repo_owner, repo_name).create_comment(pr_number, message).await {
        Ok(Comment { id, .. }) => {
            println!("Comment successfully posted with ID: {}", id);
            Ok(())
        }
        Err(e) => {
            eprintln!("Error posting comment: {}", e);
            Err(e.into())
        }
    }
}
