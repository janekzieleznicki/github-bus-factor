use crate::models::{Contributor, RepositoriesResponse, Repository};
use anyhow::{anyhow, Error};
use num::integer::gcd;
use reqwest::header::{ACCEPT, HOST, USER_AGENT};
use std::env;
use tokio::sync::mpsc::Sender;

pub struct Fetcher {
    token: String,
    client: reqwest::Client,
}

impl Fetcher {
    pub fn with_token(token: String) -> Self {
        Self {
            token,
            client: reqwest::Client::new(),
        }
    }
    pub fn with_env_token() -> Self {
        let token = env::var("TOKEN").unwrap();
        Self::with_token(token)
    }
    pub async fn fetch_repositories_with_contributors(
        self,
        language: &str,
        count: usize,
        tx: Sender<Repository>,
    ) -> Result<(), Error> {
        let reqest_size = gcd(count, 100);
        for (idx, _) in (0..(count / reqest_size)).into_iter().enumerate() {
            let repos = self
                .fetch_repositories(language, reqest_size, idx + 1)
                .await?;
            for mut repo in repos {
                self.fetch_contributors(&mut repo).await?;
                tx.send(repo).await?;
            }
        }
        Ok(())
    }
    pub async fn fetch_repositories(
        &self,
        language: &str,
        count: usize,
        page: usize,
    ) -> Result<Vec<Repository>, Error> {
        let url = format!(
            "https://api.github.com/search/repositories?q=language:{lng}&per_page={count}&page={page}",
            lng = language,
            count = count,
            page = page
        );
        let res = self
            .client
            .get(url)
            .bearer_auth(&self.token)
            .header(USER_AGENT, "rust-reqwest")
            .header(HOST, "api.github.com:443")
            .header(ACCEPT, "application/vnd.github.v3+json")
            .send()
            .await?;
        match res.status() {
            reqwest::StatusCode::OK => match res.json::<RepositoriesResponse>().await {
                Ok(parsed) => Ok(parsed.repos),
                Err(e) => Err(Error::from(e)),
            },
            reqwest::StatusCode::UNAUTHORIZED => Err(anyhow!("Unauthorized")),
            _ => Err(anyhow!("Unexpected error")),
        }
    }
    async fn fetch_contributors(&self, repo: &mut Repository) -> Result<(), Error> {
        let url = format!(
            "{}?q=anon:true&sort=stars&per_page={}",
            repo.contributors_url, 25
        );
        let res = self
            .client
            .get(url)
            .bearer_auth(&self.token)
            .header(USER_AGENT, "rust-reqwest")
            .header(HOST, "api.github.com:443")
            .header(ACCEPT, "application/vnd.github.v3+json")
            .send()
            .await?;
        match res.status() {
            reqwest::StatusCode::OK => {
                return match res.json::<Vec<Contributor>>().await {
                    Ok(parsed) => {
                        repo.contributors = parsed;
                        Ok(())
                    }
                    Err(e) => Err(Error::from(e)),
                }
            }
            reqwest::StatusCode::UNAUTHORIZED => Err(anyhow!("Unauthorized")),
            _ => Err(anyhow!("Unexpected error")),
        }
    }
}

#[cfg(test)]
mod fetch_tests {
    use crate::fetch::Fetcher;

    #[tokio::test]
    async fn fetch_repositories_test() {
        {
            let repos = Fetcher::with_env_token()
                .fetch_repositories("rust", 3, 1)
                .await;
            assert_eq!(repos.unwrap().len(), 3);
        }
        {
            let repos = Fetcher::with_env_token()
                .fetch_repositories("rust", 2, 100)
                .await;
            assert_eq!(repos.unwrap().len(), 2);
        }
    }
    #[tokio::test]
    async fn fetch_repositories_with_contributors_test() {
        let (tx, mut rx) = tokio::sync::mpsc::channel(32);
        tokio::spawn(async move {
            match Fetcher::with_env_token()
                .fetch_repositories_with_contributors("rust", 3, tx)
                .await
            {
                Err(e) => eprintln!("{:?}", e),
                _ => {}
            }
        });
        while let Some(repo) = rx.recv().await {
            dbg!(&repo.contributors);
            assert!(repo.contributors.len() > 0);
        }
    }
}
