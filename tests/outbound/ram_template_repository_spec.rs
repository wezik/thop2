use std::collections::HashSet;

use crab_hop::{
    domain::{
        template::{self, Engine, Template, TEMPLATE_ALREADY_EXSISTS},
        template_repository::TemplateRepository,
    },
    outbound::persistence::ram_template_repository::RamTemplateRepository,
};

fn repository() -> RamTemplateRepository {
    let mut repository = RamTemplateRepository::new();
    repository.clear();
    repository
}

#[test]
fn creates_and_finds_templates() {
    // given
    let template = Template::new(
        template::Path("~/some/path".to_string()),
        template::Name("SomeName".to_string()),
        Engine::Command,
    );
    let mut repository = repository();

    // when
    let result = repository.create(template.clone());

    // then
    assert!(result.is_ok());
    assert_eq!(repository.find(template.path.clone()).unwrap(), template);
}

#[test]
fn prevents_creating_duplicate_templates() {
    // given
    let template = Template::new(
        template::Path("~/some/path".to_string()),
        template::Name("SomeName".to_string()),
        Engine::Command,
    );
    let mut repository = repository();
    repository.create(template.clone()).unwrap();

    // when
    let result = repository.create(template.clone());

    // then
    let err = result.expect_err("expected error");
    assert_eq!(err.code, TEMPLATE_ALREADY_EXSISTS.code);
}

#[test]
fn lists_templates() {
    // given
    let templates = vec![
        Template::new(
            template::Path("~/some/path".to_string()),
            template::Name("SomeName".to_string()),
            Engine::Command,
        ),
        Template::new(
            template::Path("~/some/other/path".to_string()),
            template::Name("SomeOtherName".to_string()),
            Engine::Command,
        ),
    ];
    let mut repository = repository();
    templates
        .iter()
        .for_each(|t| repository.create(t.clone()).unwrap());

    // when
    let result = repository.list();

    // then
    let list = result.expect("expected result to be ok");
    assert_eq!(
        HashSet::<Template>::from_iter(list),
        HashSet::from_iter(templates)
    );
}
