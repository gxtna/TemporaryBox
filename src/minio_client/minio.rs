use awsregion::Region;
use s3::creds::Credentials;
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

pub async fn get_object() {
    let bucket = create_minio().await;
    let response = bucket.get_object("/one.text").await.unwrap();
    println!("{:?}", std::str::from_utf8(response.bytes()));
}
pub async fn put_object() {
    let bucket = create_minio().await;
    /* let path = "/home/gxt/Documents/简历.pdf";
    let file = std::fs::File::open(path).unwrap();
    let mut bfr = std::io::BufReader::new(file);
    let mut buf = vec![0; 1024];
    let d = AsyncRead::poll_read(self, cx, buf);
    let x =bfr.read(&mut buf);
    bucket.put_object_stream(&mut x, s3_path).await; */
    let response = bucket.put_object("/one.text", "123122".as_bytes()).await;
    match response {
        Ok(response) => println!("{:?}", response.status_code()),
        Err(respnse) => println!("{:?}", respnse),
    }
}

pub async fn delete_object() {
    let bucket = create_minio().await;
    let response = bucket.delete_object("/one.text").await;
    match response {
        Ok(response) => println!("{:?}", response.status_code()),
        Err(respnse) => println!("{:?}", respnse),
    }
}

async fn read_file() {
    /* let path = "/home/gxt/Documents/简历.pdf";
    let file = std::fs::File::open(path).unwrap();
    let mut bfr = std::io::BufReader::new(file);
    let mut buf = vec![0; 1024];
    bfr.read(buf); */
}
