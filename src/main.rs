
mod minio_client;
mod pg_client;
mod utils;
mod actix_client;
#[actix_rt::main]
async fn main(){
    actix_client::routers::client_server().await.unwrap()
}
