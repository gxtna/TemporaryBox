mod actix_client;
mod minio_client;
mod pg_client;
mod utils;
use log4rs;

#[actix_rt::main]
async fn main() {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    utils::task::task_build().await;
    actix_client::routers::client_server().await.unwrap();
}


