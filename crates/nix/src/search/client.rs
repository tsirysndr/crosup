use anyhow::Error;

use super::{query::Query, types::Hit};

#[async_trait::async_trait]
pub trait NixPackagesClient {
    async fn search(&self, query: Query) -> Result<Vec<Hit>, Error>;
}
