use std::rc::Rc;

use crab_hop::{
    domain::{
        environment::{MockEnvironment, RunResult, ENVIRONMENT_RUN_COMMAND_ERROR},
        selector::Selector,
        template::{self, Template},
    },
    infrastructure::selector::fzf_selector::FzfSelector,
};

#[test]
fn selects_from_empty_list() {
    // given
    let environment = MockEnvironment::new();
    let selector = FzfSelector::new(Rc::new(environment));

    // when
    let result = selector.select_template(&[]);

    // then
    let selected = result.expect("expected result to be ok");
    assert!(selected.is_none());
}

#[test]
fn selects_from_list() {
    // given
    let mut environment = MockEnvironment::new();
    let templates = vec![
        Template::new(
            template::Path("~/some/path".to_string()),
            template::Name("SomeName".to_string()),
            template::Engine::Tmux,
        ),
        Template::new(
            template::Path("~/some/other/path".to_string()),
            template::Name("SomeOtherName".to_string()),
            template::Engine::Tmux,
        ),
    ];
    environment
        .expect_run_command()
        // fzf returns with appended newline
        .return_const(Ok(RunResult::Success(
            templates[1].name.0.to_string() + "\n",
        )));

    let selector = FzfSelector::new(Rc::new(environment));

    // when
    let result = selector.select_template(&templates);

    // then
    let selected = result
        .expect("expected result to be ok")
        .expect("expected result to be some");

    assert_eq!(selected, templates[1]);
}

#[test]
fn handles_130_exit_code() {
    // given
    let mut environment = MockEnvironment::new();
    let templates = vec![
        Template::new(
            template::Path("~/some/path".to_string()),
            template::Name("SomeName".to_string()),
            template::Engine::Tmux,
        ),
        Template::new(
            template::Path("~/some/other/path".to_string()),
            template::Name("SomeOtherName".to_string()),
            template::Engine::Tmux,
        ),
    ];
    environment
        .expect_run_command()
        // fzf returns with appended newline
        .return_const(Ok(RunResult::Failure(130)));

    let selector = FzfSelector::new(Rc::new(environment));

    // when
    let result = selector.select_template(&templates);

    // then
    let selected = result.expect("expected result to be ok");
    assert!(selected.is_none());
}

#[test]
fn propagates_unhandled_exit_codes_as_errors() {
    // given
    let mut environment = MockEnvironment::new();
    let templates = vec![
        Template::new(
            template::Path("~/some/path".to_string()),
            template::Name("SomeName".to_string()),
            template::Engine::Tmux,
        ),
        Template::new(
            template::Path("~/some/other/path".to_string()),
            template::Name("SomeOtherName".to_string()),
            template::Engine::Tmux,
        ),
    ];
    environment
        .expect_run_command()
        // fzf returns with appended newline
        .return_const(Ok(RunResult::Failure(2)));

    let selector = FzfSelector::new(Rc::new(environment));

    // when
    let result = selector.select_template(&templates);

    // then
    let err = result.expect_err("expected result to be err");
    assert_eq!(err.code, ENVIRONMENT_RUN_COMMAND_ERROR.code);
}
