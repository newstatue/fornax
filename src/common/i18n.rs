use std::borrow::Cow;
use std::fmt::Display;

use garde::i18n::{
    I18n,
    InvalidCreditCard,
    InvalidEmail,
    InvalidPhoneNumber,
    InvalidUrl,
    IpKind,
};

#[derive(Debug, Clone, Copy)]
pub struct Garde;

impl I18n for Garde {
    fn length_lower_than(
        &self,
        min: usize,
    ) -> Cow<'static, str> {
        Cow::Owned(format!("长度不能少于 {min}"))
    }

    fn length_greater_than(
        &self,
        max: usize,
    ) -> Cow<'static, str> {
        Cow::Owned(format!("长度不能超过 {max}"))
    }

    fn range_lower_than(
        &self,
        min: &dyn Display,
    ) -> Cow<'static, str> {
        Cow::Owned(format!("值不能小于 {min}"))
    }

    fn range_greater_than(
        &self,
        max: &dyn Display,
    ) -> Cow<'static, str> {
        Cow::Owned(format!("值不能大于 {max}"))
    }

    fn credit_card_invalid(
        &self,
        _reason: InvalidCreditCard,
    ) -> Cow<'static, str> {
        Cow::Borrowed("信用卡号码格式不正确")
    }

    fn pattern_no_match(
        &self,
        pattern: &dyn Display,
    ) -> Cow<'static, str> {
        Cow::Owned(format!("内容不符合格式要求：{pattern}"))
    }

    fn contains_missing(
        &self,
        pattern: &dyn Display,
    ) -> Cow<'static, str> {
        Cow::Owned(format!("内容必须包含：{pattern}"))
    }

    fn url_invalid(
        &self,
        _reason: InvalidUrl,
    ) -> Cow<'static, str> {
        Cow::Borrowed("URL 格式不正确")
    }

    fn prefix_missing(
        &self,
        pattern: &dyn Display,
    ) -> Cow<'static, str> {
        Cow::Owned(format!("内容必须以 {pattern} 开头"))
    }

    fn suffix_missing(
        &self,
        pattern: &dyn Display,
    ) -> Cow<'static, str> {
        Cow::Owned(format!("内容必须以 {pattern} 结尾"))
    }

    fn phone_number_invalid(
        &self,
        _reason: InvalidPhoneNumber,
    ) -> Cow<'static, str> {
        Cow::Borrowed("手机号码格式不正确")
    }

    fn ip_invalid(
        &self,
        _kind: IpKind,
    ) -> Cow<'static, str> {
        Cow::Borrowed("IP 地址格式不正确")
    }

    fn matches_field_mismatch(
        &self,
        field: &dyn Display,
    ) -> Cow<'static, str> {
        Cow::Owned(format!("内容与字段 {field} 不一致"))
    }

    fn email_invalid(
        &self,
        _reason: InvalidEmail,
    ) -> Cow<'static, str> {
        Cow::Borrowed("邮箱格式不正确")
    }

    fn ascii_invalid(
        &self,
    ) -> Cow<'static, str> {
        Cow::Borrowed("只能包含 ASCII 字符")
    }

    fn alphanumeric_invalid(
        &self,
    ) -> Cow<'static, str> {
        Cow::Borrowed("只能包含字母和数字")
    }

    fn required_not_set(
        &self,
    ) -> Cow<'static, str> {
        Cow::Borrowed("此字段不能为空")
    }
}