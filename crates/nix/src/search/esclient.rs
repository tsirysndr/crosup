use std::time::Duration;

use surf::{http::auth::BasicAuth, Client, Config, Url};

use super::{client::NixPackagesClient, query::Query, types::Hit};

// Taken from the upstream repository
// https://github.com/NixOS/nixos-search/blob/main/frontend/src/index.js
pub const ELASTICSEARCH_USERNAME: &str = "aWVSALXpZv";
pub const ELASTICSEARCH_PASSWORD: &str = "X8gPHnzL52wFEekuxsfQ9cSh";
pub const ELASTICSEARCH_BASE_URL: &str =
    "https://nixos-search-7-1733963800.us-east-1.bonsaisearch.net:443/";
pub const ELASTICSEARCH_INDEX_PREFIX: &str = "latest-*-";

pub struct ElasticSearchClient {
    pub client: Client,
}

impl ElasticSearchClient {
    pub fn new() -> Result<Self, surf::Error> {
        let auth = BasicAuth::new(ELASTICSEARCH_USERNAME, ELASTICSEARCH_PASSWORD);
        let client = Config::new()
            .set_base_url(Url::parse(ELASTICSEARCH_BASE_URL).unwrap())
            .set_timeout(Some(Duration::from_secs(5)))
            .add_header("Content-Type", "application/json")?
            .add_header("Accept", "application/json")?
            .add_header(auth.name(), auth.value())?
            .try_into()?;
        Ok(Self { client })
    }
}

#[async_trait::async_trait]
impl NixPackagesClient for ElasticSearchClient {
    async fn search(&self, query: Query) -> Result<Vec<Hit>, anyhow::Error> {
        let body = query.payload()?;
        let response = self
            .client
            .post(format!(
                "{}nixos-{}/_search",
                ELASTICSEARCH_INDEX_PREFIX, query.channel
            ))
            .body(body)
            .recv_json::<super::types::Response>()
            .await
            .map_err(|e| anyhow::anyhow!(e))?;
        Ok(response.hits.hits)
    }
}

#[cfg(test)]
mod tests {
    use crate::search::matchers::MatchName;

    use super::*;

    #[tokio::test]
    async fn test_search() {
        let client = ElasticSearchClient::new().unwrap();
        let query = Query {
            channel: "unstable".to_string(),
            name: Some(MatchName { name: "vim".into() }),
            max_results: 10,
            ..Default::default()
        };
        let packages = client.search(query).await.unwrap();
        assert!(packages.len() > 0);
    }
}
