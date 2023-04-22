use crate::utils::config;
use crate::utils::time;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgConnection, Connection, Postgres};
use log::error;


#[derive(Deserialize, Serialize, Debug, sqlx::FromRow)]
pub struct BoxInfo {
    file_name: String,
    file_remote_name: String,
    bucket_name: String,
    storage_time: i32,
    pick_up_code: String,
    login_type: i32,
    file_remote_path: String,
    system_type: i32,
    create_time: NaiveDateTime,
    update_time: NaiveDateTime,
}

impl BoxInfo {
    pub fn new(
        file_name: String,
        file_remote_name: String,
        bucket_name: String,
        pick_up_code: String,
    ) -> Self {
        let date_time = time::get_local_time();
        Self {
            file_name,
            file_remote_name,
            bucket_name,
            storage_time: 1 * 24, // TDDO  默认的保存时间是1天 ,按照小时存储的
            pick_up_code,
            login_type: 1,                     // TDDO 默认就是1 pc登录
            file_remote_path: "/".to_string(), // TODO 默认就存放在根目录下
            system_type: 1,                    // TODO 默认为1 文件系统是minio
            create_time: date_time,
            update_time: date_time,
        }
    }
    pub fn file_name(&self) -> String {
        self.file_name.to_string()
    }
    pub fn file_remote_name(&self) -> String {
        self.file_remote_name.to_string()
    }
    pub fn storage_time(&self) -> i32 {
        self.storage_time
    }
    pub fn update_time(&self) -> NaiveDateTime {
        self.update_time
    }
}
async fn sql_connection() -> PgConnection {
    let conf = config::read_conf()
        .map_err(|err| {
            error!("{}", err);
        })
        .unwrap()
        .postgres();
    let url = conf.clone().url();
    let connection = PgConnection::connect(&url)
        .await
        .expect("get database connection eror");
    connection
}

pub async fn insert_box_info(box_info: BoxInfo) -> bool {
    let mut connection = sql_connection().await;
    let sql = sqlx::query(
        "insert into box_info 
    (file_name,file_remote_name,bucket_name,storage_time,pick_up_code,login_type,file_remote_path,system_type,create_time,update_time) 
    values ( $1,$2,$3,$4,$5,$6,$7,$8,$9,$10)",
    )
    .bind(box_info.file_name)
    .bind(box_info.file_remote_name)
    .bind(box_info.bucket_name)
    .bind(box_info.storage_time)
    .bind(box_info.pick_up_code)
    .bind(box_info.login_type)
    .bind(box_info.file_remote_path)
    .bind(box_info.system_type)
    .bind(box_info.create_time)
    .bind(box_info.update_time)
    .execute(&mut connection)
    .await;
    sql.is_ok()
}

pub async fn select_box_info(pick_up_code: String) -> Result<BoxInfo, sqlx::Error> {
    let mut connection = sql_connection().await;
    let sql = sqlx::query_as::<Postgres, BoxInfo>("select * from box_info where pick_up_code =$1")
        .bind(pick_up_code)
        .fetch_one(&mut connection)
        .await;
    sql
}

pub async fn select_box_info_all() -> Vec<BoxInfo> {
    let mut connection = sql_connection().await;
    let sql = sqlx::query_as::<Postgres, BoxInfo>("select * from box_info")
        .fetch_all(&mut connection)
        .await
        .unwrap();
    sql
}

pub async fn update_box_info(pick_up_code: String, storage_time: i32) -> bool {
    let date_time = time::get_local_time();
    let mut conn = sql_connection().await;
    let sql =
        sqlx::query("update box_info set storage_time=$1,update_time=$2 where pick_up_code=$3")
            .bind(storage_time * 24)
            .bind(date_time)
            .bind(pick_up_code)
            .execute(&mut conn)
            .await;
    sql.is_ok()
}

pub async fn delete_box_info(file_remote_name: String) {
    let mut conn = sql_connection().await;
    sqlx::query("delete from box_info where file_remote_name=$1")
        .bind(file_remote_name)
        .execute(&mut conn)
        .await
        .expect("delete from box_info error")
        .rows_affected();
}
