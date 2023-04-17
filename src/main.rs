
mod minio_client;
mod pg_client;
mod utils;
mod actix_client;
#[actix_rt::main]
async fn main(){
    
    //utils::task::task_build().await;
    //actix_client::routers::client_server().await.unwrap()
    minio_client::minio::list_objects().await;
}
