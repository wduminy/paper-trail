#[cfg(test)]
mod tests;

use std::path::Path;

use crate::result::{CoreError, CoreResult};

#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct Project {
    docs: Vec<Document>,
    next_id: i32,
}

impl Project {
    pub(crate) fn add_document(
        &mut self,
        path: impl AsRef<Path>,
        ex: &impl Extractor,
    ) -> CoreResult {
        let p_name = path.as_ref().as_os_str().display().to_string();
        if self.docs.iter().find(|e| e.source_path == p_name).is_some() {
            CoreError::new(format!(
                "Cannot insert a duplicate.  The path '{}' is already in the project",
                p_name
            ))
        } else {
            let doc = Document::new(self.next_id, path.as_ref(), ex)?;
            self.docs.push(doc);
            self.next_id += 1;
            Ok(())
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
struct Document {
    id: i32,
    source_path: String,
    metadata: Metadata,
    content: String,
}

impl Document {
    fn new(id: i32, path: &Path, ex: &impl Extractor) -> CoreResult<Self> {
        let metadata = ex.extract_meta(path)?;
        let content = ex.extract_text(path)?;
        Ok(Self {
            id,
            source_path: path.as_os_str().display().to_string(),
            metadata,
            content,
        })
    }
}

#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct Metadata {
    pub title: Option<String>,
    pub author: Option<String>,
}

pub(crate) trait Extractor {
    fn extract_meta(&self, path: &Path) -> CoreResult<Metadata>;
    fn extract_text(&self, path: &Path) -> CoreResult<String>;
}
