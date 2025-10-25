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
    pub fn get_template(&self, path: Path) -> Result<Template, DomainError>{
        self.repository.get_template(path)
    }
}
