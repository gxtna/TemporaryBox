
mod minio_client;
mod sqllit_client;
#[tokio::main]
pub async fn main() {
    let res =minio_client::minio::put_object("xxx","xxx.pdf").await;

    minio_client::minio::get_object("个人简历.pdf","xxx.pdf").await;



}
