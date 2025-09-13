use insta::assert_snapshot;
use nolana::{Parser, semantic::SemanticChecker};

fn semantics(source: &str) -> String {
    let mut result = Parser::new(source).parse();
    let diagnostics = SemanticChecker::default().check(&mut result.program);
    format!("{diagnostics:#?}")
}

#[test]
fn continue_outside_loop() {
    let out = semantics("continue;");
    assert_snapshot!(out)
}

#[test]
fn continue_inside_loop() {
    let out = semantics("loop(1, {continue;});");
    assert_snapshot!(out)
}
