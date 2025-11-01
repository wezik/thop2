use std::rc::Rc;

use mockall::automock;

use crate::domain::error::DomainError;
use crate::domain::template::{Path, Template};
use crate::domain::template_repository::TemplateRepository;

#[automock]
pub trait TemplateServicePort {
    fn create(&self, template: Template) -> Result<(), DomainError>;
    fn delete(&self, path: Path) -> Result<(), DomainError>;
    fn get(&self, path: Path) -> Result<Template, DomainError>;
    fn list(&self) -> Result<Vec<Template>, DomainError>;
}

pub struct TemplateService<R: TemplateRepository> {
    repository: Rc<R>,
}

impl<R: TemplateRepository> TemplateService<R> {
    pub fn new(repository: Rc<R>) -> Self {
        Self { repository }
    }
}

impl<R: TemplateRepository> TemplateServicePort for TemplateService<R> {
    fn create(&self, template: Template) -> Result<(), DomainError> {
        self.repository.create(template)
    }

    fn delete(&self, path: Path) -> Result<(), DomainError> {
        self.repository.delete(path)
    }

    fn get(&self, path: Path) -> Result<Template, DomainError> {
        self.repository.find(path)
    }

    fn list(&self) -> Result<Vec<Template>, DomainError> {
        self.repository.list()
    }
}
