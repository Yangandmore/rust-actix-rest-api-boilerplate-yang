use serde::{Serialize, Deserialize};
use actix_web::{HttpResponse};
use actix_http::Response;
use serde::de::DeserializeOwned;
use crate::config::error::{Error};

pub mod dict;

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

    /// 处理对象为json responder
    pub fn to_json(&self) -> Response {
        HttpResponse::Ok()
            .set_header("Access-Control-Allow-Origin", "*")
            .set_header("Cache-Control", "no-cache")
            .set_header("Content-Type", "text/json;charset=UTF-8")
            .body(self.to_string())
    }
}

impl <T> ToString for ResVO<T>
where
    T: Serialize + DeserializeOwned + Clone,
{
    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
