use crate::domain::{error::DomainError, template::Template};

pub enum SelectorResult {
    Ok(Template),
    Cancel,
    Error(DomainError),
}

pub trait TemplateSelector {
    fn select_from(&self, templates: Vec<Template>) -> SelectorResult;
}
