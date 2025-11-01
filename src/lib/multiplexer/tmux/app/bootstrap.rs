use std::rc::Rc;

use crate::multiplexer::tmux::{app::tmux_api::TmuxApi, domain::tmux_service::TmuxService};

pub type RealTmuxApi = TmuxApi<TmuxService>;

pub fn tmux_api() -> RealTmuxApi {
    let tmux_service_rc = Rc::new(TmuxService::new());
    TmuxApi::new(tmux_service_rc)
}
