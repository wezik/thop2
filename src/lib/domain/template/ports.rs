use mockall::automock;

use crate::domain::{error::DomainError, template::models::{Path, Template}};

#[automock]
pub trait TemplateRepository {
    fn find(&self, path: Path) -> Result<Template, DomainError>;
}
