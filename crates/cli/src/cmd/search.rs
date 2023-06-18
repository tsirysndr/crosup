use crate::types::SearchArgs;
use anyhow::{anyhow, Error};
use crosup_nix::search::{
    client::NixPackagesClient, esclient::ElasticSearchClient, matchers::MatchName, query::Query,
};
use owo_colors::OwoColorize;
use spinners::{Spinner, Spinners};

pub async fn execute_search(args: SearchArgs) -> Result<(), Error> {
    let client = ElasticSearchClient::new()
        .map_err(|e| anyhow!("Failed to create ElasticSearch client: {}", e))?;
    let query = Query {
        channel: args.channel,
        name: Some(MatchName { name: args.package }),
        max_results: args.max_results,
        ..Default::default()
    };

    let mut sp = Spinner::new(Spinners::Dots9, "Searching...".into());
    let results = client.search(query).await?;
    sp.stop();

    println!("");

    for result in results {
        match result.package.description {
            Some(ref description) => {
                println!(
                    "{} @ {} : {}",
                    result.package.name.cyan().underline(),
                    result.package.version.bright_green(),
                    description
                );
            }
            None => {
                println!(
                    "{} @ {}",
                    result.package.name.cyan().underline(),
                    result.package.version.bright_green()
                );
            }
        };
    }
    Ok(())
}
