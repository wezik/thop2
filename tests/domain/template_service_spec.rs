use std::rc::Rc;

use crab_hop::domain::template_service::TemplateServicePort;
use crab_hop::domain::{
    template::{self, Template},
    template_repository::MockTemplateRepository,
    template_service::TemplateService,
};

#[test]
fn creates_template() {
    // given
    let template = Template::new(
        template::Path("some path".to_string()),
        template::Name("some name".to_string()),
        template::Engine::Tmux,
    );
    let mut repository = MockTemplateRepository::new();
    repository.expect_create().return_const(Ok(()));

    let service = TemplateService::new(Rc::new(repository));

    // when
    let result = service.create(template.clone());

    // then
    assert!(result.is_ok());
}

#[test]
fn gets_template() {
    // given
    let template = Template::new(
        template::Path("some path".to_string()),
        template::Name("some name".to_string()),
        template::Engine::Tmux,
    );

    let mut repository = MockTemplateRepository::new();
    repository.expect_find().return_const(Ok(template.clone()));

    let service = TemplateService::new(Rc::new(repository));

    // when
    let result = service.get(template.path.clone());

    // then
    let template = result.expect("expected result to be ok");
    assert_eq!(template, template);
}

#[test]
fn lists_templates() {
    // given
    let templates = vec![
        Template::new(
            template::Path("some path".to_string()),
            template::Name("some name".to_string()),
            template::Engine::Tmux,
        ),
        Template::new(
            template::Path("some path".to_string()),
            template::Name("some name".to_string()),
            template::Engine::Tmux,
        ),
    ];

    let mut repository = MockTemplateRepository::new();
    repository.expect_list().return_const(Ok(templates.clone()));

    let service = TemplateService::new(Rc::new(repository));

    // when
    let result = service.list();

    // then
    let list = result.expect("expected result to be ok");
    assert_eq!(list, templates);
}

#[test]
fn deletes_template() {
    // given
    let template = Template::new(
        template::Path("some path".to_string()),
        template::Name("some name".to_string()),
        template::Engine::Tmux,
    );

    let mut repository = MockTemplateRepository::new();
    repository.expect_delete().return_const(Ok(()));

    let service = TemplateService::new(Rc::new(repository));

    // when
    let result = service.delete(template.path);

    // then
    assert!(result.is_ok());
}
