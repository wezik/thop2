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
        let result = std::env::current_dir();
        match result {
            Ok(path) => Ok(path.to_str().unwrap().to_string()),
            Err(e) => Err(ENVIRONMENT_READ_ERROR.with_attr("error", e.to_string())),
        }
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
            if let Some(mut stdin) = child.stdin.take() {
                stdin
                    .write_all(input.as_bytes())
                    .map_err(|e| ENVIRONMENT_RUN_COMMAND_ERROR.with_attr("error", e.to_string()))?;
            }
        }

        let output = child
            .wait_with_output()
            .map_err(|e| ENVIRONMENT_RUN_COMMAND_ERROR.with_attr("error", e.to_string()))?;

        if !output.status.success() {
            return Ok(RunResult::Failure(output.status.code().unwrap_or(-1)));
        }

        Ok(RunResult::Success(
            String::from_utf8_lossy(&output.stdout).to_string(),
        ))
    }
}
