use crate::domain::error::DomainErrorTemplate;

#[derive(Debug, Clone)]
pub struct Template {
    pub path: Path,
    pub name: Name,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Path(pub String);

#[derive(Debug, Clone)]
pub struct Name(pub String);

pub const TEMPLATE_NOT_FOUND: DomainErrorTemplate = DomainErrorTemplate::new("TEMPLATE_NOT_FOUND", "Template not found");
