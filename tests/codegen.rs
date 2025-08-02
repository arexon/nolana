use insta::assert_snapshot;
use nolana::{Codegen, Parser};

fn codegen(source: &str) -> String {
    let result = Parser::new(source).parse();
    assert!(result.errors.is_empty());
    assert!(!result.panicked);
    Codegen::default().build(&result.program)
}

#[test]
fn boolean() {
    let out = codegen("false; true;");
    assert_snapshot!(out);
}

#[test]
fn string() {
    let out = codegen("'foo_bar123.-$#*()'");
    assert_snapshot!(out);
}

#[test]
fn variable() {
    let out = codegen("variable.foo; v.foo; temp.foo; t.foo; context.foo; c.foo;");
    assert_snapshot!(out);
}

#[test]
fn weird_variable_members() {
    let out = codegen("variable.v.temp.t.context.c.query.q.math.a.b.c");
    assert_snapshot!(out);
}

#[test]
fn binary_and_unary_operations() {
    let out =
        codegen("1 == (((2 != 3) < 4 <= 5 > 6) >= -7 + 8 - 9 * 10 / 11 || 12) && !(13 ?? 14)");
    assert_snapshot!(out)
}

#[test]
fn conditional() {
    let out = codegen("q.foo ? 1");
    assert_snapshot!(out)
}

#[test]
fn ternary() {
    let out = codegen("q.foo ? 1 : 0");
    assert_snapshot!(out)
}

#[test]
fn assignment() {
    let out = codegen("v.cow.location = 16;");
    assert_snapshot!(out)
}

#[test]
fn parenthesis_single() {
    let out = codegen("((((16))))");
    assert_snapshot!(out)
}

#[test]
fn parenthesis_complex() {
    let out = codegen("(1; 2; (3; (4; 5;);););");
    assert_snapshot!(out)
}

#[test]
fn block() {
    let out = codegen("{v.a = 0;};");
    assert_snapshot!(out)
}

#[test]
fn resource() {
    let out = codegen("geometry.foo; material.foo; texture.foo;");
    assert_snapshot!(out)
}

#[test]
fn array_access() {
    let out = codegen("array.foo[q.bar]");
    assert_snapshot!(out)
}

#[test]
fn arrow_access() {
    let out = codegen("v.foo->v.bar");
    assert_snapshot!(out)
}

#[test]
fn r#loop() {
    let out = codegen("loop(10, {v.i = v.i + 1;});");
    assert_snapshot!(out)
}

#[test]
fn for_each() {
    let out = codegen("for_each(v.a, q.foo, {v.b = v.a + 1;});");
    assert_snapshot!(out)
}

#[test]
fn keywords() {
    let out = codegen("return v.a; break; continue; this;");
    assert_snapshot!(out)
}
