pub type CoreResult<T = ()> = std::result::Result<T, CoreError>;

#[derive(Debug)]
pub struct CoreError(String);

impl CoreError {
    pub(crate) fn new(m: impl Into<String>) -> CoreResult {
        CoreResult::Err(Self::make(m))
    }

    pub(crate) fn make(m: impl Into<String>) -> CoreError {
        CoreError(m.into())
    }
}

impl From<std::io::Error> for CoreError {
    fn from(value: std::io::Error) -> Self {
        CoreError(value.to_string())
    }
}

impl From<serde_json::Error> for CoreError {
    fn from(value: serde_json::Error) -> Self {
        CoreError(value.to_string())
    }
}

impl AsRef<str> for CoreError {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
