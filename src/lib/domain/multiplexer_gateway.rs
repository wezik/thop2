use mockall::automock;

use crate::domain::{error::DomainError, template::Template};

#[automock]
pub trait MultiplexerGateway {
    fn open(&self, template: Template) -> Result<(), DomainError>;
}
