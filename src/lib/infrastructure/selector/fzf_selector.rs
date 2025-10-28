use std::sync::Arc;

use crate::domain::{
    environment::{Environment, RunResult, ENVIRONMENT_RUN_COMMAND_ERROR},
    error::DomainError,
    selector::Selector,
};

pub struct FzfSelector {
    environment: Arc<dyn Environment>,
}

impl FzfSelector {
    pub fn new(environment: Arc<dyn Environment>) -> FzfSelector {
        FzfSelector {
            environment: environment,
        }
    }
}

impl Selector for FzfSelector {
    fn select_from(&self, entries: &[&str]) -> Result<Option<String>, DomainError> {
        if entries.is_empty() {
            return Ok(None);
        }

        let input = entries.join("\n");

        let run_result = self
            .environment
            .run_command("fzf", &[], Some(input.as_str()))?;

        match run_result {
            RunResult::Success(output) => Ok(output.strip_suffix("\n").map(|s| s.to_string())),
            RunResult::Failure(130) => Ok(None),
            RunResult::Failure(exit_code) => {
                Err(ENVIRONMENT_RUN_COMMAND_ERROR.with_attr("exitCode", exit_code.to_string()))
            }
        }
    }
}
