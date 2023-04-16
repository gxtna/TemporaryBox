use std::sync::Arc;

use actix_cors::Cors;
use actix_web::web::Data;
use actix_web::{get, http, post, App, HttpResponse, HttpServer, Responder, HttpResponseBuilder,HttpRequest, web, ResponseError};
use actix_web::http::header::{
    ContentDisposition, DispositionParam, DispositionType
};
use serde::{Serialize,Deserialize};
use crate::minio_client;
use crate::pg_client::pg;

#[derive(Serialize,Deserialize)]
struct Param {
    pick_up_code: String,
}
pub async fn client_server() -> std::io::Result<()> {
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:5173")
            .allowed_origin_fn(|origin, _req_head| origin.as_bytes().ends_with(b".rust-lang.org"))
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);
        App::new()
            .wrap(cors)
            .service(download_file)
            .service(upload_file)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

#[get("/downloadFile")]
async fn download_file(param:web::Query<Param>) ->HttpResponse {
    let box_info = pg::select_box_info(param.pick_up_code.to_string()).await.unwrap();
    let data = minio_client::minio::get_object(&box_info.file_remote_name()).await;
    let cd = ContentDisposition {
        disposition: DispositionType::FormData,
        parameters: vec![
            DispositionParam::Filename(box_info.file_name().to_string()),
        ],
    };
    let mut builder = HttpResponse::Ok();
    builder.content_type("application/octet-stream");
    // 设置 代表前端可以在Content-Disposition获取数据
    builder.append_header(("Access-Control-Expose-Headers","Content-Disposition"));
    builder.insert_header((actix_web::http::header::CONTENT_DISPOSITION, cd));
    builder.body(data)
}
#[post("/uploadFile")]
async fn upload_file() -> impl Responder {
    HttpResponse::Ok().body("哈啊")
}
