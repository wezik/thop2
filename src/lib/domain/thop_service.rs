use std::sync::Arc;

use crate::{
    domain::{environment::Environment, error::DomainError, template, template_selector::TemplateSelector, template_service::TemplateService},
    engines::command_engine::inbound::command_engine::CommandEngine,
};

pub struct CreateCommand {
    pub name: Option<template::Name>,
    pub path: Option<template::Path>,
}
//
// pub struct DeleteCommand {
//     pub name: Option<template::Name>,
// }
//
// pub struct KillCommand {
//     pub name: Option<template::Name>,
// }

pub struct OpenCommand {
    pub path: Option<template::Path>,
}

pub struct ThopService {
    pub template_service: TemplateService,
    pub command_engine: CommandEngine,
    pub environment: Arc<dyn Environment>,
    pub template_selector: Arc<dyn TemplateSelector>,
}

impl ThopService {
    pub fn new(
        template_service: TemplateService,
        command_engine: CommandEngine,
        environment: Arc<dyn Environment>,
        template_selector: Arc<dyn TemplateSelector>,
    ) -> ThopService {
        ThopService {
            template_service: template_service,
            command_engine: command_engine,
            environment: environment,
            template_selector: template_selector,
        }
    }

    pub fn create(&mut self, command: CreateCommand) -> Result<(), DomainError> {
        let path = match command.path {
            Some(path) => path,
            None => template::Path(self.environment.current_dir()?),
        };

        let name = match command.name {
            Some(name) => name,
            None => template::Name(path.0.clone()),
        };

        let template = template::Template {
            path: path,
            name: name,
            engine: template::Engine::Command,
            commands: template::Commands(vec![]),
        };

        self.template_service.create(template)?;
        Ok(())
    }


    pub fn open(&self, command: OpenCommand) -> Result<(), DomainError> {
        let path = match command.path {
            Some(path) => path,
            None => unimplemented!("Open without path"), // TODO: Selector for templates
        };

        let template = self.template_service.get(path)?;
        match template.engine {
            template::Engine::Command => self.command_engine.process(template)?,
        }

        Ok(())
    }
}
