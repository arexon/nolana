use insta::assert_snapshot;
use nolana::{parser::Parser, semantic::SemanticChecker};

fn semantics(source: &str) -> String {
    let mut result = Parser::new(source).parse();
    let diagnostics = SemanticChecker::default().check(&mut result.program);
    format!("{diagnostics:#?}")
}

#[test]
fn empty_block_expression() {
    let out = semantics("{}");
    assert_snapshot!(out)
}

#[test]
fn filled_block_expression() {
    let out = semantics("{1;};");
    assert_snapshot!(out)
}

#[test]
fn illegal_string_operation_both() {
    let out = semantics("'foo' + 'bar'");
    assert_snapshot!(out)
}

#[test]
fn illegal_string_operation_left() {
    let out = semantics("'foo' == 1");
    assert_snapshot!(out)
}

#[test]
fn illegal_string_operation_right() {
    let out = semantics("1 + 'bar'");
    assert_snapshot!(out)
}

#[test]
fn unequals_string_operation() {
    let out = semantics("'bar' != 'bar'");
    assert_snapshot!(out)
}

#[test]
fn equals_string_operation() {
    let out = semantics("'bar' == 'bar'");
    assert_snapshot!(out)
}

#[test]
fn assigning_context() {
    let out = semantics("context.foo = 0;");
    assert_snapshot!(out)
}

#[test]
fn break_outside_loop() {
    let out = semantics("break;");
    assert_snapshot!(out)
}

#[test]
fn break_inside_loop() {
    let out = semantics("loop(1, {break;});");
    assert_snapshot!(out)
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
