use crab_hop::{
    domain::{template, template_service::TemplateService, thop_service::ThopService},
    engines::command_engine::{
        domain::command_service::CommandService, inbound::command_engine::CommandEngine,
    },
    outbound::persistence::ram_template_repository::RamTemplateRepository,
};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let command_engine = CommandEngine::new(CommandService::new());

    let template_repository = RamTemplateRepository::new();
    let template_service = TemplateService::new(Box::new(template_repository));
    let thop_service = ThopService::new(template_service, command_engine);

    if args.len() > 1 {
        let path = args[1].clone();
        match thop_service.open(template::Path(path)) {
            Ok(_) => println!("Success"),
            Err(e) => println!("Error: {}", e),
        }
    }
}
