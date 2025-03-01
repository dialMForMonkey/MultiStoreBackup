use async_trait::async_trait;
use aws_sdk_s3::primitives::{ByteStream, SdkBody};
use tokio::runtime::Runtime;
use bytes::Bytes;



use crate::config::Config;
use crate::port::Storage;

struct S3 {
    client: aws_sdk_s3::Client,
    config: Config
}


impl S3 {
     fn new(self, config: Config) -> S3 {

        let config_aws = Runtime::new().unwrap().block_on(async {
            let config = aws_config::load_from_env().await;
            config
        });
        let client = aws_sdk_s3::Client::new(&config_aws);


        S3 {
            client,
            config
        }
    }
}




#[async_trait]
impl Storage for S3 {
    async fn send(&self, file_name: String, file_body: Bytes) {
        let aws_config = self.config.get_aws_config();
        match aws_config {
            Some(aws)=>{
                let stream = ByteStream::new(SdkBody::from(file_body));
                let send_result = self.client.put_object().bucket(aws.bucket.as_str()).key(file_name).body(stream).send().await;
               // send_result.is_ok()


            },
            None=>{
                print!("not load config aws")
            }
        }
    }
}
