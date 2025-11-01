// It's quite hard to reasonably test code under living under /bin/ due to specific visibility rules
// for now integraton test just start at the top level of the domain layer

use std::rc::Rc;

use crab_hop::{
    domain::{
        environment::MockEnvironment,
        template,
        template_repository::TemplateRepository,
        template_service::TemplateService,
        thop_service::{CreateCommand, ThopService, ThopServicePort},
    },
    infrastructure::{
        multiplexer::multi_multiplexer_gateway::MultiMultiplexerGateway,
        persistence::ram_template_repository::RamTemplateRepository,
        selector::fzf_selector::FzfSelector,
    },
    multiplexer::tmux::{app::tmux_api::TmuxApi, domain::tmux_service::TmuxService},
};

#[test]
fn creates_template_with_path_and_name() {
    // given
    let tmux_service_rc = Rc::new(TmuxService::new());
    let tmux_api_rc = Rc::new(TmuxApi::new(tmux_service_rc));
    let multiplexer_gateway_rc = Rc::new(MultiMultiplexerGateway::new(tmux_api_rc));

    // mock environment to not interact with the os
    let environment = MockEnvironment::new();
    let environment_rc = Rc::new(environment);
    let selector_arc = Rc::new(FzfSelector::new(environment_rc.clone()));

    let template_repository = RamTemplateRepository::new();
    template_repository.clear();

    let template_repository = Rc::new(template_repository);
    let temmplate_service_rc = Rc::new(TemplateService::new(template_repository.clone()));

    let thop = ThopService::new(
        temmplate_service_rc,
        selector_arc,
        multiplexer_gateway_rc,
        environment_rc,
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
        .find(path.clone())
        .expect("expected template to be found");

    assert_eq!(path, &template.path);
    assert_eq!(name, &template.name);
}

#[test]
fn creates_template_with_name() {
    // given
    let cwd = template::Path("~/some/path".to_string());

    let tmux_service_rc = Rc::new(TmuxService::new());
    let tmux_api_rc = Rc::new(TmuxApi::new(tmux_service_rc));
    let multiplexer_gateway_rc = Rc::new(MultiMultiplexerGateway::new(tmux_api_rc));
    // mock environment to not interact with the os
    let mut environment = MockEnvironment::new();
    environment
        .expect_current_dir()
        .return_const(Ok(cwd.0.clone()));

    let environment_rc = Rc::new(environment);
    let template_selector = Rc::new(FzfSelector::new(environment_rc.clone()));

    let template_repository = RamTemplateRepository::new();
    template_repository.clear();

    let template_repository_rc = Rc::new(template_repository);
    let temmplate_service_rc = Rc::new(TemplateService::new(template_repository_rc.clone()));

    let thop = ThopService::new(
        temmplate_service_rc,
        template_selector,
        multiplexer_gateway_rc,
        environment_rc,
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
    let template = template_repository_rc
        .find(cwd.clone())
        .expect("expected template to be found");

    assert_eq!(&cwd, &template.path);
    assert_eq!(name, &template.name);
}

#[test]
fn creates_template_with_path() {
    // given
    let tmux_service_rc = Rc::new(TmuxService::new());
    let tmux_api_rc = Rc::new(TmuxApi::new(tmux_service_rc));
    let multiplexer_gateway_rc = Rc::new(MultiMultiplexerGateway::new(tmux_api_rc));
    // mock environment to not interact with the os
    let environment = MockEnvironment::new();
    let environment_rc = Rc::new(environment);
    let template_selector = Rc::new(FzfSelector::new(environment_rc.clone()));

    let template_repository = RamTemplateRepository::new();
    template_repository.clear();

    let template_repository_rc = Rc::new(template_repository);
    let temmplate_service_rc = Rc::new(TemplateService::new(template_repository_rc.clone()));

    let thop = ThopService::new(
        temmplate_service_rc,
        template_selector,
        multiplexer_gateway_rc,
        environment_rc,
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
    let template = template_repository_rc
        .find(path.clone())
        .expect("expected template to be found");

    assert_eq!(path, template.path);
    assert_eq!(name, template.name);
}

#[test]
fn creates_template() {
    // given
    let cwd = template::Path("~/some/path".to_string());

    let tmux_service_rc = Rc::new(TmuxService::new());
    let tmux_api_rc = Rc::new(TmuxApi::new(tmux_service_rc));
    let multiplexer_gateway_rc = Rc::new(MultiMultiplexerGateway::new(tmux_api_rc));
    // mock environment to not interact with the os
    let mut environment = MockEnvironment::new();
    environment
        .expect_current_dir()
        .return_const(Ok(cwd.0.clone()));

    let environment_rc = Rc::new(environment);
    let template_selector = Rc::new(FzfSelector::new(environment_rc.clone()));

    let template_repository = RamTemplateRepository::new();
    template_repository.clear();

    let template_repository_rc = Rc::new(template_repository);
    let temmplate_service_rc = Rc::new(TemplateService::new(template_repository_rc.clone()));

    let thop = ThopService::new(
        temmplate_service_rc,
        template_selector,
        multiplexer_gateway_rc,
        environment_rc,
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
    let template = template_repository_rc
        .find(cwd.clone())
        .expect("expected template to be found");

    assert_eq!(cwd, template.path);
    assert_eq!(name, template.name);
}
