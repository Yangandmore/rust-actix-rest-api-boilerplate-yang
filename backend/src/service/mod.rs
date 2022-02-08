/// 服务层
///
/// 系统字典服务
mod sys_dict_service;

use crate::config::config::AppConfig;
use lazy_static::lazy_static;
use rbatis::rbatis::Rbatis;
use crate::service::sys_dict_service::SysDictService;

pub struct ServiceContext {
    pub config: AppConfig,
    pub rbatis: Rbatis,

    // 服务部分
    pub sys_dict_service: SysDictService,
}

impl Default for ServiceContext {
    fn default() -> Self {
        let config = AppConfig::default();
        ServiceContext {
            rbatis: async_std::task::block_on(async {
                crate::dao::init_rbatis(&config).await
            }),
            config,

            // 字典服务部分
            sys_dict_service: SysDictService{},
        }
    }
}

lazy_static! {
    pub static ref CONTEXT: ServiceContext = ServiceContext::default();
}