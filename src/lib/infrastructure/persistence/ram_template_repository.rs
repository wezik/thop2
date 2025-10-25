use std::collections::HashMap;

use crate::domain::{
    error::DomainError,
    template::{Engine, Name, Path, Template, TEMPLATE_NOT_FOUND},
    template_repository::TemplateRepository,
};

pub struct RamTemplateRepository {
    templates: HashMap<Path, Template>,
}

impl TemplateRepository for RamTemplateRepository {
    fn find(&self, path: Path) -> Result<Template, DomainError> {
        match self.templates.get(&path) {
            Some(t) => Ok(t.clone()),
            None => Err(TEMPLATE_NOT_FOUND.with_attr("path", path.0)),
        }
    }

    fn list(&self) -> Vec<Template> {
        self.templates.values().cloned().collect()
    }
}

impl RamTemplateRepository {
    pub fn new() -> RamTemplateRepository {
        let map = preload_templates()
            .into_iter()
            .map(|t| (t.path.clone(), t))
            .collect();
        RamTemplateRepository { templates: map }
    }
}

fn preload_templates() -> Vec<Template> {
    vec![
        Template {
            path: Path("resources/template.yaml".to_string()),
            name: Name("FooBar".to_string()),
            engine: Engine::Command,
        },
        Template {
            path: Path("resources/template-2.yaml".to_string()),
            name: Name("BarFoo".to_string()),
            engine: Engine::Command,
        },
    ]
}
