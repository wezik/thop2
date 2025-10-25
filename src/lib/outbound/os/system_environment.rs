use crate::domain::{environment::{Environment, ENVIRONMENT_ERROR}, error::DomainError};

pub struct SystemEnvironment {}

impl SystemEnvironment {
    pub fn new() -> SystemEnvironment {
        SystemEnvironment {}
    }
}

impl Environment for SystemEnvironment {
    fn current_dir(&self) -> Result<String, DomainError> {
        let result = std::env::current_dir();
        match result {
            Ok(path) => Ok(path.to_str().unwrap().to_string()),
            Err(e) => Err(ENVIRONMENT_ERROR.with_attr("error", e.to_string())),
        }
    }
}
