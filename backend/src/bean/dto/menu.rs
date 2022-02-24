use serde::{Serialize, Deserialize};

/// 菜单列表参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuListDTO {

}

/// 菜单新增参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuAddDTO {
    pub name: Option<String>,
    pub parent_id: Option<String>,
    pub route_name: Option<String>,
    pub sort: Option<i32>,
}

/// 菜单修改参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuUpdateDTO {
    pub id: Option<String>,
    pub name: Option<String>,
    pub parent_id: Option<String>,
    pub route_name: Option<String>,
    pub sort: Option<i32>,
}