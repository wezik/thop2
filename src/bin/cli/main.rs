use crab_hop::{
    domain::{error::DomainError, template, template_service::TemplateService, thop_service::{CreateCommand, OpenCommand, ThopService}},
    engines::command_engine::{
        domain::command_service::CommandService, inbound::command_engine::CommandEngine,
    },
    outbound::{os::system_environment::SystemEnvironment, persistence::ram_template_repository::RamTemplateRepository},
};

fn main() -> Result<(), DomainError> {
    let args: Vec<String> = std::env::args().collect();

    let command_engine = CommandEngine::new(CommandService::new());
    let environment = Box::new(SystemEnvironment::new());

    let template_repository = RamTemplateRepository::new();
    let template_service = TemplateService::new(Box::new(template_repository));

    let mut thop_service = ThopService::new(template_service, command_engine, environment);

    if args.len() > 2 {
        let command = args[1].as_str();
        match command {
            "create" => {
                let name = args[2].clone();
                let command = CreateCommand {
                    name: Some(template::Name(name)),
                    path: None,
                };

                return thop_service.create(command);
            }
            "open" => {
                let path = args[2].clone();
                let command = OpenCommand {
                    path: Some(template::Path(path)),
                };
                return thop_service.open(command);
            }
            _ => unimplemented!("Unknown command"),
        }
    }

    unimplemented!("Unknown command")
}
