use rbatis::DateTimeNative;
use rbatis::crud_table;
use rbatis::impl_field_name_method;

///字典表
#[crud_table]
#[derive(Clone, Debug)]
pub struct SysDict {
    pub id: Option<String>,
    pub name: Option<String>,
    pub code: Option<String>,
    pub state: Option<i32>,
    pub create_date: Option<DateTimeNative>,
}
impl_field_name_method!(SysDict{id, name, code, state, create_date});

/// 用户表
#[crud_table]
#[derive(Clone, Debug)]
pub struct SysUser {
    pub id: Option<String>,
    pub account: Option<String>,
    pub password: Option<String>,
    pub name: Option<String>,
    pub create_date: Option<DateTimeNative>,
}
impl_field_name_method!(SysUser{id, account, password, name, create_date});

/// 角色表
#[crud_table]
#[derive(Clone, Debug)]
pub struct SysRole {
    pub id: Option<String>,
    pub name: Option<String>,
    pub create_date: Option<DateTimeNative>,
}
impl_field_name_method!(SysRole{id, name, create_date});