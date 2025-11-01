use std::rc::Rc;

use crate::{
    domain::{
        error::DomainError,
        multiplexer_gateway::MultiplexerGateway,
        template::{self, Template},
    },
    multiplexer::tmux::app::tmux_api::TmuxApiPort,
};

pub struct MultiMultiplexerGateway<TA: TmuxApiPort> {
    tmux_api: Rc<TA>,
}

impl<TA: TmuxApiPort> MultiMultiplexerGateway<TA> {
    pub fn new(tmux_api: Rc<TA>) -> Self {
        Self { tmux_api }
    }
}

impl<TA: TmuxApiPort> MultiplexerGateway for MultiMultiplexerGateway<TA> {
    fn open(&self, template: Template) -> Result<(), DomainError> {
        if template.engine == template::Engine::Tmux {
            return self.tmux_api.open(template);
        }
        Ok(())
    }
}
