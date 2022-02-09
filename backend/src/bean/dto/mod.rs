pub mod dict;

use serde::{Serialize, Deserialize};

/// 公用ID DTO
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IdDTO {
    pub id: Option<String>,
}