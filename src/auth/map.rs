use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct SendCodeReq {
    pub email: String,
}

#[derive(Debug, Serialize)]
pub struct SendCodeResp {
    pub message: String,
}


/*#[derive(Debug, Deserialize)]
pub struct LoginReq {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResp {
    pub token: String,
    pub user: UserInfo,
}

#[derive(Debug, Serialize)]
pub struct UserInfo {
    pub id: String,
    pub email: String,
    pub name: Option<String>,
}*/