pub const LOGIN_SUCCESS: &str = "登录成功";

pub const EMAIL_SUBJECT: &str = "您的验证码: ";

pub const EMAIL_SUCCESS: &str = "验证码发送成功!";

pub fn email_html(code: &str) -> String {
    format!(
        r#"
        <p>你的验证码是：</p>
        <h1>{code}</h1>
        <p>验证码有效期为 5 分钟，请勿泄露。</p>
        "#
    )
}
pub fn email_text(code: &str) -> String {
    format!("您的验证码是：{code}，有效期为 5 分钟，请勿泄露。")
}
