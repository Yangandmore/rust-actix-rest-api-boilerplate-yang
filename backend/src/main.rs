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

            .route("/role/add", web::post().to(sys_role_controller::add))
            .route("/role/update", web::post().to(sys_role_controller::update))
            .route("/role/delete", web::post().to(sys_role_controller::delete))
            .route("/role/list", web::post().to(sys_role_controller::list))

            .route("/menu/add", web::post().to(sys_menu_controller::add))
            .route("/menu/update", web::post().to(sys_menu_controller::update))
            .route("/menu/delete", web::post().to(sys_menu_controller::delete))
            .route("/menu/list", web::post().to(sys_menu_controller::list))
    })
        .bind(&CONTEXT.config.server_url)?
        .run()
        .await
}