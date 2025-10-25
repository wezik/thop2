use crate::domain::error::{self, DomainErrorTemplate};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Template {
    pub path: Path,
    pub name: Name,
    pub engine: Engine,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Path(pub String);

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Name(pub String);

pub const TEMPLATE_NOT_FOUND: DomainErrorTemplate = error::new("TEMPLATE_NOT_FOUND", "Template not found");

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Engine {
    Command,
    // Tmux,
    // Zellij,
}

pub const ENGINE_NOT_SUPPORTED: DomainErrorTemplate = error::new("ENGINE_NOT_SUPPORTED", "Engine not supported");
