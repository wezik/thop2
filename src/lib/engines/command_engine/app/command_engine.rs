use std::sync::Arc;

use mockall::automock;

use crate::{
    domain::{error::DomainError, template::Template},
    engines::command_engine::domain::{
        command_service::CommandServicePort, command_template::CommandTemplate,
    },
};

#[automock]
pub trait CommandEnginePort {
    fn process(&self, template: Template) -> Result<(), DomainError>;
}

pub struct CommandEngine<CS: CommandServicePort> {
    command_service: Arc<CS>,
}

impl<CS: CommandServicePort> CommandEngine<CS> {
    pub fn new(command_service: Arc<CS>) -> CommandEngine<CS> {
        CommandEngine { command_service }
    }
}

impl<CS: CommandServicePort> CommandEnginePort for CommandEngine<CS> {
    fn process(&self, template: Template) -> Result<(), DomainError> {
        let command_template = CommandTemplate {
            commands: template.commands,
        };
        self.command_service.process(command_template)?;
        Ok(())
    }
}
