use crate::config::config::AppConfig;
use rbatis::rbatis::Rbatis;
use rbatis::logic_delete::RbatisLogicDeletePlugin;

pub async fn init_rbatis(config: &AppConfig) -> Rbatis {
    let rbatis = Rbatis::new();

    rbatis
        .link(&config.db_url)
        .await
        .expect("数据库连接异常");
    println!("数据库连接成功...{}", config.db_url);
    rbatis
}