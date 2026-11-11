use super::*;

use googletest::prelude::*;
use rstest::*;

macro_rules! err_has {
    ($text:expr) => {
        err(displays_as(contains_substring($text)))
    };
}
mod project {
    use super::*;

    mod add_document {
        use super::*;
        #[rstest]
        fn duplicate_file_not_allowed(mock_f: MockService) {
            let mut p = Project::default();
            assert_ok!(p.add_document("hello.pdf", &mock_f));
            assert_that!(p.add_document("hello.pdf", &mock_f), err_has!("duplicate"));
        }
    }
}

#[fixture]
fn mock_f() -> MockService {
    MockService::default()
}

#[derive(Default)]
struct MockService {}

impl Extractor for MockService {
    fn extract_meta(&self, _path: &Path) -> CoreResult<Metadata> {
        Ok(Default::default())
    }

    fn extract_text(&self, _path: &Path) -> CoreResult<String> {
        Ok(Default::default())
    }
}
