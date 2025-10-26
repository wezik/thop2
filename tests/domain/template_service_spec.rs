use std::sync::{Arc, Mutex};

use crab_hop::domain::{
    template_repository::MockTemplateRepository,
    template_service::{TemplateService, TemplateServicePort},
};

use crate::domain::template_fixtures::some_template;

fn mock_repository() -> Arc<Mutex<MockTemplateRepository>> {
    Arc::new(Mutex::new(MockTemplateRepository::new()))
}

#[test]
fn creates_template() {
    // given
    let template = some_template();

    let repository = mock_repository();
    repository
        .lock()
        .unwrap()
        .expect_create()
        .return_const(Ok(()));

    let service = TemplateService::new(repository.clone());

    // when
    let result = service.create(template.clone());

    // then
    assert_eq!(result.unwrap(), ());
}

#[test]
fn gets_template() {
    // given
    let template = some_template();

    let repository = mock_repository();
    repository
        .lock()
        .unwrap()
        .expect_find()
        .return_const(Ok(template.clone()));

    let service = TemplateService::new(repository.clone());

    // when
    let result = service.get(template.path.clone());

    // then
    assert_eq!(result.unwrap(), template);
}

#[test]
fn lists_templates() {
    // given
    let templates = vec![some_template(), some_template()];

    let repository = mock_repository();
    repository
        .lock()
        .unwrap()
        .expect_list()
        .return_const(templates.clone());

    let service = TemplateService::new(repository);

    // when
    let result = service.list();

    // then
    assert_eq!(result, templates);
}
