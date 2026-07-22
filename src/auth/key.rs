pub const EMAIL_CODE_PREFIX:&str = "auth-email-code:";

pub fn email_code(email: &str) -> String{
    format!("{EMAIL_CODE_PREFIX}{email}")
}


pub const EMAIL_CODE_CD_PREFIX:&str = "auth-email-code-cd:";

pub fn email_code_cd(email: &str) -> String{
    format!("{EMAIL_CODE_CD_PREFIX}{email}")
}