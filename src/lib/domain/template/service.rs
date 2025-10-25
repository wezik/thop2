use crate::domain::{error::DomainError, template::{models::{Path, Template}, ports::TemplateRepository}};

pub struct Service {
    repository: Box<dyn TemplateRepository>,
}

pub fn new(repository: Box<dyn TemplateRepository>) -> Service {
    Service { 
        repository: repository,
    }
}

impl Service {
    pub fn get(&self, path: Path) -> Result<Template, DomainError>{
        self.repository.find(path)
    }

    pub fn list(&self) -> Vec<Template> {
        self.repository.list()
    }
}
