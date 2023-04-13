use awsregion::Region;
use s3::creds::Credentials;
use s3::error::S3Error;
use s3::Bucket;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
pub async fn create_minio() -> Bucket {
    let credentials = Credentials::new(
        Some("TgNBUh5gZ0kj5KeT"),
        Some("ECU9cTmjwtgJWTf6GXHqL0JdtH8cA0Tb"),
        None,
        None,
        None,
    )
    .unwrap();
    let name = "test1";
    let bucket = Bucket::new(
        name,
        Region::Custom {
            region: "eu-central-1".to_owned(),
            endpoint: "http://localhost:9000".to_owned(),
        },
        credentials,
    )
    .unwrap()
    .with_path_style();
    bucket
}

pub async fn get_object(object_name: &str, local_path: &str) {
    let bucket = create_minio().await;
    let response = bucket.get_object(object_name).await.unwrap();
    let mut file = File::create(local_path).unwrap();
    file.write_all(response.bytes()).unwrap();
}
pub async fn put_object(local_path: &str, remote_path: &str) -> Result<u16, S3Error> {
    let bucket = create_minio().await;
    let content = fs::read(local_path).unwrap();
    let response = bucket.put_object(remote_path, &content).await;
    match response {
        Ok(response) => Ok(response.status_code()),
        Err(respnse) => Err(respnse),
    }
}

pub async fn delete_object(object_name: &str) -> Result<u16, S3Error>{
    let bucket = create_minio().await;
    let response = bucket.delete_object(object_name).await;
    match response {
        Ok(response) => Ok(response.status_code()),
        Err(respnse) => Err(respnse),
    }
}
