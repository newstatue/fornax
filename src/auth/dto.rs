use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use garde::Validate;

#[derive(Debug, Deserialize,ToSchema,Validate)]
pub struct SendCodeReq {
    #[garde(email)]
    pub email: String,
}

#[derive(Debug, Serialize,ToSchema)]
pub struct SendCodeResp {
    pub cd: u64,
}

#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct LoginReq {
    #[garde(email)]
    pub email: String,

    #[garde(custom(validate_code))]
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

fn validate_code(code: &str, _: &()) -> garde::Result {
    if code.len() != 5 {
        return Err(garde::Error::new("验证码必须为5位"));
    }

    if !code.bytes().all(|b| b.is_ascii_digit()) {
        return Err(garde::Error::new("验证码只能包含数字"));
    }

    Ok(())
}