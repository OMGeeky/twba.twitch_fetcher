mod client;
pub mod errors;
pub mod prelude;
use crate::prelude::*;
use twba_backup_config::prelude::Config;
use twba_backup_config::Conf;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_env_filter(
            "sea_orm=warn,sea_orm_migration=warn,sqlx=warn,twba_twitch_fetcher=trace,twba_local_db=warn,twba_twitch_data=info,twba_downloader_config=info,twba_backup_config=info,other=warn",
        )
        .init();
    info!("Hello, world!");

    run().await?;

    info!("Bye");
    Ok(())
}

async fn run() -> Result<()> {
    let conf = Conf::builder()
        .env()
        .file("./settings.toml")
        .file(shellexpand::tilde("~/twba/config.toml").into_owned())
        .load()
        .map_err(|e| FetcherError::LoadConfig(e.into()))?;

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
