use rbatis::DateTimeNative;
use crate::bean::tables::SysRole;

/// 角色出参数
#[rbatis::crud_table]
#[derive(Debug, Clone)]
pub struct  SysRoleVO {
    pub id: Option<String>,
    pub name: Option<String>,
    pub create_date: Option<DateTimeNative>,
}

impl From<SysRole> for SysRoleVO {
    fn from(dict: SysRole) -> Self {
        Self {
            id: dict.id,
            name: dict.name,
            create_date: dict.create_date,
        }
    }
}