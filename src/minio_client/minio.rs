use awsregion::Region;
use s3::creds::Credentials;
use s3::error::S3Error;
use s3::Bucket;
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

pub async fn get_object(object_name: &str) -> Vec<u8>{
    let bucket = create_minio().await;
    let response = bucket.get_object(object_name).await.unwrap();
    response.bytes().to_vec()
}
pub async fn put_object(file:Vec<u8>, remote_path: &str) -> Result<u16, S3Error> {
    let bucket = create_minio().await;
    let response = bucket.put_object(remote_path, &file).await;
    match response {
        Ok(response) => Ok(response.status_code()),
        Err(respnse) => Err(respnse),
    }
}

pub async fn delete_object(object_name: &str) -> Result<(), S3Error>{
    let bucket = create_minio().await;
    let response = bucket.delete_object(object_name).await;
    match response {
        Ok(_response) => Ok(()),
        Err(respnse) => Err(respnse),
    }
}

#[warn(dead_code)]
pub async fn list_objects() -> Vec<String>{
    let bucket = create_minio().await;
    let response = bucket.list("".to_string(), Some("".to_string())).await.unwrap();
    let mut names = Vec::new();
    for res in response {
        for con in res.contents {
            names.push(con.key)
        }
    }
    names
}