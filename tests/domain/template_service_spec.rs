use std::sync::{Arc, Mutex};

use crab_hop::domain::{
    template_repository::MockTemplateRepository,
    template_service::{TemplateService, TemplateServicePort},
};

use crate::domain::template_fixtures::some_template;

#[test]
fn creates_template() {
    // given
    let template = some_template();

    let mut repository = MockTemplateRepository::new();
    repository.expect_create().return_const(Ok(()));

    let service = TemplateService::new(Arc::new(Mutex::new(repository)));

    // when
    let result = service.create(template.clone());

    // then
    assert!(result.is_ok());
}

#[test]
fn gets_template() {
    // given
    let expected_template = some_template();

    let mut repository = MockTemplateRepository::new();
    repository
        .expect_find()
        .return_const(Ok(expected_template.clone()));

    let service = TemplateService::new(Arc::new(Mutex::new(repository)));

    // when
    let result = service.get(expected_template.path.clone());

    // then
    let template = result.expect("expected result to be ok");
    assert_eq!(template, expected_template);
}

#[test]
fn lists_templates() {
    // given
    let templates = vec![some_template(), some_template()];

    let mut repository = MockTemplateRepository::new();
    repository.expect_list().return_const(Ok(templates.clone()));

    let service = TemplateService::new(Arc::new(Mutex::new(repository)));

    // when
    let result = service.list();

    // then
    let list = result.expect("expected result to be ok");
    assert_eq!(list, templates);
}

#[test]
fn deletes_template() {
    // given
    let expected_template = some_template();

    let mut repository = MockTemplateRepository::new();
    repository.expect_delete().return_const(Ok(()));

    let service = TemplateService::new(Arc::new(Mutex::new(repository)));

    // when
    let result = service.delete(expected_template.path.clone());

    // then
    assert!(result.is_ok());
}
