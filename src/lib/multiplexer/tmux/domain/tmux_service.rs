use crate::{domain::error::DomainError, multiplexer::tmux::domain::tmux_template::TmuxTemplate};

pub trait TmuxServicePort {
    fn open(&self, template: TmuxTemplate) -> Result<(), DomainError>;
}

pub struct TmuxService;

impl TmuxService {
    pub fn new() -> Self {
        Self
    }
}

impl TmuxServicePort for TmuxService {
    fn open(&self, template: TmuxTemplate) -> Result<(), DomainError> {
        println!("Opening tmux template: {:?}", template);
        Ok(())
    }
}
