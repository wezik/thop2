use crate::domain::error::{self, DomainError, DomainErrorTemplate};

pub trait TmuxClient {
    fn list(&self) -> Result<Vec<String>, DomainError>;
    fn is_in_session(&self) -> bool;
}

pub const TMUX_CLIENT_ERROR: DomainErrorTemplate = error::new(
    "TMUX_CLIENT_ERROR",
    "Error while interacting with tmux-server",
);
