use mockall::automock;

use crate::domain::{
    error::DomainError,
    template::{Path, Template},
};

#[automock]
pub trait TemplateRepository {
    fn create(&mut self, template: Template) -> Result<(), DomainError>;
    fn find(&self, path: Path) -> Result<Template, DomainError>;
    fn list(&self) -> Result<Vec<Template>, DomainError>;
}
