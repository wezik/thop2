use crate::domain::{
    error::DomainError, template, template_selector::TemplateSelector,
    template_service::TemplateService,
};

pub struct ThopService {
    pub template_service: TemplateService,
    // pub template_selector: Box<dyn TemplateSelector>,
}

impl ThopService {
    pub fn new(
        template_service: TemplateService,
        // template_selector: Box<dyn TemplateSelector>,
    ) -> ThopService {
        ThopService {
            template_service: template_service,
            // template_selector: template_selector,
        }
    }

    pub fn open(&self, path: template::Path) -> Result<(), DomainError> {
        let template = self.template_service.get(path)?;
        println!("Template: {:?}", template);
        return Ok(());
    }
}
