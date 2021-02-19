#![allow(missing_docs)]

use super::AppSlot;
use crate::prelude::*;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Clone limit of {0} exceeded for cell: {1:?}")]
    CloneLimitExceeded(u32, AppSlot),

    #[error("Tried to access missing cell nick: '{0}'")]
    CellNickMissing(CellNick),

    #[error("Tried to install app '{0}' which contains duplicate cell nicks. The following cell nicks have duplicates: {1:?}")]
    DuplicateCellNicks(InstalledAppId, Vec<CellNick>),
}
pub type AppResult<T> = Result<T, AppError>;