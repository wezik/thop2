use crate::domain::error::{self, DomainError, DomainErrorTemplate};

pub trait TmuxClient {
    fn list(&self) -> Result<Vec<String>, DomainError>;
}

pub const TMUX_CLIENT_ERROR: DomainErrorTemplate = error::new(
    "TMUX_CLIENT_ERROR",
    "Error while interacting with tmux-server",
);
