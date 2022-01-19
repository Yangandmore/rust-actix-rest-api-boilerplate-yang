use actix_web::{HttpServer, App};


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // 初始化日志
    backend::config::log::init_log();

    HttpServer::new(|| {
        App::new()
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}