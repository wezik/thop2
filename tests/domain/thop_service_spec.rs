use std::sync::Arc;

use crab_hop::domain::{
    environment::MockEnvironment,
    multiplexer_gateway::MockMultiplexerGateway,
    selector::MockSelector,
    template::{self, Template},
    template_service::MockTemplateServicePort,
    thop_service::{CreateCommand, DeleteCommand, OpenCommand, ThopService, ThopServicePort},
};

#[test]
fn creates_template() {
    // given
    struct TestCase {
        path: Option<template::Path>,
        name: Option<template::Name>,
        expected_path: template::Path,
        expected_name: template::Name,
    }

    let cwd = template::Path("~/".to_string());
    let some_path = template::Path("~/some/path".to_string());
    let some_name = template::Name("SomeName".to_string());

    let test_cases = vec![
        TestCase {
            path: None,
            name: None,
            expected_path: cwd.clone(),
            expected_name: template::Name(cwd.0.clone()),
        },
        TestCase {
            path: Some(some_path.clone()),
            name: None,
            expected_path: some_path.clone(),
            expected_name: template::Name(some_path.0.clone()),
        },
        TestCase {
            path: None,
            name: Some(some_name.clone()),
            expected_path: cwd.clone(),
            expected_name: some_name.clone(),
        },
        TestCase {
            path: Some(some_path.clone()),
            name: Some(some_name.clone()),
            expected_path: some_path.clone(),
            expected_name: some_name.clone(),
        },
    ];

    for test_case in test_cases {
        let command = CreateCommand {
            name: test_case.name,
            path: test_case.path,
        };

        let default_template = Template::new(
            test_case.expected_path.clone(),
            test_case.expected_name.clone(),
            template::Engine::Tmux,
        );

        let mut template_service = MockTemplateServicePort::new();
        template_service
            .expect_create()
            .withf(move |t| *t == default_template)
            .return_const(Ok(()));

        let mut environment = MockEnvironment::new();
        environment
            .expect_current_dir()
            .return_const(Ok(cwd.clone().0));

        let selector = MockSelector::new();
        let multiplexer_gateway = MockMultiplexerGateway::new();

        let service = ThopService::new(
            Arc::new(template_service),
            Arc::new(selector),
            Arc::new(multiplexer_gateway),
            Arc::new(environment),
        );

        // when
        let result = service.create(command);

        // then
        assert!(result.is_ok());
    }
}

#[test]
fn opens_exact_template() {
    // given
    let path = template::Path("~/some/path".to_string());
    let template = Template::new(
        path.clone(),
        template::Name("SomeName".to_string()),
        template::Engine::Tmux,
    );

    let command = OpenCommand {
        path: Some(path.clone()),
    };

    let mut template_service = MockTemplateServicePort::new();
    template_service
        .expect_get()
        .withf(move |p| *p == path.clone())
        .return_const(Ok(template.clone()));

    let environment = MockEnvironment::new();
    let selector = MockSelector::new();
    let mut multiplexer_gateway = MockMultiplexerGateway::new();

    multiplexer_gateway
        .expect_open()
        .withf(move |t| *t == template.clone())
        .return_const(Ok(()));

    let service = ThopService::new(
        Arc::new(template_service),
        Arc::new(selector),
        Arc::new(multiplexer_gateway),
        Arc::new(environment),
    );

    // when
    let result = service.open(command);

    // then
    assert!(result.is_ok());
}

#[test]
fn opens_selected_template() {
    // given
    let path = template::Path("~/some/path".to_string());
    let name = template::Name("SomeName".to_string());
    let template = Template::new(path.clone(), name.clone(), template::Engine::Tmux);

    let templates = vec![
        template.clone(),
        Template::new(
            template::Path("~/some/other/path".to_string()),
            template::Name("SomeOtherName".to_string()),
            template::Engine::Tmux,
        ),
    ];

    let command = OpenCommand { path: None };

    let mut template_service = MockTemplateServicePort::new();
    template_service
        .expect_list()
        .return_const(Ok(templates.clone()));

    let environment = MockEnvironment::new();
    let mut selector = MockSelector::new();
    selector
        .expect_select_template()
        .withf(move |t| t == templates.clone())
        .return_const(Ok(Some(template.clone())));

    let mut multiplexer_gateway = MockMultiplexerGateway::new();
    multiplexer_gateway
        .expect_open()
        .withf(move |t| *t == template.clone())
        .return_const(Ok(()));

    let service = ThopService::new(
        Arc::new(template_service),
        Arc::new(selector),
        Arc::new(multiplexer_gateway),
        Arc::new(environment),
    );

    // when
    let result = service.open(command);

    // then
    assert!(result.is_ok());
}

