
use std::collections::HashMap;


use crate::minio_client::minio_server;
use crate::pg_client::pg::{self, BoxInfo};
use crate::utils::zip_file;
use crate::utils::{config::APPCONFIG, nanoid};
use actix_cors::Cors;

use actix_multipart::Multipart;
use actix_web::http::header::{ContentDisposition, DispositionParam, DispositionType};
use actix_web::{get, http, post, web, App, HttpResponse, HttpServer, Responder};

use chrono::Local;

use futures_util::StreamExt as _;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Param {
    pick_up_code: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct StorageTimeParam {
    pick_up_code: String,
    storage_time: i32,
}

pub async fn client_server() -> std::io::Result<()> {
    let config = &APPCONFIG.web;
    let config_temp = config.clone();
    HttpServer::new(move || {
        // 跨域配置的时候 http 和https 要单独配置才行
        let cors = Cors::default()
            .allowed_origin(&config.clone().cros)
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);
        App::new()
            .wrap(cors)
            .service(download_file)
            .service(upload_file)
            .service(extend_storage_time)
    })
    .bind(config_temp.clone().address)?
    .run()
    .await
}

#[get("/download_file")]
async fn download_file(param: web::Query<Param>) -> impl Responder {
    let box_info = pg::select_box_info(param.pick_up_code.to_string())
        .await
        .unwrap();
    let data = minio_server::get_object(&box_info.file_remote_name()).await;
    let cd = ContentDisposition {
        disposition: DispositionType::Attachment,
        parameters: vec![DispositionParam::Filename(box_info.file_name.to_string())],
    };
    let mut builder = HttpResponse::Ok();
    builder.content_type("application/octet-stream; charset=UTF-8");
    // 设置 代表前端可以在Content-Disposition获取数据
    builder.append_header(("Access-Control-Expose-Headers", "Content-Disposition"));
    builder.insert_header((actix_web::http::header::CONTENT_DISPOSITION, cd));
    builder.body(data)
}

#[post("/upload_file")]
async fn upload_file(mut payload: Multipart) -> impl Responder {
    let mut map = HashMap::new();
    while let Some(item) = payload.next().await {
        let mut field = item.unwrap();
        while let Some(chunk) = field.next().await {
            let file_name = field.content_disposition().get_filename().unwrap();
            let chunk = chunk.unwrap().to_vec();
            map.insert(file_name.clone().to_string(), chunk);
        }
    }
    let time = &Local::now().timestamp_millis().to_string();
    let pick_up_code = nanoid::nano_id();
    let pick_up_code_copy = pick_up_code.clone();
    let zip = zip_file::zip_file(map).unwrap();
    let mut remote_name = format!("{}-{}.{}", time, pick_up_code, "zip");
    let mut remote_name_copy = remote_name.clone();
    /* if minio_server::put_object(zip, &remote_name).await.unwrap() == 200 {
        match pg::insert_box_info(BoxInfo::new(
            file_name,
            remote_name,
            "test1".to_string(),
            pick_up_code,
        ))
        .await
        {
            true => HttpResponse::Ok().body(pick_up_code_copy),
            false => {
                minio_server::delete_object(&mut remote_name_copy)
                    .await
                    .unwrap();
                return HttpResponse::Ok().body("上传错误");
            }
        };
    } else {
        HttpResponse::Ok().body("上传错误")
    } */
    let res = minio_server::put_object(zip, &mut remote_name)
        .await
        .unwrap();
    if res == 200 {
        let insert_bool = pg::insert_box_info(BoxInfo::new(
            remote_name.clone(),
            remote_name,
            "test1".to_string(),
            pick_up_code,
        ))
        .await;
        if insert_bool {
            HttpResponse::Ok().body(pick_up_code_copy)
        } else {
            minio_server::delete_object(&mut remote_name_copy)
                .await
                .unwrap();
            HttpResponse::Ok().body("上传错误")
        }
    } else {
        HttpResponse::Ok().body("上传错误")
    }
    //
}

/* #[post("/upload_file")]
async fn upload_file(files: Vec<MultipartForm<Upload>>) -> impl Responder {
    println!("{:?}", files.len());
    /* let mut items: Vec<&str> = files.files.file_name.as_ref().unwrap().split(".").collect();
    if items.len()==1 {
        // 默认如果没有后缀的文件自动修改为txt格式
        items.push("txt");
    }
    let content = std::fs::read(files.files.file.path()).unwrap();
    let mut file_name = String::new();
    let name = items[0];
    let suffix = items[1];
    file_name.push_str(name);
    file_name.push_str(".");
    file_name.push_str(suffix);
    let pick_up_code = nanoid::nano_id();
    let mut remote_name = String::new();
    remote_name.push_str(name);
    remote_name.push_str("-");
    remote_name.push_str(&Local::now().timestamp_millis().to_string());
    remote_name.push_str("-");
    remote_name.push_str(&pick_up_code);
    remote_name.push_str(".");
    remote_name.push_str(suffix);
    let pick_up_code_copy = pick_up_code.clone();
    let mut remote_name_copy = remote_name.clone();
    let res = minio_server::put_object(content, &mut remote_name)
        .await
        .unwrap();
    if res == 200 {
        let insert_bool = pg::insert_box_info(BoxInfo::new(
            file_name,
            remote_name,
            "test1".to_string(),
            pick_up_code,
        ))
        .await;
        if insert_bool {
            HttpResponse::Ok().body(pick_up_code_copy)
        } else {
            minio_server::delete_object(&mut remote_name_copy)
                .await
                .unwrap();
            HttpResponse::Ok().body("上传错误")
        }
    } else {
        HttpResponse::Ok().body("上传错误")
    } */

} */

#[post("/extend_storage_time")]
async fn extend_storage_time(param: web::Json<StorageTimeParam>) -> impl Responder {
    let is_ok = pg::update_box_info(param.pick_up_code.to_string(), param.storage_time).await;
    HttpResponse::Ok().body(is_ok.to_string())
}
