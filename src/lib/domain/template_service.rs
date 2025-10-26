use std::sync::{Arc, Mutex};

use mockall::automock;

use crate::domain::error::DomainError;
use crate::domain::template::{Path, Template};
use crate::domain::template_repository::TemplateRepository;

#[automock]
pub trait TemplateServicePort {
    fn create(&self, template: Template) -> Result<(), DomainError>;
    fn get(&self, path: Path) -> Result<Template, DomainError>;
    fn list(&self) -> Vec<Template>;
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
        let mut repo = self.repository.lock().unwrap();
        repo.create(template)
    }

    fn get(&self, path: Path) -> Result<Template, DomainError> {
        let repo = self.repository.lock().unwrap();
        repo.find(path)
    }

    fn list(&self) -> Vec<Template> {
        let repo = self.repository.lock().unwrap();
        repo.list()
    }
}
