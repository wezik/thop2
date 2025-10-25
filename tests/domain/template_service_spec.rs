use crab_hop::domain::template::{self, ports::MockTemplateRepository};

use crate::domain::template_fixtures::some_template;

#[test]
fn gets_template_from_repository() {
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
