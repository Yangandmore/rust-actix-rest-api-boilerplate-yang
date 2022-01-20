use actix_web::{HttpServer, App};
use backend::service::CONTEXT;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // 初始化日志
    backend::config::log::init_log();

    HttpServer::new(|| {
        App::new()
    })
        .bind(&CONTEXT.config.server_url)?
        .run()
        .await
}