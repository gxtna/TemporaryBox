use aws_config::SdkConfig;
use aws_credential_types::{provider::SharedCredentialsProvider, Credentials};
use aws_sdk_s3::{operation::restore_object, Client};
use aws_smithy_http::byte_stream::ByteStream;
use aws_types::region::Region;
use std::path::Path;
async fn client() -> Client {
    let credentials = Credentials::new(
        "TgNBUh5gZ0kj5KeT",
        "ECU9cTmjwtgJWTf6GXHqL0JdtH8cA0Tb",
        None,
        None,
        "example",
    );
    let sdk_config = SdkConfig::builder()
        .region(Region::new("eu-central-1"))
        .credentials_provider(SharedCredentialsProvider::new(credentials))
        .build();
    Client::new(&sdk_config)
}

pub async fn upload_object(file: &Path) {
    println!("{:?}", file);
    let stream = ByteStream::from_path(file).await.unwrap();
    let client = client().await;
    let key = "test_one".to_string();
    let bucket = "test1";
    println!("{}", key);
    let upload_id = get_chunk_upload_id().await;
    println!("{:#?}", upload_id);
    let upload = client
        .upload_part()
        .key(&key)
        .bucket(bucket)
        .upload_id(upload_id)
        .body(stream)
        .part_number(1)
        .send()
        .await
        .unwrap();
    println!("{:#?}", upload)
}

pub async fn get_chunk_upload_id() -> String {
    let client = client().await;
    let key = "test_one".to_string();
    let bucket = "test1";
    let multipart_upload_res = client
        .create_multipart_upload()
        .bucket(bucket)
        .key(&key)
        .send()
        .await
        .unwrap();
    println!("{:?}", multipart_upload_res);
    let upload_id = multipart_upload_res.upload_id().unwrap();
    upload_id.to_string()
}
