use serde::{Serialize, Deserialize};

/// 字典分页参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DictListDTO {
    /// 页号
    pub page_index: Option<u64>,
    /// 页量
    pub page_count: Option<u64>,
}

/// 字典新增参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DictAddDTO {
    pub name: Option<String>,
    pub code: Option<String>,
    pub state: Option<i32>,
}