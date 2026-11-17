use super::*;

impl DataService {
    pub fn open_temp_h() -> Self {
        let file_name = tempfile::NamedTempFile::new().unwrap();
        DataService::open(file_name).unwrap()
    }
}
