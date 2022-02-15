use crate::config::error::{RestResult, Error};
use crate::bean::dto::{dict::*, IdDTO};
use crate::bean::vo::dict::*;
use crate::bean::tables::*;
use rbatis::{PageRequest, Page, IPage, IPageRequest};
use crate::service::CONTEXT;
use rbatis::crud::{CRUD, Skip};

/// 字典服务
pub struct SysDictService {}

impl SysDictService {

    pub fn is_not_empty_add_id(&self, dict: &DictUpdateDTO) -> RestResult<()> {
        if dict.id.is_none() {
            return Err(Error::from("字典id为空"));
        }
        if dict.name.is_none() {
            return Err(Error::from("字典名称为空"));
        }
        if dict.code.is_none() {
            return Err(Error::from("字典代码为空"));
        }
        Ok(())
    }

    pub fn is_not_empty(&self, dict: &DictAddDTO) -> RestResult<()> {
        if dict.name.is_none() {
            return Err(Error::from("字典名称为空"));
        }
        if dict.code.is_none() {
            return Err(Error::from("字典代码为空"));
        }
        Ok(())
    }

    pub async fn has_dict_id(&self, dict: &DictUpdateDTO) -> RestResult<()> {
        let vec = CONTEXT
            .rbatis
            .fetch_list_by_wrapper::<SysDict>(
                CONTEXT
                    .rbatis
                    .new_wrapper()
                    .eq(SysDict::id(), &dict.id)
            )
            .await?;
        if vec.len() > 0 {
            Ok(())
        } else {
            Err(Error::from("字典不存在"))
        }
    }

    pub async fn has_id(&self, dict: &IdDTO) -> RestResult<()> {
        let vec = CONTEXT
            .rbatis
            .fetch_list_by_wrapper::<SysDict>(
                CONTEXT
                    .rbatis
                    .new_wrapper()
                    .eq(SysDict::id(), &dict.id)
            )
            .await?;
        if vec.len() > 0 {
            Ok(())
        } else {
            Err(Error::from("字典不存在"))
        }
    }

    pub async fn has_name_and_code(&self, dict: &DictAddDTO) -> RestResult<()> {
        let vec = CONTEXT
            .rbatis
            .fetch_list_by_wrapper::<SysDict>(
                CONTEXT
                    .rbatis
                    .new_wrapper()
                    .eq(SysDict::code(), &dict.code)
                    .and()
                    .eq(SysDict::name(), &dict.name),
            )
            .await?;
        if vec.len() > 0 {
            Err(Error::from("字典已存在"))
        } else {
            Ok(())
        }
    }

    /// 字典分页
    pub async fn list(&self, list: &DictListDTO) -> RestResult<Page<SysDictVO>> {
        let request = PageRequest::new(list.page_index.unwrap_or(1), list.page_count.unwrap_or(10));
        let mut wrapper = CONTEXT.rbatis.new_wrapper();
        if list.name.is_some() {
            wrapper = wrapper.eq(SysDict::name(), &list.name).and();
        }
        if list.code.is_some() {
            wrapper = wrapper.eq(SysDict::code(), &list.code).and();
        }
        if list.state.is_some() {
            wrapper = wrapper.eq(SysDict::state(), &list.state).and();
        }
        let data = CONTEXT
            .rbatis
            .fetch_page_by_wrapper::<SysDict>(
                wrapper
                    .order_by(false, &[SysDict::create_date()]),
                &request,
            ).await?;

        let mut page = Page::<SysDictVO>::new(request.page_no, request.page_size);
        let mut records = vec![];
        for dict in data.records {
            let vo = SysDictVO::from(dict);
            records.push(vo);
        }

        page.pages = data.pages;
        page.set_records(records);
        page.set_total(data.total);
        Ok(page)
    }

    /// 字典新增
    pub async fn add(&self, dict: SysDict) -> RestResult<()> {
        if 1 == CONTEXT.rbatis.save(&dict, &[]).await?.rows_affected {
            Ok(())
        } else {
            Err(Error::from("添加失败"))
        }
    }

    /// 字典修改
    pub async fn update(&self, dict: &DictUpdateDTO) -> RestResult<()> {
        let mut data = SysDict {
            id: dict.id.clone(),
            name: dict.name.clone(),
            code: dict.code.clone(),
            state: dict.state.clone(),
            create_date: None,
        };
        if 1 == CONTEXT
            .rbatis
            .update_by_wrapper(
                &mut data,
                CONTEXT.rbatis
                    .new_wrapper()
                    .eq(SysDict::id(), &dict.id),
                &[Skip::Column(SysDict::id()), Skip::Column(SysDict::create_date())],
            ).await? {
            Ok(())
        } else {
            Err(Error::from("修改失败"))
        }
    }

    /// 字典删除
    pub async fn delete(&self, id: &str) -> RestResult<()> {
        if 1 == CONTEXT
            .rbatis
            .remove_by_wrapper::<SysDict>(
                CONTEXT.rbatis
                    .new_wrapper()
                    .eq(SysDict::id(), &id)
            ).await? {
            Ok(())
        } else {
            Err(Error::from("删除失败"))
        }
    }
}