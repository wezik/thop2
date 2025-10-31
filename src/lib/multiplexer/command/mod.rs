// Command engine specific module which essentially functions as a simple command runner
// It is likely I will delete this module and mantain only mutliplexers as it seems like an
// entirely different thing not fitting into this project
pub mod app {
    pub mod command_engine;
}
pub mod domain {
    pub mod command_service;
    pub mod command_template;
}
