use crate::domain::error::{self, DomainErrorTemplate};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Template {
    pub path: Path,
    pub name: Name,
    pub engine: Engine,
    pub commands: Commands,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Path(pub String);

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Name(pub String);

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Commands(pub Vec<String>);

pub const TEMPLATE_NOT_FOUND: DomainErrorTemplate =
    error::new("TEMPLATE_NOT_FOUND", "Template not found");

pub const TEMPLATE_ALREADY_EXSISTS: DomainErrorTemplate =
    error::new("TEMPLATE_ALREADY_EXISTS", "Template already exists");

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Engine {
    Command,
    // Tmux,
    // Zellij,
}

pub const ENGINE_NOT_SUPPORTED: DomainErrorTemplate =
    error::new("ENGINE_NOT_SUPPORTED", "Engine not supported");
