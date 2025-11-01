// Tmux specific module for mapping and talking to tmux-server
pub mod app {
    pub mod bootstrap;
    pub mod tmux_api;
}

pub mod domain {
    pub mod tmux_service;
    pub mod tmux_template;
}
