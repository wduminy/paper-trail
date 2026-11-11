use super::*;

use googletest::prelude::*;
use rstest::*;

mod project {
    use super::*;

    mod add_document {
        use super::*;
        #[rstest]
        fn duplicate_file_not_allowed(mock_f: MockService) {
            let mut p = Project::default();
            let mut add = || p.add_document("something.pdf", &mock_f);
            assert_ok!(add());
            assert_that!(add(), err(contains_substring("duplicate")));
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
