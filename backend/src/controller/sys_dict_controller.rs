use actix_web::{Responder, web};
use crate::bean::dto::dict::*;
use crate::bean::vo::ResVO;
use crate::service::CONTEXT;
use crate::bean::tables::SysDict;
use rbatis::DateTimeNative;
use rbatis::plugin::object_id::ObjectId;

/// 字典分页
pub async fn list(list: web::Json<DictListDTO>) -> impl Responder {
    let dict = CONTEXT.sys_dict_service.list(&list.0).await;
    ResVO::unwrap(dict).to_json()
}

/// 新增字典
pub async fn add(mut dict: web::Json<DictAddDTO>) -> impl Responder {
    if let Err(err) = CONTEXT.sys_dict_service.is_empty(&dict.0) {
        return ResVO::<String>::unwrap_error_string("", err.description()).to_json();
    }
    if dict.state.is_none() {
        dict.state = Some(1)
    }

    if let Err(err) = CONTEXT.sys_dict_service.has(&dict.0).await {
        // 存在该字典
       return  ResVO::<String>::unwrap_error_string("", err.description()).to_json();
    }

    let res = SysDict {
        id: Some(ObjectId::new().to_string()),
        name: dict.name.clone(),
        code: dict.code.clone(),
        state: dict.state.clone(),
        create_date: DateTimeNative::now().into()
    };
    if let Err(error) = CONTEXT.sys_dict_service.add(res).await {
        return  ResVO::<String>::unwrap_error_string("", error.description()).to_json();
    }

    ResVO::<String>::unwrap_success_string("", "添加成功").to_json()
}
