use insta::assert_snapshot;
use nolana::{
    nolana::parser::Parser,
    nolana::semantic::SemanticChecker   
};

#[cfg(test)]
fn test_semantics_helper(source: &str) -> String {
    let ret = Parser::new(source).parse();
    // Return the errors in a debug formatted string
    format!("{:?}", SemanticChecker::default().check(&ret.program))
}

#[test]
fn empty_block_expression() {
    let out = test_semantics_helper("{}");
    assert_snapshot!(out)
}

#[test]
fn filled_block_expression() {
    let out = test_semantics_helper("{1;};");
    assert_snapshot!(out)
}

#[test]
fn illegal_string_operation_both() {
    let out = test_semantics_helper("'foo' + 'bar'");
    assert_snapshot!(out)
}

#[test]
fn illegal_string_operation_left() {
    let out = test_semantics_helper("'foo' == 1");
    assert_snapshot!(out)
}

#[test]
fn illegal_string_operation_right() {
    let out = test_semantics_helper("1 + 'bar'");
    assert_snapshot!(out)
}

#[test]
fn unequals_string_operation() {
    let out = test_semantics_helper("'bar' != 'bar'");
    assert_snapshot!(out)
}

#[test]
fn equals_string_operation() {
    let out = test_semantics_helper("'bar' == 'bar'");
    assert_snapshot!(out)
}

#[test]
fn assigning_context() {
    let out = test_semantics_helper("context.foo = 0;");
    assert_snapshot!(out)
}

#[test]
fn break_outside_loop() {
    let out = test_semantics_helper("break;");
    assert_snapshot!(out)
}

#[test]
fn break_inside_loop() {
    let out = test_semantics_helper("loop(1, {break;});");
    assert_snapshot!(out)
}

#[test]
fn continue_outside_loop() {
    let out = test_semantics_helper("continue;");
    assert_snapshot!(out)
}

#[test]
fn continue_inside_loop() {
    let out = test_semantics_helper("loop(1, {continue;});");
    assert_snapshot!(out)
}
