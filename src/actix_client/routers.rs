use actix_cors::Cors;
use actix_web::{get, http, post, App, HttpResponse, HttpServer, Responder};

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
async fn download_file() -> HttpResponse {
    HttpResponse::Ok().body("get")
}
#[post("/uploadFile")]
async fn upload_file() -> impl Responder {
    HttpResponse::Ok().body("okkk")
}
