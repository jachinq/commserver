use std::io;

use actix_cors::Cors;
use actix_web::{App, HttpResponse, HttpServer, Responder, middleware::Logger, web};
use env_logger::Env;
use lib_log::log;

mod controller;
mod model;
mod utils;

use controller::auth::*;
use lib_sql::utils::read_config;
use model::*;
use serde::{Deserialize, Serialize};

use crate::utils::table;

// 应用状态
#[derive(Clone, Debug, Deserialize, Serialize, Default)]
struct AppState {}

#[actix_web::main]
async fn main() -> io::Result<()> {
    // 初始化日志记录
    unsafe {
        std::env::set_var("RUST_LOG", "debug");
    }
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    // 初始化数据库
    if let Err(e) = table::init() {
        log::log_err(&format!("init database error: {:?}", e));
        return Err(io::Error::new(io::ErrorKind::Other, "init database error"));
    };

    
    let config = read_config("./conf/config.toml");
    if let Err(e) = config {
        log::log_err(&format!("read config error: {:?}", e));
        return Err(io::Error::new(io::ErrorKind::Other, "read config error"));
    }
    let config = config.unwrap();

    // 静态文件目录
    let web_dir = config.webdir.unwrap_or("./web".to_string());
    let port = config.port.unwrap_or(3000);

    // 启动定时任务
    // job::account::start();

    let ip = "0.0.0.0";
    log::log_info(&format!("Server started on http://{}:{}", ip, port));
    // 启动 HTTP 服务
    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default() // 添加 CORS 中间件
                    .allow_any_origin() // 允许所有来源的跨域请求，你可以根据需要更改为特定的域名
                    .allow_any_method() // 允许所有方法，如 GET, POST, PUT, DELETE 等
                    .allow_any_header(), // 允许所有请求头，你可以根据需要更改为特定的请求头
            )
            // 认证相关路由
            .route("/api/auth/me", web::get().to(handle_me))
            .route("/api/auth/login", web::post().to(handle_login))
            .route("/api/auth/register", web::post().to(handle_register))
            .route("/api/auth/logout", web::post().to(handle_logout))
            .route("/api/auth/refresh", web::post().to(handle_refresh))
            .route("/api/auth/verify", web::post().to(handle_verify))
            .route("/api/auth/password", web::put().to(handle_change_password))
            .route(
                "/api/auth/reset-password",
                web::post().to(handle_reset_password),
            )
            // 心跳路由
            .route("/api/ping", web::route().to(handle_ping))
            .wrap(Logger::default()) // 日志记录中间件
            .app_data(web::JsonConfig::default().error_handler(handle_server_error)) //
            .app_data(web::Data::new(AppState {}))
            .service(actix_files::Files::new("/", &web_dir).index_file("index.html"))
            .default_service(web::get().to(handle_all_others))
    })
    .bind(&format!("{}:{}", ip, port))?
    .run()
    .await
}

// 心跳
async fn handle_ping() -> HttpResponse {
    HttpResponse::Ok().json("pong")
}

// 处理所有其他请求，重定向到 index.html
// async fn handle_all_others(_app_data: web::Data<AppState>) -> Result<NamedFile, actix_web::Error> {
//     let web_path = "";
//     let index_path = format!("{}/index.html", web_path);
//     let path = Path::new(&index_path);
//     NamedFile::open(path).map_err(|_| actix_web::error::ErrorNotFound("index.html not found"))
// }
// 处理所有其他请求，返回错误信息
async fn handle_all_others(_app_data: web::Data<AppState>) -> impl Responder {
    HttpResponse::BadRequest().json(JsonResult::<()>::error("not found`"))
}

fn handle_server_error(
    err: actix_web::error::JsonPayloadError,
    _req: &actix_web::HttpRequest,
) -> actix_web::Error {
    let err2 = JsonResult::<()>::error(&err.to_string());
    let err2 = serde_json::to_string(&err2).unwrap_or_default();

    actix_web::error::ErrorInternalServerError(err2)
}
