use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Repo {
    pub name: String,
    pub stargazers_count: u32,
    pub language: Option<String>,
    pub description: Option<String>,
    pub fork: bool,
    pub html_url: String,
}

pub async fn fetch_repos(username: &str) -> Result<Vec<Repo>, String> {
    let client = reqwest::Client::new();
    let mut all_repos: Vec<Repo> = Vec::new();
    let mut page = 1;

    loop {
        let url = format!(
            "https://api.github.com/users/{username}/repos?per_page=100&page={page}&sort=stars&direction=desc"
        );

        let response = client
            .get(&url)
            .header("User-Agent", "github-stats-rust-cli")
            .send()
            .await
            .map_err(|e| format!("Request failed: {e}"))?;

        if response.status() == reqwest::StatusCode::NOT_FOUND {
            return Err(format!("User '{username}' not found"));
        }

        if !response.status().is_success() {
            return Err(format!(
                "GitHub API error: {} {}",
                response.status(),
                response
                    .text()
                    .await
                    .unwrap_or_else(|_| "unknown error".to_string())
            ));
        }

        let repos: Vec<Repo> = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {e}"))?;

        if repos.is_empty() {
            break;
        }

        all_repos.extend(repos);
        page += 1;
    }

    Ok(all_repos)
}
