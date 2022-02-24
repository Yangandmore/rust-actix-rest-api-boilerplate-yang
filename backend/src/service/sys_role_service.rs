use crate::bean::dto::role::{RoleListDTO, RoleAddDTO, RoleUpdateDTO};
use crate::config::error::{RestResult, Error};
use rbatis::{Page, PageRequest, IPage, IPageRequest, DateTimeNative};
use crate::bean::tables::SysRole;
use crate::bean::vo::role::SysRoleVO;
use crate::service::CONTEXT;
use rbatis::crud::{CRUD, Skip};
use crate::bean::dto::IdDTO;
use rbatis::plugin::object_id::ObjectId;

/// 角色服务
pub struct SysRoleService {}

impl SysRoleService {

    /// 判断 RoleUpdateDTO 中 id、name 数据不为空
    pub fn is_not_empty_add_id(&self, role: &RoleUpdateDTO) -> RestResult<()> {
        if role.id.is_none() {
            return Err(Error::from("角色id为空"));
        }
        if role.name.is_none() {
            return Err(Error::from("角色名称为空"));
        }
        Ok(())
    }

    /// 判断 RoleAddDTO 中 name 数据不为空
    pub fn is_not_empty(&self, role: &RoleAddDTO) -> RestResult<()> {
        if role.name.is_none() {
            return Err(Error::from("角色名称为空"));
        }
        Ok(())
    }

    /// 判断是否存在重名
    pub async fn has_name(&self, role: &RoleAddDTO) -> RestResult<()> {
        let vec = CONTEXT
            .rbatis
            .fetch_list_by_wrapper::<SysRole>(
                CONTEXT
                    .rbatis
                    .new_wrapper()
                    .eq(SysRole::name(), &role.name)
            ).await?;
        if vec.len() > 0 {
            Err(Error::from("角色已存在"))
        } else {
            Ok(())
        }
    }

    /// 根据 RoleUpdateDTO 中id查找是否存在该角色
    pub async fn has_role_id(&self, role: &RoleUpdateDTO) -> RestResult<()> {
        let vec = CONTEXT
            .rbatis
            .fetch_list_by_wrapper::<SysRole>(
                CONTEXT
                    .rbatis
                    .new_wrapper()
                    .eq(SysRole::id(), &role.id)
            ).await?;
        if vec.len() > 0 {
            Ok(())
        } else {
            Err(Error::from("角色不存在"))
        }
    }

    /// 根据 IdDTO 中id查找角色
    pub async fn has_id(&self, role: &IdDTO) -> RestResult<()> {
        let vec = CONTEXT
            .rbatis
            .fetch_list_by_wrapper::<SysRole>(
                CONTEXT
                    .rbatis
                    .new_wrapper()
                    .eq(SysRole::id(), &role.id)
            )
            .await?;
        if vec.len() > 0 {
            Ok(())
        } else {
            Err(Error::from("角色不存在"))
        }
    }

    /// 角色分页
    pub async fn list(&self, list: &RoleListDTO) -> RestResult<Page<SysRoleVO>>{
        let request = PageRequest::new(list.page_index.unwrap_or(1), list.page_count.unwrap_or(10));
        let mut wrapper = CONTEXT.rbatis.new_wrapper();
        if list.name.is_some() {
            wrapper = wrapper.eq(SysRole::name(), &list.name).and();
        }
        let data = CONTEXT
            .rbatis
            .fetch_page_by_wrapper::<SysRole>(
                wrapper
                    .order_by(false, &[SysRole::create_date()]),
                &request,
            ).await?;

        let mut page = Page::<SysRoleVO>::new(request.page_no, request.page_size);
        let mut records = vec![];
        for dict in data.records {
            let vo = SysRoleVO::from(dict);
            records.push(vo);
        }

        page.pages = data.pages;
        page.set_records(records);
        page.set_total(data.total);
        Ok(page)
    }

    /// 角色新增
    pub async fn add(&self, role: RoleAddDTO) -> RestResult<()> {
        let data = SysRole {
            id: Some(ObjectId::new().to_string()),
            name: role.name.clone(),
            create_date: DateTimeNative::now().into()
        };
        if 1 == CONTEXT.rbatis.save::<SysRole>(&data, &[]).await?.rows_affected {
            Ok(())
        } else {
            Err(Error::from("添加失败"))
        }
    }

    /// 角色修改
    pub async fn update(&self, role: &RoleUpdateDTO) -> RestResult<()> {
        let mut data = SysRole {
            id: role.id.clone(),
            name: role.name.clone(),
            create_date: None,
        };
        if 1 == CONTEXT.rbatis.update_by_wrapper(
            &mut data,
            CONTEXT.rbatis
                .new_wrapper()
                .eq(SysRole::id(), &role.id),
            &[Skip::Column(SysRole::id()), Skip::Column(SysRole::create_date())],
        ).await? {
            Ok(())
        } else {
            Err(Error::from("修改失败"))
        }
    }

    /// 角色删除
    pub async fn delete(&self, id: &str) -> RestResult<()> {
        if 1 == CONTEXT
            .rbatis
            .remove_by_wrapper::<SysRole>(
                CONTEXT.rbatis
                    .new_wrapper()
                    .eq(SysRole::id(), &id)
            ).await? {
            Ok(())
        } else {
            Err(Error::from("删除失败"))
        }
    }
}