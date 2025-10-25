use crab_hop::domain::{
    template_repository::MockTemplateRepository, template_service::TemplateService,
};

use crate::domain::template_fixtures::some_template;

#[test]
fn gets_template() {
    // given
    let template = some_template();

    let mut repository = MockTemplateRepository::new();
    repository.expect_find().return_const(Ok(template.clone()));

    let service = TemplateService::new(Box::new(repository));

    // when
    let result = service.get(template.path.clone());

    // then
    assert_eq!(result.unwrap(), template);
}

#[test]
fn lists_templates() {
    // given
    let templates = vec![some_template(), some_template()];

    let mut repository = MockTemplateRepository::new();
    repository.expect_list().return_const(templates.clone());

    let service = TemplateService::new(Box::new(repository));

    // when
    let result = service.list();

    // then
    assert_eq!(result, templates);
}
