use crate::config::error::{RestResult, Error};
use crate::bean::vo::menu::SysMenuVO;
use crate::service::CONTEXT;
use rbatis::{Page, DateTimeNative};
use rbatis::crud::{CRUD, Skip};
use crate::bean::tables::SysMenu;
use crate::bean::dto::menu::{MenuAddDTO, MenuUpdateDTO};
use rbatis::plugin::object_id::ObjectId;
use crate::bean::dto::IdDTO;

pub struct SysMenuService {}

impl SysMenuService {

    /// 判断是否为主菜单，如果是则不允许
    pub fn is_root_menu(&self, menu: &IdDTO) -> RestResult<()> {
        if "0".to_string() == menu.id.clone().unwrap() {
            Err(Error::from("不允许删除主菜单"))
        } else {
            Ok(())
        }
    }

    /// 判断 MenuUpdateDTO 中 id、name、route_name、parent_id、sort 数据不为空
    pub fn is_not_empty_add_id(&self, menu: &MenuUpdateDTO) -> RestResult<()> {
        if menu.id.is_none() {
            return Err(Error::from("目录id为空"));
        }
        if menu.name.is_none() {
            return Err(Error::from("菜单名称为空"));
        }
        if menu.route_name.is_none() {
            return Err(Error::from("菜单路由为空"));
        }
        if menu.parent_id.is_none() {
            return Err(Error::from("父菜单为空"));
        }
        if menu.sort.is_none() {
            return Err(Error::from("菜单排序为空"));
        }
        Ok(())
    }

    /// 判断 MenuAddDTO 中字段是否为空
    pub fn is_not_empty(&self, menu: &MenuAddDTO) -> RestResult<()> {
        if menu.name.is_none() {
            return Err(Error::from("菜单名称为空"));
        }
        if menu.route_name.is_none() {
            return Err(Error::from("菜单路由为空"));
        }
        if menu.parent_id.is_none() {
            return Err(Error::from("父菜单为空"));
        }
        if menu.sort.is_none() {
            return Err(Error::from("菜单排序为空"));
        }
        Ok(())
    }

    /// 判断是否存在重名和路由
    pub async fn has_name_and_route_name(&self, menu: &MenuAddDTO) -> RestResult<()> {
        let vec = CONTEXT.rbatis.fetch_list_by_wrapper::<SysMenu>(
            CONTEXT.rbatis.new_wrapper()
                .eq(SysMenu::name(), &menu.name)
                .and()
                .eq(SysMenu::route_name(), &menu.route_name)
        ).await?;

        if vec.len() > 0 {
            Err(Error::from("菜单已存在"))
        } else {
            Ok(())
        }
    }

    /// 根据 MenuUpdateDTO 中 id 查找是否存在
    pub async fn has_menu_id(&self, menu: &MenuUpdateDTO) -> RestResult<()> {
        let vec = CONTEXT.rbatis.fetch_list_by_wrapper::<SysMenu>(
            CONTEXT.rbatis.new_wrapper()
                .eq(SysMenu::id(), &menu.id)
        ).await?;
        if vec.len() > 0 {
            Ok(())
        } else {
            Err(Error::from("目录不存在"))
        }
    }

    /// 根据 IdDTO 中id查找目录
    pub async fn has_id(&self, menu: &IdDTO) -> RestResult<()> {
        let vec = CONTEXT
            .rbatis
            .fetch_list_by_wrapper::<SysMenu>(
                CONTEXT
                    .rbatis
                    .new_wrapper()
                    .eq(SysMenu::id(), &menu.id)
            )
            .await?;
        if vec.len() > 0 {
            let parent = CONTEXT.rbatis
                .fetch_list_by_wrapper::<SysMenu>(
                    CONTEXT.rbatis
                        .new_wrapper()
                        .eq(SysMenu::parent_id(), &menu.id)
                )
                .await?;
            if parent.len() > 0 {
                Err(Error::from("目录中存在子目录，需要先删除子目录"))
            } else {
                Ok(())
            }
        } else {
            Err(Error::from("目录不存在"))
        }
    }

    /// 目录输状列表
    pub async fn list(&self) -> RestResult<Page<SysMenuVO>> {
        let master_data = CONTEXT.rbatis.fetch_list_by_wrapper::<SysMenu>(
            CONTEXT.rbatis.new_wrapper()
                .eq(SysMenu::parent_id(), 0)
                .and()
                .order_by(true, &[SysMenu::sort()])
        ).await?;

        let no_master_data = CONTEXT.rbatis.fetch_list_by_wrapper::<SysMenu>(
            CONTEXT.rbatis.new_wrapper()
                .ne(SysMenu::parent_id(), 0)
                .and()
                .order_by(true, &[SysMenu::sort()])
        ).await?;

        let mut page: Vec<SysMenuVO> = vec![];
        for data in master_data {
            let new_data = SysMenuVO::from_vo(data, &no_master_data);
            page.push(new_data);
        }
        Ok(Page{
            records: SysMenuVO::to_root_menu(page),
            total: 1,
            pages: 1,
            page_no: 1,
            page_size: 1,
            search_count: true
        })
    }

    /// 新增目录
    pub async fn add(&self, menu: MenuAddDTO) -> RestResult<()> {
        let data = SysMenu {
            id: Some(ObjectId::new().to_string()),
            name: menu.name.clone(),
            route_name: menu.route_name.clone(),
            parent_id: menu.parent_id.clone(),
            sort: menu.sort.clone(),
            create_date: DateTimeNative::now().into()
        };
        if 1 == CONTEXT.rbatis.save::<SysMenu>(&data, &[]).await?.rows_affected {
            Ok(())
        } else {
            Err(Error::from("添加失败"))
        }
    }

    /// 修改目录
    pub async fn update(&self, menu: MenuUpdateDTO) -> RestResult<()> {
        let mut data = SysMenu {
            id: menu.id.clone(),
            name: menu.name.clone(),
            route_name: menu.route_name.clone(),
            parent_id: menu.parent_id.clone(),
            sort: menu.sort.clone(),
            create_date: None
        };
        if 1 == CONTEXT.rbatis.update_by_wrapper(
            &mut data,
            CONTEXT.rbatis.new_wrapper()
                .eq(SysMenu::id(), &menu.id),
            &[Skip::Column(SysMenu::id()), Skip::Column(SysMenu::create_date())]
        ).await? {
            Ok(())
        } else {
            Err(Error::from("修改失败"))
        }
    }

    /// 目录删除
    pub async fn delete(&self, id: &str) -> RestResult<()> {
        if 1 == CONTEXT
            .rbatis
            .remove_by_wrapper::<SysMenu>(
                CONTEXT.rbatis
                    .new_wrapper()
                    .eq(SysMenu::id(), &id)
            ).await? {
            Ok(())
        } else {
            Err(Error::from("删除失败"))
        }
    }

}