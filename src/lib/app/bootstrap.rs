use std::sync::{Arc, Mutex};

use crate::{
    domain::{template_service::TemplateService, thop_service::ThopService},
    infrastructure::{
        multiplexer::multi_multiplexer_gateway::MultiMultiplexerGateway,
        os::system_environment::SystemEnvironment,
        persistence::ram_template_repository::RamTemplateRepository,
        selector::fzf_selector::FzfSelector,
    },
    multiplexer::tmux::{app::tmux_api::TmuxApi, domain::tmux_service::TmuxService},
};

pub type RealThopService = ThopService<
    TemplateService<RamTemplateRepository>,
    FzfSelector<SystemEnvironment>,
    MultiMultiplexerGateway<RealTmuxApi>,
    SystemEnvironment,
>;

pub fn thop_service() -> RealThopService {
    // multiplexers
    let multi_multiplexer_gateway_arc = multiplexer_gateway();

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
        multi_multiplexer_gateway_arc,
        environment_arc,
    )
}

type RealTmuxApi = TmuxApi<TmuxService>;

fn multiplexer_gateway() -> Arc<MultiMultiplexerGateway<RealTmuxApi>> {
    let tmux_service_arc = Arc::new(TmuxService::new());
    let tmux_api_arc = Arc::new(TmuxApi::new(tmux_service_arc));
    Arc::new(MultiMultiplexerGateway::new(tmux_api_arc))
}
