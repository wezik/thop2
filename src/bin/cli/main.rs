use crab_hop::domain::template;

fn main() {
    let service = template::service::new();
    match service.get_template() {
        Ok(t) => println!("Template: {:?}", t),
        Err(e) => println!("Error: {}", e),
    }
}
