use crate::bean::tables::SysDict;

/// 字典出参数
#[rbatis::crud_table]
#[derive(Debug, Clone)]
pub struct SysDictVO {
    pub id: Option<String>,
    pub name: Option<String>,
    pub code: Option<String>,
    pub state: Option<i32>,
    pub create_date: Option<rbatis::DateTimeNative>,
}

impl From<SysDict> for SysDictVO {
    fn from(dict: SysDict) -> Self {
        Self {
            id: dict.id,
            name: dict.name,
            code: dict.code,
            state: dict.state,
            create_date: dict.create_date,
        }
    }
}