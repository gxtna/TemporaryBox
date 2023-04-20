mod actix_client;
mod minio_client;
mod pg_client;
mod utils;


#[actix_rt::main]
async fn main() {
    utils::task::task_build().await;
    actix_client::routers::client_server().await.unwrap();

    
}
