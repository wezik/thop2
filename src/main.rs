use std::{fs::File, io::Read};

use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Template {
    name: String,
    version: f32,
    #[serde(rename = "magicNumber")]
    magic_number: u8,
}

fn main() {
    let mut file = File::open("resources/template.yaml").expect("Failed to open template file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read template file");

    let template: Template = serde_yml::from_str(&contents).expect("Failed to parse template file");
    println!("{:?}", template);
}
