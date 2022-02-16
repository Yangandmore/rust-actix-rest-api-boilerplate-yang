/// 服务层
///
/// 系统字典服务
mod sys_dict_service;
/// 系统用户服务
mod sys_user_service;
/// 系统角色服务
mod sys_role_service;

use crate::config::{config::AppConfig, mysql};
use lazy_static::lazy_static;
use rbatis::rbatis::Rbatis;
use crate::service::sys_dict_service::SysDictService;
use crate::service::sys_user_service::SysUserService;
use crate::service::sys_role_service::SysRoleService;

pub struct ServiceContext {
    pub config: AppConfig,
    pub rbatis: Rbatis,

    // 服务部分
    pub sys_dict_service: SysDictService,
    pub sys_user_service: SysUserService,
    pub sys_role_service: SysRoleService,
}

impl Default for ServiceContext {
    fn default() -> Self {
        let config = AppConfig::default();
        ServiceContext {
            rbatis: async_std::task::block_on(async {
                mysql::init_rbatis(&config).await
            }),
            config,

            // 字典服务部分
            sys_dict_service: SysDictService{},
            // 用户服务部分
            sys_user_service: SysUserService{},
            // 角色服务部分
            sys_role_service: SysRoleService{},
        }
    }
}

lazy_static! {
    pub static ref CONTEXT: ServiceContext = ServiceContext::default();
}