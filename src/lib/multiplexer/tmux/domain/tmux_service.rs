use std::rc::Rc;

use mockall::automock;

use crate::{
    domain::error::DomainError,
    multiplexer::tmux::domain::{tmux_client::TmuxClient, tmux_template::TmuxTemplate},
};

#[automock]
pub trait TmuxServicePort {
    fn open(&self, template: TmuxTemplate) -> Result<(), DomainError>;
}

pub struct TmuxService<C: TmuxClient> {
    client: Rc<C>,
}

impl<C: TmuxClient> TmuxService<C> {
    pub fn new(client: Rc<C>) -> Self {
        Self { client }
    }
}

impl<C: TmuxClient> TmuxServicePort for TmuxService<C> {
    fn open(&self, template: TmuxTemplate) -> Result<(), DomainError> {
        let active_sessions = self.client.list()?;
        // build session if it doesn't exist
        if !active_sessions.contains(&template.session.name) {
            // TODO: Create session
        }

        if self.client.is_in_session() {
            println!("Inside tmux");
            // TODO: Switch
        } else {
            println!("Outside tmux");
            // TODO: Attach
        }

        Ok(())
    }
}
