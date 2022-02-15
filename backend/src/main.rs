use actix_web::{HttpServer, App, web};
use actix_cors::Cors;
use actix_web::middleware::Logger;
use backend::service::CONTEXT;
use backend::controller::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // 初始化日志
    backend::config::log::init_log();

    HttpServer::new(|| {
        App::new()
            // 日志处理
            .wrap(Logger::default())
            // 跨域处理
            .wrap(Cors::permissive())
            // 接口
            .route("/dict/add", web::post().to(sys_dict_controller::add))
            .route("/dict/update", web::post().to(sys_dict_controller::update))
            .route("/dict/delete", web::post().to(sys_dict_controller::delete))
            .route("/dict/list", web::post().to(sys_dict_controller::list))
    })
        .bind(&CONTEXT.config.server_url)?
        .run()
        .await
}