#[test]
fn skips_opening_if_selection_is_none() {
    // given
    let path = template::Path("~/some/path".to_string());
    let name = template::Name("SomeName".to_string());
    let template = Template::new(path.clone(), name.clone(), template::Engine::Tmux);

    let templates = vec![
        template.clone(),
        Template::new(
            template::Path("~/some/other/path".to_string()),
            template::Name("SomeOtherName".to_string()),
            template::Engine::Tmux,
        ),
    ];

    let command = OpenCommand { path: None };

    let mut template_service = MockTemplateServicePort::new();
    template_service
        .expect_list()
        .return_const(Ok(templates.clone()));

    let environment = MockEnvironment::new();
    let mut selector = MockSelector::new();
    selector
        .expect_select_template()
        .withf(move |t| t == templates.clone())
        .return_const(Ok(None));

    let multiplexer_gateway = MockMultiplexerGateway::new();

    let service = ThopService::new(
        Arc::new(template_service),
        Arc::new(selector),
        Arc::new(multiplexer_gateway),
        Arc::new(environment),
    );

    // when
    let result = service.open(command);

    // then
    assert!(result.is_ok());
}

#[test]
fn deletes_selected_template() {
    // given
    let some_path = template::Path("~/some/path".to_string());
    let some_name = template::Name("SomeName".to_string());

    let command = DeleteCommand { path: None };
    let template = Template::new(some_path.clone(), some_name.clone(), template::Engine::Tmux);

    let templates = vec![
        template.clone(),
        Template::new(
            template::Path("~/some/other/path".to_string()),
            template::Name("SomeOtherName".to_string()),
            template::Engine::Tmux,
        ),
    ];

    let mut template_service = MockTemplateServicePort::new();
    template_service
        .expect_list()
        .return_const(Ok(templates.clone()));
    template_service
        .expect_delete()
        .withf(move |p| p == &some_path)
        .return_const(Ok(()));

    let environment = MockEnvironment::new();

    let mut selector = MockSelector::new();
    selector
        .expect_select_template()
        .withf(move |t| t == templates.clone())
        .return_const(Ok(Some(template.clone())));

    let multiplexer_gateway = MockMultiplexerGateway::new();

    let service = ThopService::new(
        Arc::new(template_service),
        Arc::new(selector),
        Arc::new(multiplexer_gateway),
        Arc::new(environment),
    );

    // when
    let result = service.delete(command);

    // then
    assert!(result.is_ok());
}

#[test]
fn deletes_exact_template() {
    // given
    let some_path = template::Path("~/some/path".to_string());

    let command = DeleteCommand {
        path: Some(some_path.clone()),
    };

    let mut template_service = MockTemplateServicePort::new();
    template_service
        .expect_delete()
        .withf(move |p| p == &some_path)
        .return_const(Ok(()));

    let environment = MockEnvironment::new();

    let selector = MockSelector::new();
    let multiplexer_gateway = MockMultiplexerGateway::new();

    let service = ThopService::new(
        Arc::new(template_service),
        Arc::new(selector),
        Arc::new(multiplexer_gateway),
        Arc::new(environment),
    );

    // when
    let result = service.delete(command);

    // then
    assert!(result.is_ok());
}

#[test]
fn skips_deleting_if_selection_is_none() {
    // given
    let some_path = template::Path("~/some/path".to_string());
    let some_name = template::Name("SomeName".to_string());

    let command = DeleteCommand { path: None };

    let templates = vec![
        Template::new(some_path.clone(), some_name.clone(), template::Engine::Tmux),
        Template::new(
            template::Path("~/some/other/path".to_string()),
            template::Name("SomeOtherName".to_string()),
            template::Engine::Tmux,
        ),
    ];

    let mut template_service = MockTemplateServicePort::new();
    template_service
        .expect_list()
        .return_const(Ok(templates.clone()));
    template_service
        .expect_delete()
        .withf(move |p| p == &some_path)
        .return_const(Ok(()));

    let environment = MockEnvironment::new();

    let mut selector = MockSelector::new();
    selector
        .expect_select_template()
        .withf(move |t| t == templates.clone())
        .return_const(Ok(None));

    let multiplexer_gateway = MockMultiplexerGateway::new();

    let service = ThopService::new(
        Arc::new(template_service),
        Arc::new(selector),
        Arc::new(multiplexer_gateway),
        Arc::new(environment),
    );

    // when
    let result = service.delete(command);

    // then
    assert!(result.is_ok());
}
