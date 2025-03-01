use async_trait::async_trait;
use bytes::Bytes;
#[async_trait]
pub trait Storage {
    async fn send(&self, file_name: String, file_body: Bytes);
}