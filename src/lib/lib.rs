// Core "upper" module containing non engine specific code
pub mod app {
    pub mod bootstrap;
}

pub mod domain {
    pub mod environment;
    pub mod error;
    pub mod multiplexer_gateway;
    pub mod selector;

    pub mod template;
    pub mod template_repository;
    pub mod template_service;

    pub mod thop_service;
}

pub mod infrastructure {
    pub mod multiplexer {
        pub mod multi_multiplexer_gateway;
    }

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

pub mod multiplexer {
    pub mod tmux;
    pub mod zellij;
}
