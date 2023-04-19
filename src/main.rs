mod actix_client;
mod minio_client;
mod pg_client;
mod utils;
#[actix_rt::main]
async fn main() {
    //utils::task::task_build().await;
    //actix_client::routers::client_server().await.unwrap();
    let conf = utils::config::read_conf().minio().unwrap().access_key().unwrap();
    println!("{}", conf)
}
