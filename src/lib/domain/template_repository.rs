use mockall::automock;

use crate::domain::{
    error::DomainError,
    template::{Path, Template},
};

#[automock]
pub trait TemplateRepository {
    fn create(&self, template: Template) -> Result<(), DomainError>;
    fn delete(&self, path: Path) -> Result<(), DomainError>;
    fn find(&self, path: Path) -> Result<Template, DomainError>;
    fn list(&self) -> Result<Vec<Template>, DomainError>;
}
