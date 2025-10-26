use mockall::automock;

use crate::domain::error::{self, DomainError, DomainErrorTemplate};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum RunResult {
    Success(String),
    Failure(i32),
}

#[automock]
pub trait Environment {
    fn current_dir(&self) -> Result<String, DomainError>;
    fn run_command<'a>(
        &self,
        command: &str,
        args: &[&'a str],
        input: Option<&'a str>,
    ) -> Result<RunResult, DomainError>;
}

pub const ENVIRONMENT_READ_ERROR: DomainErrorTemplate =
    error::new("ENVIRONMENT_READ_ERROR", "Failed to read from environment");

pub const ENVIRONMENT_RUN_COMMAND_ERROR: DomainErrorTemplate = error::new(
    "ENVIRONMENT_RUN_COMMAND_ERROR",
    "Failed to run command in environment",
);
