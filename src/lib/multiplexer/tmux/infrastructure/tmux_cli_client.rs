use crate::domain::environment::Environment;
use crate::domain::error::DomainError;
use crate::multiplexer::tmux::domain::tmux_client::{TmuxClient, TMUX_CLIENT_ERROR};
use std::rc::Rc;

pub struct TmuxCliClient<E: Environment> {
    environment: Rc<E>,
}

impl<E: Environment> TmuxCliClient<E> {
    pub fn new(environment: Rc<E>) -> Self {
        Self { environment }
    }
}

impl<E: Environment> TmuxClient for TmuxCliClient<E> {
    fn list(&self) -> Result<Vec<String>, DomainError> {
        let output = self
            .environment
            .run_command("tmux", &["list-sessions", "-F", "#{session_name}"], None)
            .map_err(|e| TMUX_CLIENT_ERROR.with_attr("error", e.to_string()))?
            .map_failure(|exit_code| {
                TMUX_CLIENT_ERROR.with_attr("exitCode", exit_code.to_string())
            })?;

        Ok(output
            .split("\n")
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect())
    }
}
