use serde::{Serialize, Deserialize};
use actix_web::{HttpResponse};
use actix_http::Response;
use serde::de::DeserializeOwned;
use crate::config::error::{Error};
use std::fmt::{Formatter, Display};

/// 出口bean
///
/// 字典
pub mod dict;
/// 用户
pub mod user;
/// 角色
pub mod role;
/// 菜单
pub mod menu;

const SUCCESS: &str = "success";
const FAIL: &str = "fail";

/// 返回数据对象模型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResVO<T> {
    pub code: Option<String>,
    pub msg: Option<String>,
    pub data: Option<T>,
}

impl <T> ResVO<T>
where
    T: Serialize + DeserializeOwned + Clone
{
    /// 打开Option对象
    pub fn unwrap(data: Result<T, Error>) -> Self {
        if data.is_ok() {
            Self {
                code: Some(SUCCESS.to_string()),
                msg: None,
                data: data.ok(),
            }
        } else {
            Self {
                code: Some(FAIL.to_string()),
                msg: Some("".to_string()),
                data: None,
            }
        }
    }

    /// 成功字符串包装
    pub fn unwrap_success_string(code: &str, msg: &str) -> Self {
        let mut code_str = code.to_string();
        if code_str.is_empty() {
            code_str = SUCCESS.to_string();
        }
        Self {
            code: Some(code_str),
            msg: Some(msg.to_string()),
            data: None,
        }

    }

    /// 异常字符串包装
    pub fn unwrap_error_string(code: &str, msg: &str) -> Self {
        let mut code_str = code.to_string();
        if code_str.is_empty() {
            code_str = FAIL.to_string();
        }
        Self {
            code: Some(code_str),
            msg: Some(msg.to_string()),
            data: None,
        }
    }

    /// 处理对象为json responder
    pub fn to_json(&self) -> Response {
        HttpResponse::Ok()
            .set_header("Access-Control-Allow-Origin", "*")
            .set_header("Cache-Control", "no-cache")
            .set_header("Content-Type", "text/json;charset=UTF-8")
            .body(self.to_string())
    }
}

impl <T>Display for ResVO<T>
    where
        T: Serialize + DeserializeOwned + Clone,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
