use crate::config::error::RestResult;
use crate::bean::dto::dict::*;
use crate::bean::vo::dict::*;
use crate::bean::tables::*;
use rbatis::{PageRequest, Page, IPage, IPageRequest};
use crate::service::CONTEXT;
use rbatis::crud::CRUD;

/// 字典服务
pub struct SysDictService {}

impl SysDictService {

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
}