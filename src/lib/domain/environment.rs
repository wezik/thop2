use mockall::automock;

use crate::domain::error::{self, DomainError, DomainErrorTemplate};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum RunResult {
    Success(String),
    Failure(i32),
}

impl RunResult {
    pub fn map_failure<F, O: FnOnce(i32) -> F>(self, op: O) -> Result<String, F> {
        match self {
            RunResult::Success(result) => Ok(result),
            RunResult::Failure(code) => Err(op(code)),
        }
    }
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
    fn get_config_value(&self, key: &str) -> Result<String, DomainError>;
}

pub const ENVIRONMENT_READ_ERROR: DomainErrorTemplate =
    error::new("ENVIRONMENT_READ_ERROR", "Failed to read from environment");

pub const ENVIRONMENT_RUN_COMMAND_ERROR: DomainErrorTemplate = error::new(
    "ENVIRONMENT_RUN_COMMAND_ERROR",
    "Failed to run command in environment",
);
