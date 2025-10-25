use std::sync::Arc;

use crate::domain::{environment::Environment, template::Template, template_selector::{SelectorResult, TemplateSelector}};

pub struct FzfSelector {
    environment: Arc<dyn Environment>,
} 

impl TemplateSelector for FzfSelector {
    fn select_from(&self, templates: Vec<Template>) -> SelectorResult {
        // TODO: Unimplemented
        if let Some(template) = templates.first() {
            return SelectorResult::Ok(template.clone());
        } else {
            return SelectorResult::Cancel;
        }
    }
}

impl FzfSelector {
    pub fn new(environment: Arc<dyn Environment>) -> FzfSelector {
        FzfSelector {
            environment: environment,
        }
    }
}
