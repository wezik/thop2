use mockall::automock;

use crate::domain::error::{self, DomainError, DomainErrorTemplate};

#[automock]
pub trait Environment {
    fn current_dir(&self) -> Result<String, DomainError>;
}

pub const ENVIRONMENT_ERROR: DomainErrorTemplate = error::new("ENVIRONMENT_ERROR", "Failed to communicate with environment");
