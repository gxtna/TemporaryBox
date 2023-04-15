
mod minio_client;
mod pg_client;
mod utils;
#[tokio::main]
pub async fn main() {
    //let res =minio_client::minio::put_object("xxx","xxx.pdf").await;

    //minio_client::minio::get_object("个人简历.pdf","xxx.pdf").await;
    let pick_up_code = utils::nanoid::get_nanoid();
    //pg_client::pg::delete_box_info(pick_up_code).await;
    let box_info = pg_client::pg::BoxInfo::new(
        1223345,
        "个人简历.pdf".to_string(),
        "dasdasa.pdf".to_string(),
        "test1".to_string(),
        pick_up_code,
    );
    let ok = pg_client::pg::insert_box_info(box_info).await;
    println!("{}", ok);
    //let res = pg_client::pg::select_box_info(pick_up_code).await.unwrap();
    //println!("{:?}", res);
    //let ok = pg_client::pg::update_box_info(pick_up_code, 50).await;
    //println!("{}", ok);
    
}
