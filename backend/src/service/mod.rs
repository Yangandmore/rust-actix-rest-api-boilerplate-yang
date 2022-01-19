/// 服务层

use crate::config::config::AppConfig;
use lazy_static::lazy_static;
use rbatis::rbatis::Rbatis;

pub struct ServiceContext {
    pub config: AppConfig,
    pub rbatis: Rbatis,
}

impl Default for ServiceContext {
    fn default() -> Self {
        let config = AppConfig::default();
        ServiceContext {
            rbatis: async_std::task::block_on(async {
                crate::dao::init_rbatis(&config).await
            }),
            config,
        }
    }
}

lazy_static! {
    pub static ref CONTEXT: ServiceContext = ServiceContext::default();
}