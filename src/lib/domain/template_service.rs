use std::sync::{Arc, Mutex};

use crate::domain::error::DomainError;
use crate::domain::template::{Path, Template};
use crate::domain::template_repository::TemplateRepository;

pub struct TemplateService {
    repository: Arc<Mutex<dyn TemplateRepository>>,
}

impl TemplateService {
    pub fn new(repository: Arc<Mutex<dyn TemplateRepository>>) -> TemplateService {
        TemplateService {
            repository: repository,
        }
    }

    pub fn create(&self, template: Template) -> Result<(), DomainError> {
        let mut repo = self.repository.lock().unwrap();
        repo.create(template)
    }

    pub fn get(&self, path: Path) -> Result<Template, DomainError> {
        let repo = self.repository.lock().unwrap();
        repo.find(path)
    }

    pub fn list(&self) -> Vec<Template> {
        let repo = self.repository.lock().unwrap();
        repo.list()
    }
}
