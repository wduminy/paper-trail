use std::path::Path;

use pdf_extract::{extract_text_from_mem, Document};

use crate::{project::*, result::*};

#[derive(Default)]
pub(crate) struct PdfService;

impl Extractor for PdfService {
    fn extract_meta(&self, path: &Path) -> CoreResult<Metadata> {
        let data = std::fs::read(path)?;
        let meta = Document::load_metadata_mem(&data)?;
        Ok(Metadata {
            title: meta.title,
            author: meta.author,
        })
    }

    fn extract_text(&self, path: &Path) -> CoreResult<String> {
        let data = std::fs::read(path)?;
        extract_text_from_mem(&data).map_err(|e| CoreError::make(e.to_string()))
    }
}

impl From<pdf_extract::Error> for CoreError {
    fn from(value: pdf_extract::Error) -> Self {
        CoreError::make(value.to_string())
    }
}
