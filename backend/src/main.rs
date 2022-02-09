use actix_web::{HttpServer, App, web};
use backend::service::CONTEXT;
use backend::controller::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // 初始化日志
    backend::config::log::init_log();

    HttpServer::new(|| {
        App::new()
            .route("/dict/add", web::get().to(sys_dict_controller::add))
            .route("/dict/update", web::get().to(sys_dict_controller::update))
            .route("/dict/delete", web::get().to(sys_dict_controller::delete))
            .route("/dict/list", web::get().to(sys_dict_controller::list))
    })
        .bind(&CONTEXT.config.server_url)?
        .run()
        .await
}