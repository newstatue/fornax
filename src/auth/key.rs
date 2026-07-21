pub const EMAIL_CODE_PREFIX:&str = "auth-email-code:";

pub fn email_code(email: &str) -> String{
    format!("{EMAIL_CODE_PREFIX}{email}")
}