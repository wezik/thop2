use std::rc::Rc;

use crate::{domain::error::DomainError, multiplexer::tmux::domain::{tmux_client::TmuxClient, tmux_template::TmuxTemplate}};

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
        self.client.foo();
        println!("Opening tmux template: {:?}", template);
        Ok(())
    }
}
