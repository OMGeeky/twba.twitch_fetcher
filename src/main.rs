mod client;
pub mod errors;
pub mod prelude;
use crate::prelude::*;
use twba_backup_config::prelude::Config;
use twba_backup_config::Conf;
use twba_common::{get_config, init_tracing};

#[tokio::main]
async fn main() -> Result<()> {
    let _guard = init_tracing("twba_twitch_fetcher");
    info!("Hello, world!");

    run().await?;

    info!("Bye");
    Ok(())
}

async fn run() -> Result<()> {
    let conf = get_config();

    trace!("Opening database");
    let db = twba_local_db::open_database(Some(&conf.db_url)).await?;
    twba_local_db::migrate_db(&db).await?;
    trace!("Creating twitch client");
    let twitch_client = twba_twitch_data::get_client()
        .await
        .map_err(FetcherError::CreateClientError)?;

    info!("Creating client");
    let client = client::FetcherClient::new(conf, db, twitch_client);
    info!("Fetching new videos");
    client.fetch_new_videos().await?;
    info!("Done fetching new videos");
    Ok(())
}
