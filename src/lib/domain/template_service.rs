use std::sync::{Arc, Mutex};

use mockall::automock;

use crate::domain::error::{DomainError, POISONED_LOCK_ERROR};
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
    repository: Arc<Mutex<R>>,
}

impl<R: TemplateRepository> TemplateService<R> {
    pub fn new(repository: Arc<Mutex<R>>) -> Self {
        Self { repository }
    }
}

impl<R: TemplateRepository> TemplateServicePort for TemplateService<R> {
    fn create(&self, template: Template) -> Result<(), DomainError> {
        let mut repo = self
            .repository
            .lock()
            .map_err(|err| POISONED_LOCK_ERROR.with_attr("error", err.to_string()))?;
        repo.create(template)
    }

    fn delete(&self, path: Path) -> Result<(), DomainError> {
        let mut repo = self
            .repository
            .lock()
            .map_err(|err| POISONED_LOCK_ERROR.with_attr("error", err.to_string()))?;
        repo.delete(path)
    }

    fn get(&self, path: Path) -> Result<Template, DomainError> {
        let repo = self
            .repository
            .lock()
            .map_err(|err| POISONED_LOCK_ERROR.with_attr("error", err.to_string()))?;
        repo.find(path)
    }

    fn list(&self) -> Result<Vec<Template>, DomainError> {
        let repo = self
            .repository
            .lock()
            .map_err(|err| POISONED_LOCK_ERROR.with_attr("error", err.to_string()))?;
        repo.list()
    }
}
