use crate::{
    domain::{error::DomainError, template, template_service::TemplateService},
    engines::command_engine::inbound::command_engine::CommandEngine,
};

pub struct ThopService {
    pub template_service: TemplateService,
    pub command_engine: CommandEngine,
    // pub template_selector: Box<dyn TemplateSelector>,
}

impl ThopService {
    pub fn new(
        template_service: TemplateService,
        command_engine: CommandEngine,
        // template_selector: Box<dyn TemplateSelector>,
    ) -> ThopService {
        ThopService {
            template_service: template_service,
            command_engine: command_engine,
            // template_selector: template_selector,
        }
    }

    pub fn open(&self, path: template::Path) -> Result<(), DomainError> {
        let template = self.template_service.get(path)?;
        match template.engine {
            template::Engine::Command => self.command_engine.process(template)?,
        }

        Ok(())
    }
}
