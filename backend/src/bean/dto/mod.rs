/// 入口bean
///
/// 字典
pub mod dict;
/// 用户
pub mod user;
/// 角色
pub mod role;

use serde::{Serialize, Deserialize};

/// 公用ID DTO
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IdDTO {
    pub id: Option<String>,
}