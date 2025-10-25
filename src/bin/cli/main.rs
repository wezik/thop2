use crab_hop::{
    domain::{template, template_service::TemplateService, thop_service::ThopService},
    infrastructure::persistence::ram_template_repository::RamTemplateRepository,
};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let template_repository = RamTemplateRepository::new();
    let template_service = TemplateService::new(Box::new(template_repository));
    let thop_service = ThopService::new(template_service);

    if args.len() > 1 {
        let path = args[1].clone();
        match thop_service.open(template::Path(path)) {
            Ok(_) => println!("Success"),
            Err(e) => println!("Error: {}", e),
        }
    }
}
