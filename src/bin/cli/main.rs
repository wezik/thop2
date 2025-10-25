use crab_hop::{
    domain::{template, template_service::TemplateService},
    infrastructure::persistence::ram_template_repository::RamTemplateRepository,
};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let repository = RamTemplateRepository::new();
    let service = TemplateService::new(Box::new(repository));

    if args.len() > 1 {
        let path = args[1].clone();
        match service.get(template::Path(path)) {
            Ok(t) => println!("Template: {:?}", t),
            Err(e) => println!("Error: {}", e),
        }
    }

    println!("Templates:");
    for t in service.list() {
        println!("{:?}", t);
    }
}
