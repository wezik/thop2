use std::{cell::RefCell, collections::HashMap};

use crate::domain::{
    error::DomainError,
    template::{
        Commands, Engine, Name, Path, Template, TEMPLATE_ALREADY_EXSISTS, TEMPLATE_NOT_FOUND,
    },
    template_repository::TemplateRepository,
};

pub struct RamTemplateRepository {
    templates: RefCell<HashMap<Path, Template>>,
}

impl TemplateRepository for RamTemplateRepository {
    fn create(&self, template: Template) -> Result<(), DomainError> {
        let mut templates = self.templates.borrow_mut();
        match templates.get(&template.path) {
            Some(_) => return Err(TEMPLATE_ALREADY_EXSISTS.with_attr("path", template.path.0)),
            None => templates.insert(template.path.clone(), template),
        };

        Ok(())
    }

    fn delete(&self, path: Path) -> Result<(), DomainError> {
        match self.templates.borrow_mut().remove(&path) {
            Some(_) => Ok(()),
            None => Err(TEMPLATE_NOT_FOUND.with_attr("path", path.0)),
        }
    }

    fn find(&self, path: Path) -> Result<Template, DomainError> {
        match self.templates.borrow().get(&path) {
            Some(t) => Ok(t.clone()),
            None => Err(TEMPLATE_NOT_FOUND.with_attr("path", path.0)),
        }
    }

    fn list(&self) -> Result<Vec<Template>, DomainError> {
        Ok(self.templates.borrow().values().cloned().collect())
    }
}

impl RamTemplateRepository {
    pub fn new() -> RamTemplateRepository {
        let map = preload_templates()
            .into_iter()
            .map(|t| (t.path.clone(), t))
            .collect();
        RamTemplateRepository {
            templates: RefCell::new(map),
        }
    }

    pub fn clear(&self) {
        self.templates.borrow_mut().clear();
    }
}

fn preload_templates() -> Vec<Template> {
    vec![
        Template {
            path: Path("resources/template.yaml".to_string()),
            name: Name("FooBar".to_string()),
            engine: Engine::Tmux,
            commands: Commands(vec![
                "echo 'Hello World from FooBar'".to_string(),
                "echo 'Goodbye World from FooBar'".to_string(),
            ]),
        },
        Template {
            path: Path("resources/template-2.yaml".to_string()),
            name: Name("BarFoo".to_string()),
            engine: Engine::Tmux,
            commands: Commands(vec![
                "echo 'Hello World from BarFoo'".to_string(),
                "echo 'Goodbye World from BarFoo'".to_string(),
            ]),
        },
    ]
}
