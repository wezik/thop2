use mockall::automock;

use crate::{
    domain::{error::DomainError, template::Template},
    engines::command_engine::domain::{
        command_service::CommandService, command_template::CommandTemplate,
    },
};

#[automock]
pub trait CommandEnginePort {
    fn process(&self, template: Template) -> Result<(), DomainError>;
}

pub struct CommandEngine {
    pub command_service: CommandService,
}

impl CommandEngine {
    pub fn new(command_service: CommandService) -> CommandEngine {
        CommandEngine {
            command_service: command_service,
        }
    }
}

impl CommandEnginePort for CommandEngine {
    fn process(&self, template: Template) -> Result<(), DomainError> {
        let command_template = CommandTemplate {
            commands: template.commands,
        };
        self.command_service.process(command_template)?;
        Ok(())
    }
}
