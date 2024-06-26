pub use crate::errors::FetcherError;
pub(crate) use std::result::Result as StdResult;
#[allow(unused_imports)]
pub(crate) use tracing::{debug, error, info, trace, warn};
pub(crate) use twba_common::prelude::*;

pub type Result<T> = StdResult<T, FetcherError>;
