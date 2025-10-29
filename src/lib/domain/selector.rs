use mockall::automock;

use crate::domain::{
    error::{self, DomainError, DomainErrorTemplate},
    template::Template,
};

#[automock]
pub trait Selector {
    fn select_template(&self, templates: &[Template]) -> Result<Option<Template>, DomainError>;
}

pub const SELECTION_FAILED: DomainErrorTemplate = error::new(
    "SELECTOR_SELECTION_FAILED",
    "Selector failed to select a template",
);
