use mockall::automock;

use crate::domain::{error::DomainError, template::Template};

#[automock]
pub trait TmuxApiPort {
    fn open(&self, template: Template) -> Result<(), DomainError>;
}

pub struct TmuxApi;

impl TmuxApi {
    pub fn new() -> Self {
        Self {}
    }
}

impl TmuxApiPort for TmuxApi {
    fn open(&self, template: Template) -> Result<(), DomainError> {
        println!("Opening template: {:?}", template);
        Ok(())
    }
}
