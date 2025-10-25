use crate::domain::error::DomainError;
use crate::domain::template::{Path, Template};
use crate::domain::template_repository::TemplateRepository;

pub struct TemplateService {
    repository: Box<dyn TemplateRepository>,
}

impl TemplateService {
    pub fn new(repository: Box<dyn TemplateRepository>) -> TemplateService {
        TemplateService {
            repository: repository,
        }
    }

    pub fn get(&self, path: Path) -> Result<Template, DomainError> {
        self.repository.find(path)
    }

    pub fn list(&self) -> Vec<Template> {
        self.repository.list()
    }
}
