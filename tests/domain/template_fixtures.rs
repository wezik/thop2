use crab_hop::domain::template::{Commands, Engine, Name, Path, Template};

pub fn some_template() -> Template {
    Template {
        path: Path("resources/template.yaml".to_string()),
        name: Name("FooBar".to_string()),
        engine: Engine::Command,
        commands: Commands(vec![
            "echo 'Hello World'".to_string(),
            "echo 'Goodbye World'".to_string(),
        ]),
    }
}
