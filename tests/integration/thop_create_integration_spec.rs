// It's quite hard to reasonably test code under living under /bin/ due to specific visibility rules
// for now integraton test just start at the top level of the domain layer

use std::sync::{Arc, Mutex};

use crab_hop::{
    domain::{
        environment::MockEnvironment,
        template,
        template_repository::TemplateRepository,
        template_service::TemplateService,
        thop_service::{CreateCommand, ThopService, ThopServicePort},
    },
    engines::command_engine::{
        app::command_engine::CommandEngine, domain::command_service::CommandService,
    },
    infrastructure::{
        persistence::ram_template_repository::RamTemplateRepository,
        selector::fzf_selector::FzfSelector,
    },
};

#[test]
fn creates_template_with_path_and_name() {
    // given
    let command_service = Arc::new(CommandService::new());
    let command_engine = Arc::new(CommandEngine::new(command_service));
    // mock environment to not interact with the os
    let environment = MockEnvironment::new();
    let environment_arc = Arc::new(environment);
    let selector_arc = Arc::new(FzfSelector::new(environment_arc.clone()));

    let mut template_repository = RamTemplateRepository::new();
    template_repository.clear();

    let template_repository = Arc::new(Mutex::new(template_repository));
    let template_service = Arc::new(TemplateService::new(template_repository.clone()));

    let thop = ThopService::new(
        template_service,
        selector_arc,
        command_engine,
        environment_arc,
    );

    let command = CreateCommand {
        name: Some(template::Name("SomeName".to_string())),
        path: Some(template::Path("~/some/path".to_string())),
    };

    // when
    let result = thop.create(command.clone());

    // then
    assert!(result.is_ok());
    let path = command.path.as_ref().expect("expected path to be set");
    let name = command.name.as_ref().expect("expected name to be set");
    let template = template_repository
        .lock()
        .expect("expected repository lock to be ok")
        .find(path.clone())
        .expect("expected template to be found");

    assert_eq!(path, &template.path);
    assert_eq!(name, &template.name);
}

#[test]
fn creates_template_with_name() {
    // given
    let cwd = template::Path("~/some/path".to_string());

    let command_service = Arc::new(CommandService::new());
    let command_engine = Arc::new(CommandEngine::new(command_service));
    // mock environment to not interact with the os
    let mut environment = MockEnvironment::new();
    environment
        .expect_current_dir()
        .return_const(Ok(cwd.0.clone()));

    let environment_arc = Arc::new(environment);
    let template_selector = Arc::new(FzfSelector::new(environment_arc.clone()));

    let mut template_repository = RamTemplateRepository::new();
    template_repository.clear();

    let template_repository_arc = Arc::new(Mutex::new(template_repository));
    let template_service = Arc::new(TemplateService::new(template_repository_arc.clone()));

    let thop = ThopService::new(
        template_service,
        template_selector,
        command_engine,
        environment_arc,
    );

    let command = CreateCommand {
        name: Some(template::Name("SomeName".to_string())),
        path: None,
    };

    // when
    let result = thop.create(command.clone());

    // then
    assert!(result.is_ok());
    assert!(command.path.is_none());
    let name = command.name.as_ref().expect("expected name to be set");
    let template = template_repository_arc
        .lock()
        .expect("expected repository lock to be ok")
        .find(cwd.clone())
        .expect("expected template to be found");

    assert_eq!(&cwd, &template.path);
    assert_eq!(name, &template.name);
}

#[test]
fn creates_template_with_path() {
    // given
    let command_service = Arc::new(CommandService::new());
    let command_engine = Arc::new(CommandEngine::new(command_service));
    // mock environment to not interact with the os
    let environment = MockEnvironment::new();
    let environment_arc = Arc::new(environment);
    let template_selector = Arc::new(FzfSelector::new(environment_arc.clone()));

    let mut template_repository = RamTemplateRepository::new();
    template_repository.clear();

    let template_repository_arc = Arc::new(Mutex::new(template_repository));
    let template_service = Arc::new(TemplateService::new(template_repository_arc.clone()));

    let thop = ThopService::new(
        template_service,
        template_selector,
        command_engine,
        environment_arc,
    );

    let command = CreateCommand {
        name: None,
        path: Some(template::Path("~/some/path".to_string())),
    };

    // when
    let result = thop.create(command.clone());

    // then
    assert!(result.is_ok());
    assert!(command.name.is_none());
    let path = command.path.expect("expected path to be set");
    let name = template::Name(path.0.clone());
    let template = template_repository_arc
        .lock()
        .expect("expected repository lock to be ok")
        .find(path.clone())
        .expect("expected template to be found");

    assert_eq!(path, template.path);
    assert_eq!(name, template.name);
}

#[test]
fn creates_template() {
    // given
    let cwd = template::Path("~/some/path".to_string());

    let command_service = Arc::new(CommandService::new());
    let command_engine = Arc::new(CommandEngine::new(command_service));
    // mock environment to not interact with the os
    let mut environment = MockEnvironment::new();
    environment
        .expect_current_dir()
        .return_const(Ok(cwd.0.clone()));

    let environment_arc = Arc::new(environment);
    let template_selector = Arc::new(FzfSelector::new(environment_arc.clone()));

    let mut template_repository = RamTemplateRepository::new();
    template_repository.clear();

    let template_repository_arc = Arc::new(Mutex::new(template_repository));
    let template_service = Arc::new(TemplateService::new(template_repository_arc.clone()));

    let thop = ThopService::new(
        template_service,
        template_selector,
        command_engine,
        environment_arc,
    );

    let command = CreateCommand {
        name: None,
        path: None,
    };

    // when
    let result = thop.create(command.clone());

    // then
    assert!(result.is_ok());
    assert!(command.path.is_none());
    let name = template::Name(cwd.0.clone());
    let template = template_repository_arc
        .lock()
        .expect("expected repository lock to be ok")
        .find(cwd.clone())
        .expect("expected template to be found");

    assert_eq!(cwd, template.path);
    assert_eq!(name, template.name);
}
