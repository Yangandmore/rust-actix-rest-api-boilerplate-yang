use serde::{Serialize, Deserialize};

/// 字典分页参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DictListDTO {
    /// 页号
    pub page_index: Option<u64>,
    /// 页量
    pub page_count: Option<u64>,
    /// 名称
    pub name: Option<String>,
    /// 代码
    pub code: Option<String>,
    /// 启用状态
    pub state: Option<u64>,
    /// 创建时间
    pub create_date: Option<rbatis::DateTimeNative>,
}

/// 字典新增参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DictAddDTO {
    pub name: Option<String>,
    pub code: Option<String>,
    pub state: Option<i32>,
}

/// 字典修改参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DictUpdateDTO {
    pub id: Option<String>,
    pub name: Option<String>,
    pub code: Option<String>,
    pub state: Option<i32>,
}