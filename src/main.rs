extern crate sanitize_filename;

use actix_web::{get, middleware, post, web, App, HttpServer};
use config::Config;
use core::api_response::ApiResponse;
use std::collections::HashMap;
use actix_multipart::Multipart;
use crate::core::ext::MultipartExt;

mod core;

#[get("/")]
async fn hello() -> ApiResponse {
    ApiResponse::default()
}

#[post("/echo")]
async fn echo(_: String) -> ApiResponse {
    ApiResponse::builder()
        .with_message("Fuckyou".to_string())
        .with_data(34)
        .build()
}

#[get("/config")]
async fn get_settings(settings: web::Data<Config>) -> ApiResponse {
    let settings_map: HashMap<String, String> =
        settings.get_ref().clone().try_deserialize().unwrap();
    ApiResponse::builder().with_data(settings_map).build()
}

#[post("/upload")]
async fn upload_file(mut payload: Multipart) -> ApiResponse {
    if let Err(_) = payload.save_file().await {
        return ApiResponse::builder()
            .with_code(123)
            .with_message("error with uploading file".to_string())
            .build();
    }
    ApiResponse::builder()
        .with_message("success file uploading".to_string())
        .build()
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let settings = Config::builder()
        .add_source(config::File::with_name("Settings"))
        .build()
        .unwrap();

    log::info!("creating temporary upload directory");
    std::fs::create_dir_all("./tmp")?;

    log::info!("starting HTTP server at http://localhost:8080");
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(settings.clone()))
            .service(hello)
            .service(echo)
            .service(get_settings)
            .service(upload_file)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
