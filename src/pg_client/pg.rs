use std::error::Error;

use chrono::{Local, NaiveDateTime, TimeZone};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgConnection, Connection, Postgres};

#[derive(Deserialize, Serialize, Debug, sqlx::FromRow)]
pub struct BoxInfo {
    id: i32,
    file_storage_id: i32,
    pick_up_code: i32,
    login_type: i32,
    create_time: chrono::NaiveDateTime,
    update_time: chrono::NaiveDateTime,
}

impl BoxInfo {
    pub fn new(id: i32, file_storage_id: i32, pick_up_code: i32) -> Self {
        let now =Local::now().timestamp_millis();
        let ndt =NaiveDateTime::from_timestamp_millis(now).unwrap();
        Self {
            id,
            file_storage_id,
            pick_up_code,
            login_type: 1,
            create_time: ndt,
            update_time: ndt,
        }
    }
}
async fn sql_connection() -> Option<PgConnection> {
    let connection = PgConnection::connect("postgres://root:123456@localhost:5432/database").await;
    match connection {
        Ok(conn) => Some(conn),
        Err(err) => {
            println!("err message: {:?}", err);
            None
        }
    }
}

pub async fn insert_box_info(box_info: BoxInfo) -> bool {
    let mut connection = sql_connection()
        .await
        .expect("get database connection eror");
    let sql = sqlx::query(
        "insert into box_info 
    (id,file_storage_id,pick_up_code,login_type,create_time,update_time) 
    values ( $1,$2,$3,$4,$5,$6)",
    )
    .bind(box_info.id)
    .bind(box_info.file_storage_id)
    .bind(box_info.pick_up_code)
    .bind(box_info.login_type)
    .bind(box_info.create_time)
    .bind(box_info.update_time)
    .execute(&mut connection)
    .await;
    sql.is_ok()
}

pub async fn select_box_info(pick_up_code: i32) -> Result<BoxInfo,sqlx::Error> {
    let mut connection = sql_connection()
        .await
        .expect("get database connection eror");
    let sql = sqlx::query_as::<Postgres,BoxInfo>("select * from box_info where pick_up_code =$1")
        .bind(pick_up_code)
        .fetch_one(&mut connection)
        .await;
    sql
}
