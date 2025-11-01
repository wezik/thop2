use std::rc::Rc;

use crate::{
    domain::{template_service::TemplateService, thop_service::ThopService},
    infrastructure::{
        multiplexer::multi_multiplexer_gateway::MultiMultiplexerGateway,
        os::system_environment::SystemEnvironment,
        persistence::ram_template_repository::RamTemplateRepository,
        selector::fzf_selector::FzfSelector,
    },
    multiplexer::tmux::app::bootstrap::{tmux_api, RealTmuxApi},
};

pub type RealThopService = ThopService<
    TemplateService<RamTemplateRepository>,
    FzfSelector<SystemEnvironment>,
    MultiMultiplexerGateway<RealTmuxApi>,
    SystemEnvironment,
>;

pub fn thop_service() -> RealThopService {
    // multiplexers
    let multi_multiplexer_gateway_rc = multiplexer_gateway();

    // environment
    let environment_rc = Rc::new(SystemEnvironment::new());

    // selector
    let selector_rc = Rc::new(FzfSelector::new(environment_rc.clone()));

    // template domain
    let template_repository_rc = Rc::new(RamTemplateRepository::new());
    let template_service_rc = Rc::new(TemplateService::new(template_repository_rc));

    // thop domain
    ThopService::new(
        template_service_rc,
        selector_rc,
        multi_multiplexer_gateway_rc,
        environment_rc,
    )
}

fn multiplexer_gateway() -> Rc<MultiMultiplexerGateway<RealTmuxApi>> {
    let tmux_api_rc = Rc::new(tmux_api());
    Rc::new(MultiMultiplexerGateway::new(tmux_api_rc))
}
