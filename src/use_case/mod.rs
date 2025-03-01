
mod download;
mod upload;
mod entities;

pub trait upload_use_case {
    async fn execute(&self, files_path: Vec<String>);
}

pub trait download_use_case {
    async fn execute(&self);
}

