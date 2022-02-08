use actix_web::{Responder, web};
use crate::bean::dto::dict::*;
use crate::bean::vo::ResVO;
use crate::service::CONTEXT;

/// 字典分页
pub async fn list(list: web::Json<DictListDTO>) -> impl Responder {
    let dict = CONTEXT.sys_dict_service.list(&list.0).await;
    ResVO::unwrap(dict).to_json()
}

/// 新增字典
pub async fn add(mut dict: web::Json<DictAddDTO>) -> impl Responder {
    if dict.name.is_none() {

    }
    if dict.code.is_none() {

    }
    if dict.state.is_none() {
        dict.state = Some(1)
    }


}
