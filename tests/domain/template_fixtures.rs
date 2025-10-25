use crab_hop::domain::template::models::{Engine, Name, Path, Template};

pub fn some_template() -> Template {
    Template {
        path: Path("resources/template.yaml".to_string()),
        name: Name("FooBar".to_string()),
        engine: Engine::Command,
    }
}
