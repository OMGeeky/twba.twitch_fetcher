use std::error::Error as StdError;
use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum FetcherError {
    #[error("Could not load config")]
    LoadConfig(#[source] anyhow::Error),
    #[error("Some error with the database: {0:?}")]
    OpenDatabase(#[from] twba_local_db::re_exports::sea_orm::DbErr),
    #[error("File or Folder not found or invalid: {0:?}")]
    NotFound(PathBuf),
    #[error("Error creating client: {0:?}")]
    CreateClientError(#[source] Box<dyn StdError>),
    #[error("Could not get videos: {0:?}")]
    GetVideosError(#[source] Box<dyn StdError>),
}
