use bytes::Bytes;
use mockall::Any;
use crate::use_case::upload_use_case;
use crate::use_case::entities::{encrypt_symmetric_algorithm, CryptographySymmetrical};
use tokio::fs;
use tokio::io::{AsyncReadExt, AsyncSeekExt};

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
        for file_path in files_path {
            let mut parts = 0;
            let mut file = fs::File::open(file_path.clone()).await.unwrap();
            let a = file.metadata().await.unwrap();
            a.len();
             loop {
                 let mut buf = vec![1; 50 * 1024 * 1024]; //52428800
                 file.set_max_buf_size(50 * 1024 * 1024);
                 let len = file.read(&mut buf).await.unwrap();

                    if len > 0 {
                        let encrypted = encrypt_symmetric_algorithm::ChaCha20::encrypt(buf).unwrap();

                        self.storage.send(format!("{}_{}", parts, file_path), Bytes::from(encrypted.encrypted_data)).await;

                        parts += 1;
                        continue
                    }
                 break
             }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::future::Future;
    use std::pin::Pin;
    use bytes::Bytes;
    use tokio::fs;
    use crate::port::{MockStorage, Storage};
    use crate::use_case::upload::upload;
    use crate::use_case::upload_use_case;

    #[tokio::test]
    async fn upload_complety_test() {
        let mb: usize = 1024 * 1024;
        let total_bytes = 200 * mb;

        let _ = fs::write("/home/gabriel/workspace/MultiStoreBackup/asserts/file_test", vec![0u8; total_bytes]).await;
        let mut mock = MockStorage::new();
        mock.expect_send().times(4).returning(move |_,_|{
            ()
        });

        let mock_storage = Box::from(mock);
        let  upload_file = upload::new(mock_storage);
        upload_file.execute(vec!["/home/gabriel/workspace/MultiStoreBackup/asserts/file_test".to_string()]).await;
    }


}