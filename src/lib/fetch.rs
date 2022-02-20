use crate::models::{RepositoriesResponse, Repository};
use anyhow::{anyhow, Error};
use reqwest::header::{ACCEPT, AUTHORIZATION, HOST, USER_AGENT};

struct Fetcher {
    token: String,
}

impl Fetcher {
    pub fn with_token(token: String) -> Self {
        Self { token }
    }
    pub async fn fetch_repositories(
        self,
        language: &str,
        count: usize,
    ) -> Result<Vec<Repository>, Error> {
        let client = reqwest::Client::new();
        let url = format!(
            "https://api.github.com/search/repositories?q=language:{lng}&per_page={count}",
            lng = language,
            count = count
        );
        let res = dbg!(client
            .get(url)
            .bearer_auth(self.token)
            .header(USER_AGENT, "rust-reqwest")
            .header(HOST, "api.github.com:443")
            .header(ACCEPT, "application/vnd.github.v3+json"))
        .send()
        .await?;
        dbg!(&res);
        match res.status() {
            reqwest::StatusCode::OK => match res.json::<RepositoriesResponse>().await {
                Ok(parsed) => Ok(parsed.repos),
                Err(e) => Err(Error::from(e)),
            },
            reqwest::StatusCode::UNAUTHORIZED => Err(anyhow!("Unauthorized")),
            _ => Err(anyhow!("Unexpected error")),
        }
    }
}

#[cfg(test)]
mod fetch_tests {
    use crate::fetch::Fetcher;
    use std::env;

    #[tokio::test]
    async fn fetch_repositories_test() {
        let token = env::var("TOKEN").unwrap();
        let repos = Fetcher::with_token(token)
            .fetch_repositories("rust", 3)
            .await;
        assert!(repos.unwrap().len() > 0);
    }
}
