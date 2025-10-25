use crate::domain::error::DomainErrorTemplate;

#[derive(Debug, Clone)]
pub struct Template {
    pub name: String,
}

pub const TEMPLATE_NOT_FOUND: DomainErrorTemplate = DomainErrorTemplate::new("TEMPLATE_NOT_FOUND", "Template not found");
