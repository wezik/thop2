use crate::domain::error::{self, DomainError, DomainErrorTemplate};

pub trait Selector {
    fn select_from(&self, templates: &[&str]) -> Result<Option<String>, DomainError>;
}

pub const SELECTION_FAILED: DomainErrorTemplate = error::new(
    "SELECTOR_SELECTION_FAILED",
    "Selector failed to select a template",
);
