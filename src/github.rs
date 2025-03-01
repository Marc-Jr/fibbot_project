use octocrab::Octocrab;

pub async fn post_comment(owner: &str, repo: &str, pr_number: u64, body: &str) -> Result<(), Box<dyn std::error::Error>> {
    let github = Octocrab::builder()
        .personal_token(std::env::var("GITHUB_TOKEN")?)
        .build()?;

    github.issues(owner , repo ,)
        .create_comment(pr_number, body )
        .await?;

    Ok(())
}
