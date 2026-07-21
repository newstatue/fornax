use crate::common::var;

pub const EMAIL_SUBJECT: &str = "您的FORNAX验证码: ";

pub const EMAIL_SUCCESS: &str = "验证码发送成功!";

pub fn email_html(code: &str) -> String {
    let app_name = var::APP_NAME;

    format!(
        r#"
        <h2>{app_name}登录验证码</h2>
        <p>你的验证码是：</p>
        <h1>{code}</h1>
        <p>验证码有效期为 5 分钟，请勿泄露。</p>
        "#
    )
}
pub fn email_text(code: &str) -> String {
    let app_name = var::APP_NAME;
    format!("您的 {app_name} 验证码是：{code}，有效期为 5 分钟，请勿泄露。")
}