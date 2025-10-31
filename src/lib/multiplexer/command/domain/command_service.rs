use mockall::automock;

use crate::{
    domain::error::DomainError, multiplexer::command::domain::command_template::CommandTemplate,
};

#[automock]
pub trait CommandServicePort {
    fn process(&self, template: CommandTemplate) -> Result<(), DomainError>;
}

pub struct CommandService;

impl CommandService {
    pub fn new() -> CommandService {
        CommandService {}
    }
}

impl CommandServicePort for CommandService {
    fn process(&self, template: CommandTemplate) -> Result<(), DomainError> {
        template
            .commands
            .0
            .iter()
            .for_each(|c| println!("Mock executing: {}", c));
        Ok(())
    }
}
