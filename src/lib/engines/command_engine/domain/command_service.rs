use crate::{
    domain::error::DomainError, engines::command_engine::domain::command_template::CommandTemplate,
};

pub struct CommandService {}

impl CommandService {
    pub fn new() -> CommandService {
        CommandService {}
    }

    pub fn process(&self, template: CommandTemplate) -> Result<(), DomainError> {
        template
            .commands
            .0
            .iter()
            .for_each(|c| println!("Mock executing: {}", c));
        Ok(())
    }
}
