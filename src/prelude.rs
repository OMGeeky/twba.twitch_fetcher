pub use crate::errors::FetcherError;
pub(crate) use std::result::Result as StdResult;
pub(crate) use tracing::{debug, error, info, trace, warn};

pub type Result<T> = StdResult<T, FetcherError>;
