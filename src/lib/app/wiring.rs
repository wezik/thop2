use std::sync::{Arc, Mutex};

use crate::{
    domain::{
        template_service::TemplateService,
        thop_service::{ThopService, ThopServicePort},
    },
    engines::command_engine::{
        app::command_engine::CommandEngine, domain::command_service::CommandService,
    },
    infrastructure::{
        os::system_environment::SystemEnvironment,
        persistence::ram_template_repository::RamTemplateRepository,
        selector::fzf_selector::FzfSelector,
    },
};

pub fn thop_service() -> Arc<dyn ThopServicePort> {
    let command_engine = Arc::new(CommandEngine::new(CommandService::new()));
    let environment = Arc::new(SystemEnvironment::new());
    let template_selector = Arc::new(FzfSelector::new(environment.clone()));

    let template_repository = Arc::new(Mutex::new(RamTemplateRepository::new()));
    let template_service = Arc::new(TemplateService::new(template_repository.clone()));

    Arc::new(ThopService::new(
        template_service,
        command_engine,
        environment.clone(),
        template_selector,
    ))
}
