use std::sync::{Arc, Mutex};

use crate::{
    domain::{template_service::TemplateService, thop_service::ThopService},
    infrastructure::{
        os::system_environment::SystemEnvironment,
        persistence::ram_template_repository::RamTemplateRepository,
        selector::fzf_selector::FzfSelector,
    },
    multiplexer::command::{
        app::command_engine::CommandEngine, domain::command_service::CommandService,
    },
};

pub type RealThopService = ThopService<
    TemplateService<RamTemplateRepository>,
    FzfSelector<SystemEnvironment>,
    RealCommandEngine,
    SystemEnvironment,
>;

pub fn thop_service() -> RealThopService {
    // multiplexers
    let command_engine_arc = Arc::new(command_engine());

    // environment
    let environment_arc = Arc::new(SystemEnvironment::new());

    // selector
    let selector_arc = Arc::new(FzfSelector::new(environment_arc.clone()));

    // template domain
    let template_repository_arc = Arc::new(Mutex::new(RamTemplateRepository::new()));
    let template_service_arc = Arc::new(TemplateService::new(template_repository_arc));

    // thop domain
    ThopService::new(
        template_service_arc,
        selector_arc,
        command_engine_arc,
        environment_arc,
    )
}

type RealCommandEngine = CommandEngine<CommandService>;

fn command_engine() -> RealCommandEngine {
    let command_service_arc = Arc::new(CommandService::new());
    CommandEngine::new(command_service_arc)
}
