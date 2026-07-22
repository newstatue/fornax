use std::collections::HashMap;
use garde::Validate;
use crate::common::i18n;

pub fn report_to_map(
    report: garde::Report,
) -> HashMap<String, Vec<String>> {
    let mut errors = HashMap::<String, Vec<String>>::new();

    for (path, error) in report.into_inner() {
        let field = if path.is_empty() {
            "_root".to_string()
        } else {
            path.to_string()
        };

        errors
            .entry(field)
            .or_default()
            .push(error.message().to_string());
    }

    errors
}

pub fn validate<T>(
    request: &T,
) -> Result<(), garde::Report>
where
    T: Validate,
    T::Context: Default,
{
    garde::with_i18n(i18n::Garde, || request.validate())
}