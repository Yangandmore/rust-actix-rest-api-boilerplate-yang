use actix_web::{Responder, web};
use crate::bean::dto::{dict::*, IdDTO};
use crate::bean::vo::ResVO;
use crate::service::CONTEXT;

/// 字典分页
pub async fn list(list: web::Json<DictListDTO>) -> impl Responder {
    let dict_list = CONTEXT.sys_dict_service.list(&list.0).await;
    ResVO::unwrap(dict_list).to_json()
}

/// 新增字典
pub async fn add(mut dict: web::Json<DictAddDTO>) -> impl Responder {
    if let Err(err) = CONTEXT.sys_dict_service.is_not_empty(&dict.0) {
        return ResVO::<String>::unwrap_error_string("", err.description()).to_json();
    }
    if dict.state.is_none() {
        dict.state = Some(1)
    }

    if let Err(err) = CONTEXT.sys_dict_service.has_name_and_code(&dict.0).await {
        // 存在该字典
       return  ResVO::<String>::unwrap_error_string("", err.description()).to_json();
    }

    if let Err(err) = CONTEXT.sys_dict_service.add(dict.0).await {
        return  ResVO::<String>::unwrap_error_string("", err.description()).to_json();
    }

    ResVO::<String>::unwrap_success_string("", "添加成功").to_json()
}

/// 修改字典
pub async fn update(dict: web::Json<DictUpdateDTO>) -> impl Responder {
    if let Err(err) = CONTEXT.sys_dict_service.is_not_empty_add_id(&dict.0) {
        return ResVO::<String>::unwrap_error_string("", err.description()).to_json();
    }

    if let Err(err) = CONTEXT.sys_dict_service.has_dict_id(&dict.0).await {
        // 不存在该字典
        return  ResVO::<String>::unwrap_error_string("", err.description()).to_json();
    }
    if let Err(error) = CONTEXT.sys_dict_service.update(&dict.0).await {
        return  ResVO::<String>::unwrap_error_string("", error.description()).to_json();
    }
    ResVO::<String>::unwrap_success_string("", "修改成功").to_json()
}

/// 删除字典
pub async fn delete(dict: web::Json<IdDTO>) -> impl Responder {
    if let Err(err) = CONTEXT.sys_dict_service.has_id(&dict.0).await {
        // 不存在该字典
        return  ResVO::<String>::unwrap_error_string("", err.description()).to_json();
    }
    if let Err(error) = CONTEXT.sys_dict_service.delete(&dict.0.id.unwrap_or_default()).await {
        return  ResVO::<String>::unwrap_error_string("", error.description()).to_json();
    }
    ResVO::<String>::unwrap_success_string("", "删除成功").to_json()
}