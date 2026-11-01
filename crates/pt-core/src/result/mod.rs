#[derive(Debug, thiserror::Error)]
pub enum CoreError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error("{0}")]
    Normal(String),
}

impl CoreError {
    pub(crate) fn new(m: impl Into<String>) -> CoreResult {
        CoreResult::Err(Self::make(m))
    }

    pub(crate) fn make(m: impl Into<String>) -> CoreError {
        CoreError::Normal(m.into())
    }
}

pub type CoreResult<T = ()> = std::result::Result<T, CoreError>;
