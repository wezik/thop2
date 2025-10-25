use crate::domain::{error::DomainError, template::models::{Path, Template}};

pub trait TemplateRepository {
    fn get_template(&self, path: Path) -> Result<Template, DomainError>;
}
