use nolana::diagnostic::Diagnostic;

fn test_semantics_helper(source: &str) -> String {
    let ret = nolana::parser::Parser::new(source).parse();
    // Return the errors in a debug formatted string
    format!("{:?}", nolana::semantic::SemanticChecker::default().check(&ret.program))
}

#[test]
fn empty_block_expression() {
    let out = test_semantics_helper("{}");
    insta::assert_snapshot!(out)
}

#[test]
fn filled_block_expression() {
    let out = test_semantics_helper("{1;};");
    insta::assert_snapshot!(out)
}

#[test]
fn illegal_string_operation_both() {
    let out = test_semantics_helper("'foo' + 'bar'");

    insta::assert_snapshot!(out)
}

#[test]
fn illegal_string_operation_left() {
    let out = test_semantics_helper("'foo' == 1");

    insta::assert_snapshot!(out)
}

#[test]
fn illegal_string_operation_right() {
    let out = test_semantics_helper("1 + 'bar'");

    insta::assert_snapshot!(out)
}

#[test]
fn unequals_string_operation() {
    let out = test_semantics_helper("'bar' != 'bar'");

    insta::assert_snapshot!(out)
}

#[test]
fn equals_string_operation() {
    let out = test_semantics_helper("'bar' == 'bar'");

    insta::assert_snapshot!(out)
}

#[test]
fn assigning_context() {
    let out = test_semantics_helper("context.foo = 0;");

    insta::assert_snapshot!(out)
}

#[test]
fn break_outside_loop() {
    let out = test_semantics_helper("break;");

    insta::assert_snapshot!(out)
}

#[test]
fn break_inside_loop() {
    let out = test_semantics_helper("loop(1, {break;});");

    insta::assert_snapshot!(out)
}

#[test]
fn continue_outside_loop() {
    let out = test_semantics_helper("continue;");

    insta::assert_snapshot!(out)
}

#[test]
fn continue_inside_loop() {
    let out = test_semantics_helper("loop(1, {continue;});");

    insta::assert_snapshot!(out)
}
