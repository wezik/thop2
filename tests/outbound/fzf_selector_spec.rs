use std::sync::Arc;

use crab_hop::{
    domain::{
        environment::{MockEnvironment, RunResult},
        selector::Selector,
    },
    outbound::selector::fzf_selector::FzfSelector,
};

#[test]
fn selects_from_empty_list() {
    // given
    let environment = MockEnvironment::new();
    let selector = FzfSelector::new(Arc::new(environment));

    // when
    let result = selector.select_from(&[]);

    // then
    let selected = result.expect("expected result to be ok");
    assert!(selected.is_none());
}

#[test]
fn selects_from_list() {
    // given
    let mut environment = MockEnvironment::new();
    let choices = vec!["choice1", "choice2", "choice3"];
    environment
        .expect_run_command()
        // fzf returns with appended newline
        .return_const(Ok(RunResult::Success(choices[1].to_string() + "\n"))); 

    let selector = FzfSelector::new(Arc::new(environment));

    // when
    let result = selector.select_from(&choices);

    // then
    let selected = result
        .expect("expected result to be ok")
        .expect("expected result to be some");

    assert_eq!(selected, choices[1]);
}
