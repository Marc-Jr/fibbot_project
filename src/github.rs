use octocrab::models::pulls::PullRequest;
use octocrab::Octocrab;
use std::env;

pub async fn post_comment(repo_owner: &str, repo_name: &str, pr_number: u64, message: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Retrieve the GitHub token from the environment
    let token = env::var("GITHUB_TOKEN")?;

    // Create an Octocrab client with the personal access token
    let octocrab = Octocrab::builder().personal_token(token).build()?;

    // Retrieve the pull request
    let pull_request: PullRequest = octocrab.pulls(repo_owner, repo_name).get(pr_number).await?;
    println!("Posting comment to PR: {:?}", pull_request);

    // Post the comment to the pull request
    match octocrab.issues(repo_owner, repo_name).create_comment(pr_number, message).await {
        Ok(_) => println!("Comment posted successfully"),
        Err(e) => return Err(e.into()),
    }

    Ok(())
}