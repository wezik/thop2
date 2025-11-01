use std::rc::Rc;

use crate::{
    infrastructure::os::system_environment::SystemEnvironment,
    multiplexer::tmux::{
        app::tmux_api::TmuxApi, domain::tmux_service::TmuxService,
        infrastructure::tmux_cli_client::TmuxCliClient,
    },
};

pub type RealTmuxApi = TmuxApi<TmuxService<TmuxCliClient<SystemEnvironment>>>;

pub fn tmux_api(environment: Rc<SystemEnvironment>) -> RealTmuxApi {
    let tmux_client_rc = Rc::new(TmuxCliClient::new(environment));
    let tmux_service_rc = Rc::new(TmuxService::new(tmux_client_rc));
    TmuxApi::new(tmux_service_rc)
}
