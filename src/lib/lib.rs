// Core "upper" module containing non engine specific code
pub mod app {}

pub mod domain {

    pub mod template {
        pub mod models;
        pub mod ports;
        pub mod service;
    }

    pub mod error;
}

pub mod infrastructure {

    pub mod persistence {
        pub mod ram_template_repository;
    }
}

pub mod engines {
    pub mod command_engine;
    pub mod tmux_engine;
    pub mod zellij_engine;
}
