use std::path::Path;

use crate::{pdf::*, project::*, result::*};

#[derive(Default)]
pub struct ProjectService {
    project: Project,
    pdf_service: PdfService,
}

impl ProjectService {
    pub fn add_pdf(&mut self, path: impl AsRef<Path>) -> CoreResult {
        self.project.add_document(path, &self.pdf_service)
    }

    pub fn save(&self, path: impl AsRef<Path>) -> CoreResult {
        let text = serde_json::to_string_pretty(&self.project)?;
        std::fs::write(path, text)?;
        Ok(())
    }
}
