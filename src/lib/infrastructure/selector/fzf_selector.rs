use std::sync::Arc;

use crate::domain::{
    environment::{Environment, RunResult, ENVIRONMENT_RUN_COMMAND_ERROR},
    error::DomainError,
    selector::Selector,
    template::Template,
};

pub struct FzfSelector<E: Environment> {
    environment: Arc<E>,
}

impl<E: Environment> FzfSelector<E> {
    pub fn new(environment: Arc<E>) -> FzfSelector<E> {
        FzfSelector {
            environment: environment,
        }
    }
}

impl<E: Environment> Selector for FzfSelector<E> {
    fn select_template(&self, templates: &[Template]) -> Result<Option<Template>, DomainError> {
        let entries = templates
            .iter()
            .map(|template| template.name.0.as_str())
            .collect::<Vec<&str>>();
        let selection = self.select_from(&entries)?;
        match selection {
            Some(selection) => Ok(templates
                .iter()
                .find(|template| template.name.0 == selection)
                .map(|template| template.to_owned())),
            None => Ok(None),
        }
    }
}

impl<E: Environment> FzfSelector<E> {
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
