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
            .wrap(Logger::default())
            .wrap(Cors::permissive())
            .route("/dict/add", web::get().to(sys_dict_controller::add))
            .route("/dict/update", web::get().to(sys_dict_controller::update))
            .route("/dict/delete", web::get().to(sys_dict_controller::delete))
            .route("/dict/list", web::post().to(sys_dict_controller::list))
    })
        .bind(&CONTEXT.config.server_url)?
        .run()
        .await
}