use actix_web::{Responder, web};
use crate::service::CONTEXT;
use crate::bean::vo::ResVO;
use crate::bean::dto::menu::{MenuAddDTO, MenuUpdateDTO, MenuListDTO};
use crate::bean::dto::IdDTO;

/// 目录数状列表
pub async fn list(menu: web::Json<MenuListDTO>) -> impl Responder {
    let list = CONTEXT.sys_menu_service.list().await;
    ResVO::unwrap(list).to_json()
}

/// 新增目录
pub async fn add(menu: web::Json<MenuAddDTO>) -> impl Responder {
    if let Err(err) = CONTEXT.sys_menu_service.is_not_empty(&menu.0) {
        return ResVO::<String>::unwrap_error_string("", err.description()).to_json();
    }

    if let Err(err) = CONTEXT.sys_menu_service.has_name_and_route_name(&menu.0).await {
        // 存在该菜单
        return  ResVO::<String>::unwrap_error_string("", err.description()).to_json();
    }

    if let Err(err) = CONTEXT.sys_menu_service.add(menu.0).await {
        return  ResVO::<String>::unwrap_error_string("", err.description()).to_json();
    }

    ResVO::<String>::unwrap_success_string("", "添加成功").to_json()
}

/// 修改目录
pub async fn update(menu: web::Json<MenuUpdateDTO>) -> impl Responder {
    if let Err(err) = CONTEXT.sys_menu_service.is_not_empty_add_id(&menu.0) {
        return ResVO::<String>::unwrap_error_string("", err.description()).to_json();
    }
    if let Err(err) = CONTEXT.sys_menu_service.has_menu_id(&menu.0).await {
        // 不存在该菜单
        return  ResVO::<String>::unwrap_error_string("", err.description()).to_json();
    }
    if let Err(error) = CONTEXT.sys_menu_service.update(menu.0).await {
        return  ResVO::<String>::unwrap_error_string("", error.description()).to_json();
    }
    ResVO::<String>::unwrap_success_string("", "修改成功").to_json()
}

/// 删除目录
pub async fn delete(menu: web::Json<IdDTO>) -> impl Responder {
    if let Err(err) = CONTEXT.sys_menu_service.has_id(&menu.0).await {
        // 不存在该目录或者存在子目录
        return  ResVO::<String>::unwrap_error_string("", err.description()).to_json();
    }
    if let Err(err) = CONTEXT.sys_menu_service.is_root_menu(&menu.0) {
        return  ResVO::<String>::unwrap_error_string("", err.description()).to_json();
    }
    if let Err(error) = CONTEXT.sys_menu_service.delete(&menu.0.id.unwrap_or_default()).await {
        return  ResVO::<String>::unwrap_error_string("", error.description()).to_json();
    }
    ResVO::<String>::unwrap_success_string("", "删除成功").to_json()
}