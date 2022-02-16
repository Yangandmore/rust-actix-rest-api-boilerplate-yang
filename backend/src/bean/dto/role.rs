use serde::{Serialize, Deserialize};

/// 角色分页参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleListDTO {
    /// 页号
    pub page_index: Option<u64>,
    /// 页量
    pub page_count: Option<u64>,
    /// 名称
    pub name: Option<String>,
    /// 创建时间
    pub create_date: Option<rbatis::DateTimeNative>,
}

/// 角色新增参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleAddDTO {
    pub name: Option<String>,
}

/// 角色修改参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleUpdateDTO {
    pub id: Option<String>,
    pub name: Option<String>,
}

