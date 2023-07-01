use super::minio_client::create_minio;
use anyhow::Result;
use s3::error::S3Error;

pub async fn get_object(object_name: &str) -> Vec<u8> {
    let bucket = create_minio().await;
    let response = bucket.get_object(object_name).await.unwrap();
    response.bytes().to_vec()
}
pub async fn put_object(file: Vec<u8>, remote_path: &str) -> Result<u16, S3Error> {
    let bucket = create_minio().await;
    let response = bucket.put_object(remote_path, &file).await;
    match response {
        Ok(response) => Ok(response.status_code()),
        Err(response) => Err(response),
    }
}

pub async fn delete_object(object_name: &str) -> Result<(), S3Error> {
    let bucket = create_minio().await;
    let response = bucket.delete_object(object_name).await;
    match response {
        Ok(_response) => Ok(()),
        Err(response) => Err(response),
    }
}

#[allow(dead_code)]
pub async fn list_objects() -> Vec<String> {
    let bucket = create_minio().await;
    let response = bucket
        .list("".to_string(), Some("".to_string()))
        .await
        .unwrap();
    let mut names = Vec::new();
    for res in response {
        for con in res.contents {
            names.push(con.key)
        }
    }
    names
}
