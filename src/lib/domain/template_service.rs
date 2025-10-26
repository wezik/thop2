use std::sync::{Arc, Mutex};

use mockall::automock;

use crate::domain::error::{DomainError, POISONED_LOCK_ERROR};
use crate::domain::template::{Path, Template};
use crate::domain::template_repository::TemplateRepository;

#[automock]
pub trait TemplateServicePort {
    fn create(&self, template: Template) -> Result<(), DomainError>;
    fn get(&self, path: Path) -> Result<Template, DomainError>;
    fn list(&self) -> Result<Vec<Template>, DomainError>;
}

pub struct TemplateService {
    repository: Arc<Mutex<dyn TemplateRepository>>,
}

impl TemplateService {
    pub fn new(repository: Arc<Mutex<dyn TemplateRepository>>) -> TemplateService {
        TemplateService {
            repository: repository,
        }
    }
}

impl TemplateServicePort for TemplateService {
    fn create(&self, template: Template) -> Result<(), DomainError> {
        let mut repo = match self.repository.lock() {
            Ok(repo) => repo,
            Err(err) => return Err(POISONED_LOCK_ERROR.with_attr("error", err.to_string())),
        };
        repo.create(template)
    }

    fn get(&self, path: Path) -> Result<Template, DomainError> {
        let repo = match self.repository.lock() {
            Ok(repo) => repo,
            Err(err) => return Err(POISONED_LOCK_ERROR.with_attr("error", err.to_string())),
        };
        repo.find(path)
    }

    fn list(&self) -> Result<Vec<Template>, DomainError> {
        let repo = match self.repository.lock() {
            Ok(repo) => repo,
            Err(err) => return Err(POISONED_LOCK_ERROR.with_attr("error", err.to_string())),
        };
        repo.list()
    }
}
