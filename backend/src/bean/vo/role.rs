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
    fn from(role: SysRole) -> Self {
        Self {
            id: role.id,
            name: role.name,
            create_date: role.create_date,
        }
    }
}