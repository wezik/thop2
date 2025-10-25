use crab_hop::{domain::template, infrastructure::persistence};

fn main() {

    let args: Vec<String> = std::env::args().collect();

    let repository = persistence::ram_template_repository::new();
    let service = template::service::new(Box::new(repository));

    if args.len() > 1 {
        let path = args[1].clone();
        match service.get(template::models::Path(path)) {
            Ok(t) => println!("Template: {:?}", t),
            Err(e) => println!("Error: {}", e),
        }
    }
}
