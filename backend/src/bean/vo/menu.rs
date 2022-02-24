use crate::bean::tables::SysMenu;

/// 目录出参数
#[rbatis::crud_table]
#[derive(Debug, Clone)]
pub struct SysMenuVO {
    pub id: Option<String>,
    pub name: Option<String>,
    pub route_name: Option<String>,
    pub parent_id: Option<String>,
    pub sort: Option<i32>,
    pub create_date: Option<rbatis::DateTimeNative>,
    pub children: Option<Vec<SysMenuVO>>,
}

impl SysMenuVO {

    /// 将child数组包装在root菜单上
    pub fn to_root_menu(child: Vec<SysMenuVO>) -> Vec<SysMenuVO> {
        vec![
            SysMenuVO {
                id: Some("0".to_string()),
                name: Some("主菜单".to_string()),
                route_name: Some("/".to_lowercase()),
                parent_id: None,
                sort: Some(1),
                create_date: None,
                children: Some(child)
            }
        ]
    }

    /// SysMenu转为SysMenuVO入口
    pub fn from_vo(menu: SysMenu, child: &Vec<SysMenu>) -> Self {
        let id = menu.id.clone();

        let new_child: Vec<SysMenuVO> = SysMenuVO::get_children(id.unwrap().as_str(), child);

        Self {
            id: menu.id,
            name: menu.name,
            route_name: menu.route_name,
            parent_id: menu.parent_id,
            sort: menu.sort,
            create_date: menu.create_date,
            children: Some(new_child),
        }
    }

    /// 递归获取child
    fn get_children(old_data: &str, child: &Vec<SysMenu>) -> Vec<SysMenuVO> {
        let mut new_child: Vec<SysMenuVO> = vec![];

        for menu in child {
            if menu.parent_id.as_ref().unwrap().as_str() == old_data {
                // 深入查询 SysMenuVO
                let c = SysMenuVO::get_children(&menu.id.clone().unwrap().as_str(), child);
                // 数据转为 SysMenuVO
                new_child.push(SysMenuVO::add_child(menu.clone(), c));
            }
        }
        new_child
    }

    // 添加child数组
    fn add_child(menu: SysMenu, child: Vec<SysMenuVO>) -> Self {
        Self {
            id: menu.id,
            name: menu.name,
            route_name: menu.route_name,
            parent_id: menu.parent_id,
            sort: menu.sort,
            create_date: menu.create_date,
            children: Some(child),
        }
    }
}