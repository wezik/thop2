use std::rc::Rc;

use crate::{domain::environment::Environment, multiplexer::tmux::domain::tmux_client::TmuxClient};

pub struct TmuxCliClient<E: Environment> {
    environment: Rc<E>,
}

impl<E: Environment> TmuxCliClient<E> {
    pub fn new(environment: Rc<E>) -> Self {
        Self { environment }
    }
}

impl<E: Environment> TmuxClient for TmuxCliClient<E> {
    fn foo(&self) {
        println!("foo");
    }
}
