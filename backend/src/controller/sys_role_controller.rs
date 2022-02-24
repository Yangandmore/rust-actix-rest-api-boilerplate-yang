use actix_web::{Responder, web};
use crate::bean::dto::role::{RoleListDTO, RoleAddDTO, RoleUpdateDTO};
use crate::service::CONTEXT;
use crate::bean::vo::ResVO;
use crate::bean::dto::IdDTO;

/// 角色分页
pub async fn list(list: web::Json<RoleListDTO>) -> impl Responder {
    let role_list = CONTEXT.sys_role_service.list(&list.0).await;
    ResVO::unwrap(role_list).to_json()
}

/// 新增角色
pub async fn add(role: web::Json<RoleAddDTO>) -> impl Responder {
    if let Err(err) = CONTEXT.sys_role_service.is_not_empty(&role.0) {
        return ResVO::<String>::unwrap_error_string("", err.description()).to_json();
    }
    if let Err(err) = CONTEXT.sys_role_service.has_name(&role.0).await {
        return ResVO::<String>::unwrap_error_string("", err.description()).to_json();
    }

    if let Err(err) = CONTEXT.sys_role_service.add(role.0).await {
        return  ResVO::<String>::unwrap_error_string("", err.description()).to_json();
    }
    ResVO::<String>::unwrap_success_string("", "添加成功").to_json()
}

/// 修改角色
pub async fn update(role: web::Json<RoleUpdateDTO>) -> impl Responder {
    if let Err(err) = CONTEXT.sys_role_service.is_not_empty_add_id(&role.0) {
        return ResVO::<String>::unwrap_error_string("", err.description()).to_json();
    }
    if let Err(err) = CONTEXT.sys_role_service.has_role_id(&role.0).await {
        return  ResVO::<String>::unwrap_error_string("", err.description()).to_json();
    }
    if let Err(err) = CONTEXT.sys_role_service.update(&role.0).await {
        return  ResVO::<String>::unwrap_error_string("", err.description()).to_json();
    }
    ResVO::<String>::unwrap_success_string("", "修改成功").to_json()
}

/// 删除角色
pub async fn delete(role: web::Json<IdDTO>) -> impl Responder {
    if let Err(err) = CONTEXT.sys_role_service.has_id(&role.0).await {
        // 不存在该字典
        return  ResVO::<String>::unwrap_error_string("", err.description()).to_json();
    }
    if let Err(err) = CONTEXT.sys_role_service.delete(&role.0.id.unwrap_or_default()).await {
        return  ResVO::<String>::unwrap_error_string("", err.description()).to_json();
    }
    ResVO::<String>::unwrap_success_string("", "删除成功").to_json()

}