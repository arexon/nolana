use insta::assert_snapshot;
use nolana::{codegen::Codegen, parser::Parser};

fn codegen_test_helper(source: &str) -> String {
    let ret = Parser::new(source).parse();
    let out = Codegen::default().build(&ret.program);
    assert!(ret.errors.is_empty());
    assert!(!ret.panicked);
    out
}

#[test]
fn boolean() {
    let out = codegen_test_helper("false; true;");
    assert_snapshot!(out);
}

#[test]
fn string() {
    let out = codegen_test_helper("'foo_bar123.-$#*()'");
    assert_snapshot!(out);
}

#[test]
fn variable() {
    let out = codegen_test_helper("variable.foo; v.foo; temp.foo; t.foo; context.foo; c.foo;");
    assert_snapshot!(out);
}

#[test]
fn weird_variable_members() {
    let out = codegen_test_helper("variable.v.temp.t.context.c.query.q.math.a.b.c");
    assert_snapshot!(out);
}

#[test]
fn binary_and_unary_operations() {
    let out = codegen_test_helper(
        "1 == (((2 != 3) < 4 <= 5 > 6) >= -7 + 8 - 9 * 10 / 11 || 12) && !(13 ?? 14)",
    );
    assert_snapshot!(out)
}

#[test]
fn conditional() {
    let out = codegen_test_helper("q.foo ? 1");
    assert_snapshot!(out)
}

#[test]
fn ternary() {
    let out = codegen_test_helper("q.foo ? 1 : 0");
    assert_snapshot!(out)
}

#[test]
fn assignment() {
    let out = codegen_test_helper("v.cow.location = 16;");
    assert_snapshot!(out)
}

#[test]
fn parenthesis_single() {
    let out = codegen_test_helper("((((16))))");
    assert_snapshot!(out)
}

#[test]
fn parenthesis_complex() {
    let out = codegen_test_helper("(1; 2; (3; (4; 5;);););");
    assert_snapshot!(out)
}

#[test]
fn block() {
    let out = codegen_test_helper("{v.a = 0;};");
    assert_snapshot!(out)
}

#[test]
fn resource() {
    let out = codegen_test_helper("geometry.foo; material.foo; texture.foo;");
    assert_snapshot!(out)
}

#[test]
fn array_access() {
    let out = codegen_test_helper("array.foo[q.bar]");
    assert_snapshot!(out)
}

#[test]
fn arrow_access() {
    let out = codegen_test_helper("v.foo->v.bar");
    assert_snapshot!(out)
}

#[test]
fn r#loop() {
    let out = codegen_test_helper("loop(10, {v.i = v.i + 1;});");
    assert_snapshot!(out)
}

#[test]
fn for_each() {
    let out = codegen_test_helper("for_each(v.a, q.foo, {v.b = v.a + 1;});");
    assert_snapshot!(out)
}

#[test]
fn keywords() {
    let out = codegen_test_helper("return v.a; break; continue; this;");
    assert_snapshot!(out)
}
