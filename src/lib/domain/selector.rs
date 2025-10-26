use mockall::automock;

use crate::domain::error::{self, DomainError, DomainErrorTemplate};

#[automock]
pub trait Selector {
    fn select_from<'a>(&self, templates: &[&'a str]) -> Result<Option<String>, DomainError>;
}

pub const SELECTION_FAILED: DomainErrorTemplate = error::new(
    "SELECTOR_SELECTION_FAILED",
    "Selector failed to select a template",
);
