use insta::assert_snapshot;
use nolana::Parser;

fn parse(source: &str) -> String {
    let result = Parser::new(source).parse();
    format!("{result:#?}")
}

#[test]
fn boolean_false() {
    let out = parse("false");
    assert_snapshot!(out)
}

#[test]
fn boolean_true() {
    let out = parse("true");
    assert_snapshot!(out)
}

#[test]
fn string() {
    let out = parse("'foo_bar123.-$#*()'");
    assert_snapshot!(out)
}

#[test]
fn unterminated_string() {
    let out = parse("'hello wor-");
    assert_snapshot!(out)
}

#[test]
fn variable_variable() {
    let out = parse("variable.foo");
    assert_snapshot!(out)
}

#[test]
fn variable_v() {
    let out = parse("v.foo");
    assert_snapshot!(out)
}

#[test]
fn variable_temp() {
    let out = parse("temp.foo");
    assert_snapshot!(out)
}

#[test]
fn variable_t() {
    let out = parse("t.foo");
    assert_snapshot!(out)
}

#[test]
fn variable_context() {
    let out = parse("context.foo");
    assert_snapshot!(out)
}

#[test]
fn variable_c() {
    let out = parse("c.foo");
    assert_snapshot!(out);
}

#[test]
fn weird_variable_members() {
    let out = parse("variable.v.temp.t.context.c.query.q.math.a.b.c");
    assert_snapshot!(out)
}

#[test]
fn binary_operation() {
    let out = parse("1 + 2 * 3");
    assert_snapshot!(out)
}

#[test]
fn parenthesized_binary_operation() {
    let out = parse("(1 + 1) * (1 + 1)");
    assert_snapshot!(out)
}

#[test]
fn parenthesized_binary_operation_alt() {
    let out = parse("((2 * 3) + 1) / 2");
    assert_snapshot!(out)
}

#[test]
fn negate_operation() {
    let out = parse("-(1 + 1)");
    assert_snapshot!(out)
}

#[test]
fn not_operation() {
    let out = parse("!(1 && 0)");
    assert_snapshot!(out)
}

#[test]
fn null_operation() {
    let out = parse("v.a ?? 1.2");
    assert_snapshot!(out)
}

#[test]
fn ternary_double_left() {
    let out = parse("q.foo ? v.bar == 13 ? 1 : 2 : 3");
    assert_snapshot!(out)
}

#[test]
fn ternary_double_right() {
    let out = parse("q.foo ? 1 : v.bar == 13 ? 2 : 3");
    assert_snapshot!(out)
}

#[test]
fn conditional() {
    let out = parse("q.foo ? 1");
    assert_snapshot!(out)
}

#[test]
fn assignment() {
    let out = parse("v.cow.location.x = 204.31; v.cow.location.y = 87; v.cow.location.z = 48.933;");
    assert_snapshot!(out)
}

#[test]
fn complex_expression() {
    let out = parse("0; 0; 0;");
    assert_snapshot!(out);
}

#[test]
fn complex_parenthesized_expression() {
    let out = parse("(v.a = 1; v.b = 2;);");

    assert_snapshot!(out);
}

#[test]
fn empty_parenthesized_expression() {
    let out = parse("()");
    assert_snapshot!(out);
}

#[test]
fn nested_parenthesis() {
    let out = parse("((((16))))");
    assert_snapshot!(out);
}

#[test]
fn block() {
    let out = parse("{1;};");
    assert_snapshot!(out);
}

#[test]
fn block_undelimited() {
    let out = parse("{1}");
    assert_snapshot!(out);
}

#[test]
fn unclosed_parenthesis_in_call() {
    let out = parse("q.a(1");
    assert_snapshot!(out);
}

#[test]
fn unclosed_parenthesis_in_parenthesized_expression() {
    let out = parse("(1+1");
    assert_snapshot!(out);
}

#[test]
fn resource_geometry() {
    let out = parse("geometry.foo");
    assert_snapshot!(out);
}

#[test]
fn resource_material() {
    let out = parse("material.bar");
    assert_snapshot!(out);
}

#[test]
fn resource_texture() {
    let out = parse("texture.baz");
    assert_snapshot!(out);
}

#[test]
fn array_access() {
    let out = parse("array.foo[q.bar]");
    assert_snapshot!(out);
}

#[test]
fn arrow_access() {
    let out = parse("v.foo->v.bar");
    assert_snapshot!(out);
}

#[test]
fn r#loop() {
    let out = parse("loop(10, {v.i = v.i + 1;});");
    assert_snapshot!(out);
}

#[test]
fn loop_in_expression() {
    let out = parse("1 + loop(10, {0;})");
    assert_snapshot!(out);
}

#[test]
fn for_each() {
    let out = parse("for_each(v.a, q.foo, {v.b = v.a + 1;});");
    assert_snapshot!(out);
}

#[test]
fn for_each_in_expression() {
    let out = parse("1 + for_each(v.a, q.foo, {0;})");
    assert_snapshot!(out);
}

#[test]
fn for_each_wrong_first_arg() {
    let out = parse("for_each(1, q.foo, {v.b = v.a + 1;});");
    assert_snapshot!(out);
}

#[test]
fn r#return() {
    let out = parse("return v.a");
    assert_snapshot!(out);
}

#[test]
fn r#break() {
    let out = parse("break");
    assert_snapshot!(out);
}

#[test]
fn r#continue() {
    let out = parse("continue");
    assert_snapshot!(out);
}

#[test]
fn this() {
    let out = parse("this");
    assert_snapshot!(out);
}

#[test]
fn missing_semi_with_semi() {
    let out = parse("0; 0");
    assert_snapshot!(out);
}

#[test]
fn missing_semi_with_assignment() {
    let out = parse("v.a = 0; v.a");
    assert_snapshot!(out);
}

#[test]
fn semisemisemisemi() {
    let out = parse(
        "
        ;;;;;;; ;;;;;;; ;;;    ;;; ;;
        ;;      ;;      ;;;;  ;;;; ;;
        ;;;;;;; ;;;;;   ;; ;;;; ;; ;;
             ;; ;;      ;;  ;;  ;; ;;
        ;;;;;;; ;;;;;;; ;;      ;; ;;
        ",
    );
    assert_snapshot!(out);
}
