use std::collections::HashMap;

use crate::domain::{error::DomainError, template::{models::{Name, Path, Template, TEMPLATE_NOT_FOUND}, ports::TemplateRepository}};

pub struct RamTemplateRepository {
    templates: HashMap<Path, Template>,
}

pub fn preload_templates() -> Vec<Template> {
    vec![
        Template {
            path: Path("resources/template.yaml".to_string()),
            name: Name("FooBar".to_string()),
        },
        Template {
            path: Path("resources/template-2.yaml".to_string()),
            name: Name("BarFoo".to_string()),
        },
    ]
}

pub fn new() -> RamTemplateRepository {
    let map = preload_templates().into_iter().map(|t| (t.path.clone(), t)).collect();
    RamTemplateRepository {
        templates: map,
    }
}

impl TemplateRepository for RamTemplateRepository {
    fn get_template(&self, path: Path) -> Result<Template, DomainError> {
        match self.templates.get(&path) {
            Some(t) => Ok(t.clone()),
            None => Err(TEMPLATE_NOT_FOUND.with_attr("path", path.0)),
        }
    }
}
