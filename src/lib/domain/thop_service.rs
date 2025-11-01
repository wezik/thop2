use std::rc::Rc;

use mockall::automock;

use crate::domain::multiplexer_gateway::MultiplexerGateway;
use crate::domain::template_service::TemplateServicePort;
use crate::domain::{environment::Environment, error::DomainError, selector::Selector, template};

#[automock]
pub trait ThopServicePort {
    fn create(&self, command: CreateCommand) -> Result<(), DomainError>;
    fn delete(&self, command: DeleteCommand) -> Result<(), DomainError>;
    fn open(&self, command: OpenCommand) -> Result<(), DomainError>;
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct CreateCommand {
    pub name: Option<template::Name>,
    pub path: Option<template::Path>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct DeleteCommand {
    pub path: Option<template::Path>,
}
//
// #[derive(Debug, Clone, Hash, PartialEq, Eq)]
// pub struct KillCommand {
//     pub name: Option<template::Name>,
// }

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct OpenCommand {
    pub path: Option<template::Path>,
}

pub struct ThopService<TS, SE, MG, E>
where
    TS: TemplateServicePort,
    SE: Selector,
    MG: MultiplexerGateway,
    E: Environment,
{
    template_service: Rc<TS>,
    selector: Rc<SE>,
    multiplexer_gateway: Rc<MG>,
    environment: Rc<E>,
}

impl<TS, SE, MG, E> ThopService<TS, SE, MG, E>
where
    TS: TemplateServicePort,
    SE: Selector,
    MG: MultiplexerGateway,
    E: Environment,
{
    pub fn new(
        template_service: Rc<TS>,
        selector: Rc<SE>,
        multiplexer_gateway: Rc<MG>,
        environment: Rc<E>,
    ) -> Self {
        Self {
            template_service,
            multiplexer_gateway,
            environment,
            selector,
        }
    }
}

impl<TS, SE, MG, E> ThopServicePort for ThopService<TS, SE, MG, E>
where
    TS: TemplateServicePort,
    SE: Selector,
    MG: MultiplexerGateway,
    E: Environment,
{
    fn create(&self, command: CreateCommand) -> Result<(), DomainError> {
        let path = match command.path {
            Some(path) => path,
            None => template::Path(self.environment.current_dir()?),
        };

        let name = match command.name {
            Some(name) => name,
            None => template::Name(path.0.clone()),
        };

        let template = template::Template::new(path, name, template::Engine::Tmux);

        self.template_service.create(template)?;
        Ok(())
    }

    fn delete(&self, command: DeleteCommand) -> Result<(), DomainError> {
        if let Some(path) = command.path {
            self.template_service.delete(path)?;
            return Ok(());
        }
        let templates = self.template_service.list()?;
        match self.selector.select_template(&templates)? {
            Some(template) => self.template_service.delete(template.path),
            // noop if no template is selected
            None => Ok(()),
        }
    }

    fn open(&self, command: OpenCommand) -> Result<(), DomainError> {
        let template = match command.path {
            Some(path) => self.template_service.get(path)?,
            None => {
                let templates = self.template_service.list()?;
                match self.selector.select_template(&templates)? {
                    Some(template) => template.to_owned(),
                    // noop if no template is selected
                    None => return Ok(()),
                }
            }
        };

        return self.multiplexer_gateway.open(template);
    }
}
