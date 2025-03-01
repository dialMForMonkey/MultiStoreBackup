use crate::use_case::upload_use_case;

struct upload {
    storage: Box<dyn crate::port::Storage>
}

impl upload {
    fn new(storage: Box<dyn crate::port::Storage>) -> Self {
        upload{ storage }
    }
}

impl upload_use_case for upload {
    async fn execute(&self, files_path: Vec<String>) {
        todo!()
    }
}
