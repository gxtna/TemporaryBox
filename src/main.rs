use chrono::Local;


mod minio_client;
mod pg_client;
#[tokio::main]
pub async fn main() {
    //let res =minio_client::minio::put_object("xxx","xxx.pdf").await;

    //minio_client::minio::get_object("个人简历.pdf","xxx.pdf").await;
    let box_info = pg_client::pg::BoxInfo::new(1223345, 999999, 123456);
    let ok = pg_client::pg::insert_box_info(box_info).await;
    println!("{}",ok);
    let res = pg_client::pg::select_box_info(123456).await.unwrap();
    println!("{:?}",res);
}
