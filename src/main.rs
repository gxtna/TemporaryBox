
mod minio_client;
mod sqllit_client;

#[tokio::main]
pub async fn  main() {
    minio_client::minio::put_object().await;
    minio_client::minio::get_object().await;
   
    minio_client::minio::delete_object().await;
    minio_client::minio::get_object().await;
}
