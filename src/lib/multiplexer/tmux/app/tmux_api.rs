use std::rc::Rc;

use mockall::automock;

use crate::{
    domain::{error::DomainError, template::Template},
    multiplexer::tmux::domain::tmux_service::TmuxServicePort,
};

#[automock]
pub trait TmuxApiPort {
    fn open(&self, template: Template) -> Result<(), DomainError>;
}

pub struct TmuxApi<S: TmuxServicePort> {
    tmux_service: Rc<S>,
}

impl<S: TmuxServicePort> TmuxApi<S> {
    pub fn new(tmux_service: Rc<S>) -> Self {
        Self { tmux_service }
    }
}

impl<S: TmuxServicePort> TmuxApiPort for TmuxApi<S> {
    fn open(&self, template: Template) -> Result<(), DomainError> {
        self.tmux_service.open(template.try_into()?)
    }
}
