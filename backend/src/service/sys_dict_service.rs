use crate::config::error::{RestResult, Error};
use crate::bean::dto::dict::*;
use crate::bean::vo::dict::*;
use crate::bean::tables::*;
use rbatis::{PageRequest, Page, IPage, IPageRequest};
use crate::service::CONTEXT;
use rbatis::crud::CRUD;

/// 字典服务
pub struct SysDictService {}

impl SysDictService {

    pub fn is_empty(&self, dict: &DictAddDTO) -> RestResult<()> {
        if dict.name.is_none() {
            return Err(Error::from("字典名称为空"));
        }
        if dict.code.is_none() {
            return Err(Error::from("字典代码为空"));
        }
        Ok(())
    }

    pub async fn has(&self, dict: &DictAddDTO) -> RestResult<()> {
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
        let data = CONTEXT
            .rbatis
            .fetch_page_by_wrapper::<SysDict>(
                CONTEXT
                    .rbatis
                    .new_wrapper()
                    .order_by(false, &[SysDict::create_date()]),
                &request,
            ).await?;

        let mut page = Page::<SysDictVO>::new(request.page_no, request.page_size);
        let mut records = vec![];
        for dict in data.records {
            let vo = SysDictVO::from(dict);
            records.push(vo);
        }

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

}