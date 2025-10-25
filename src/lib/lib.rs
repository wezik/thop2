// Core "upper" module containing non engine specific code
pub mod inbound {}

pub mod domain {

    pub mod environment;

    pub mod thop_service;

    pub mod template;
    pub mod template_repository;
    pub mod template_selector;
    pub mod template_service;

    pub mod error;
}

pub mod outbound {

    pub mod os {
        pub mod system_environment;
    }

    pub mod persistence {
        pub mod ram_template_repository;
    }

    pub mod selector {
        pub mod fzf_selector;
    }
}

pub mod engines {
    pub mod command_engine;
    pub mod tmux_engine;
    pub mod zellij_engine;
}
