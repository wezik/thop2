use crate::domain::{error::DomainError, template::models::{Template, TEMPLATE_NOT_FOUND}};

pub struct Service {}

pub fn new() -> Service {
    Service {}
}

impl Service {
    pub fn get_template(&self) -> Result<Template, DomainError>{
        Err(TEMPLATE_NOT_FOUND.with_attr("template_name", "foo").build())
    }
}
