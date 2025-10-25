use crab_hop::domain::template::{self, ports::MockTemplateRepository};

use crate::domain::template_fixtures::some_template;

#[test]
fn gets_template() {
    // given
    let template = some_template();

    let mut repository = MockTemplateRepository::new();
    repository.expect_find().return_const(Ok(template.clone()));

    let service = template::service::new(Box::new(repository));

    // when
    let result = service.get(template.path.clone());

    // then
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), template);
}

#[test]
fn lists_templates() {
    // given
    let templates = vec![some_template()];

    let mut repository = MockTemplateRepository::new();
    repository.expect_list().return_const(templates.clone());

    let service = template::service::new(Box::new(repository));

    // when
    let result = service.list();

    // then
    assert_eq!(result, templates);
}
