use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize,ToSchema)]
pub struct SendCodeReq {
    pub email: String,
}

#[derive(Debug, Serialize,ToSchema)]
pub struct SendCodeResp {
    pub msg: String,
}

#[derive(Debug, Deserialize,ToSchema)]
pub struct LoginReq {
    pub email: String,
    pub code: String,
}

#[derive(Debug, Serialize,ToSchema)]
pub struct LoginResp {
    pub token: String,
}

// #[derive(Debug, Serialize,ToSchema)]
// pub struct UserInfo {
//     pub id: String,
//     pub email: String,
//     pub name: Option<String>,
// }