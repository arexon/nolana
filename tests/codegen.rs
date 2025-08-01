use nolana::{codegen::Codegen, parser::Parser};
use insta::assert_snapshot;

fn codegen_test_helper(source: &str) -> String {
    let result = Parser::new(source).parse();
    assert!(result.errors.is_empty());
    assert!(!result.panicked);
    Codegen::default().build(&result.program)
}

#[test]
fn boolean() {
    let out = codegen_test_helper("false; true;");
    insta::assert_snapshot!(out);
}

#[test]
fn string() {
    let out = codegen_test_helper("'foo_bar123.-$#*()'");

    insta::assert_snapshot!(out);
}

#[test]
fn variable() {
    let out = codegen_test_helper("variable.foo; v.foo; temp.foo; t.foo; context.foo; c.foo;");

    insta::assert_snapshot!(out);
}

#[test]
fn weird_variable_members() {
    let out = codegen_test_helper("variable.v.temp.t.context.c.query.q.math.a.b.c");

    insta::assert_snapshot!(out);
}

#[test]
fn binary_and_unary_operations() {
    let out = codegen_test_helper(
        "1 == (((2 != 3) < 4 <= 5 > 6) >= -7 + 8 - 9 * 10 / 11 || 12) && !(13 ?? 14)",
    );

    insta::assert_snapshot!(out)
}

#[test]
fn conditional() {
    let out = codegen_test_helper("q.foo ? 1");
    insta::assert_snapshot!(out)
}

#[test]
fn ternary() {
    let out = codegen_test_helper("q.foo ? 1 : 0");
    insta::assert_snapshot!(out)
}

#[test]
fn assignment() {
    let out = codegen_test_helper("v.cow.location = 16;");
    insta::assert_snapshot!(out)
}

#[test]
fn parenthesis_single() {
    let out = codegen_test_helper("((((16))))");
    insta::assert_snapshot!(out)
}

#[test]
fn parenthesis_complex() {
    let out = codegen_test_helper("(1; 2; (3; (4; 5;);););");
    insta::assert_snapshot!(out)
}

#[test]
fn block() {
    let out = codegen_test_helper("{v.a = 0;};");
    insta::assert_snapshot!(out)
}

#[test]
fn resource() {
    let out = codegen_test_helper("geometry.foo; material.foo; texture.foo;");
    insta::assert_snapshot!(out)
}

#[test]
fn array_access() {
    let out = codegen_test_helper("array.foo[q.bar]");
    insta::assert_snapshot!(out)
}

#[test]
fn arrow_access() {
    let out = codegen_test_helper("v.foo->v.bar");
    insta::assert_snapshot!(out)
}

#[test]
fn r#loop() {
    let out = codegen_test_helper("loop(10, {v.i = v.i + 1;});");
    insta::assert_snapshot!(out)
}

#[test]
fn for_each() {
    let out = codegen_test_helper("for_each(v.a, q.foo, {v.b = v.a + 1;});");
    insta::assert_snapshot!(out)
}

#[test]
fn keywords() {
    let out = codegen_test_helper("return v.a; break; continue; this;");
    insta::assert_snapshot!(out)
}
