use std::{
    io::Write,
    process::{Command, Stdio},
};

use crate::domain::{
    environment::{Environment, RunResult, ENVIRONMENT_READ_ERROR, ENVIRONMENT_RUN_COMMAND_ERROR},
    error::DomainError,
};

pub struct SystemEnvironment {}

impl SystemEnvironment {
    pub fn new() -> SystemEnvironment {
        SystemEnvironment {}
    }
}

impl Environment for SystemEnvironment {
    fn current_dir(&self) -> Result<String, DomainError> {
        std::env::current_dir()
            .map_err(|e| ENVIRONMENT_READ_ERROR.with_attr("error", e.to_string()))?
            .to_str()
            .map(|s| s.to_string())
            .ok_or_else(|| ENVIRONMENT_READ_ERROR.with_attr("error", "path is not valid UTF-8"))
    }

    fn run_command<'a>(
        &self,
        command: &str,
        args: &[&'a str],
        input: Option<&'a str>,
    ) -> Result<RunResult, DomainError> {
        let mut child = Command::new(command)
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .map_err(|e| ENVIRONMENT_RUN_COMMAND_ERROR.with_attr("error", e.to_string()))?;

        if let Some(input) = input {
            child
                .stdin
                .as_mut()
                .ok_or_else(|| {
                    ENVIRONMENT_RUN_COMMAND_ERROR.with_attr("error", "Failed to open stdin")
                })?
                .write_all(input.as_bytes())
                .map_err(|e| ENVIRONMENT_RUN_COMMAND_ERROR.with_attr("error", e.to_string()))?;
        }

        let output = child
            .wait_with_output()
            .map_err(|e| ENVIRONMENT_RUN_COMMAND_ERROR.with_attr("error", e.to_string()))?;

        if !output.status.success() {
            let code = output.status.code().unwrap_or(-1);
            return Ok(RunResult::Failure(code));
        }

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        Ok(RunResult::Success(stdout))
    }

    fn get_config_value(&self, key: &str) -> Result<String, DomainError> {
        std::env::var(key).map_err(|e| {
            ENVIRONMENT_READ_ERROR.with_attr("error", e.to_string())
        })
    }
}
