use awscreds::Credentials;
use awsregion::Region;
use s3::error::S3Error;
use s3::Bucket;

// TgNBUh5gZ0kj5KeT
// ECU9cTmjwtgJWTf6GXHqL0JdtH8cA0Tb
#[tokio::main]
async fn main() {
    let credentials = s3::creds::Credentials::new(
        Some("TgNBUh5gZ0kj5KeT"),
        Some("ECU9cTmjwtgJWTf6GXHqL0JdtH8cA0Tb"),
        None,
        None,
        None,
    ).unwrap();
    let name = "test1";
    let bucket = Bucket::new(
        name,
        Region::Custom {
            region: "eu-central-1".to_owned(),
            endpoint: "http://localhost:9000".to_owned(),
        },
        credentials,
    ).unwrap().with_path_style();
    //let response = bucket.list("".into(), None).await;
    let response = bucket.get_object("Screenshot-6.png").await.unwrap();
    println!("{:?}", "11111111111");
    println!("{:?}", response.status_code());
}
