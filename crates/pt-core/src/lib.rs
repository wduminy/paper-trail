use std::path::Path;

#[derive(Debug, thiserror::Error)]
pub enum PdfError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Extract(#[from] pdf_extract::OutputError),
}

pub type Result<T> = std::result::Result<T, PdfError>;

/// Extract the plain text from the PDF at `path`.
pub fn extract(path: impl AsRef<Path>) -> Result<String> {
    let data = std::fs::read(path)?;
    Ok(pdf_extract::extract_text_from_mem(&data)?)
}
