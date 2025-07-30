#[test]
fn boolean() {
    let ret = nolana::parser::Parser::new("false; true;").parse();
    let out = nolana::codegen::Codegen::default().build(&ret.program);
    assert!(ret.errors.is_empty());
    assert!(!ret.panicked);
    insta::assert_snapshot!(out, @"false;true;");
}

#[test]
fn string() {
    let ret = nolana::parser::Parser::new("'foo_bar123.-$#*()'").parse();
    let out = nolana::codegen::Codegen::default().build(&ret.program);
    assert!(ret.errors.is_empty());
    assert!(!ret.panicked);
    insta::assert_snapshot!(out, @"'foo_bar123.-$#*()'");
}

#[test]
fn variable() {
    let ret =
        nolana::parser::Parser::new("variable.foo; v.foo; temp.foo; t.foo; context.foo; c.foo;")
            .parse();
    let out = nolana::codegen::Codegen::default().build(&ret.program);
    assert!(ret.errors.is_empty());
    assert!(!ret.panicked);
    insta::assert_snapshot!(out, @"v.foo;v.foo;t.foo;t.foo;c.foo;c.foo;");
}

#[test]
fn weird_variable_members() {
    let ret = nolana::parser::Parser::new("variable.v.temp.t.context.c.query.q.math.a.b.c").parse();
    let out = nolana::codegen::Codegen::default().build(&ret.program);
    assert!(ret.errors.is_empty());
    assert!(!ret.panicked);
    insta::assert_snapshot!(out, @"v.v.temp.t.context.c.query.q.math.a.b.c");
}

#[test]
fn binary_and_unary_operations() {
    let ret = nolana::parser::Parser::new(
        "1 == (((2 != 3) < 4 <= 5 > 6) >= -7 + 8 - 9 * 10 / 11 || 12) && !(13 ?? 14)",
    )
    .parse();
    let out = nolana::codegen::Codegen::default().build(&ret.program);
    assert!(ret.errors.is_empty());
    assert!(!ret.panicked);
    insta::assert_snapshot!(out, @"1==(((2!=3)<4<=5>6)>=-7+8-9*10/11||12)&&!(13??14)");
}

#[test]
fn conditional() {
    let ret = nolana::parser::Parser::new("q.foo ? 1").parse();
    let out = nolana::codegen::Codegen::default().build(&ret.program);
    assert!(ret.errors.is_empty());
    assert!(!ret.panicked);
    insta::assert_snapshot!(out, @"q.foo?1");
}

#[test]
fn ternary() {
    let ret = nolana::parser::Parser::new("q.foo ? 1 : 0").parse();
    let out = nolana::codegen::Codegen::default().build(&ret.program);
    assert!(ret.errors.is_empty());
    assert!(!ret.panicked);
    insta::assert_snapshot!(out, @"q.foo?1:0");
}

#[test]
fn assignment() {
    let ret = nolana::parser::Parser::new("v.cow.location = 16;").parse();
    let out = nolana::codegen::Codegen::default().build(&ret.program);
    assert!(ret.errors.is_empty());
    assert!(!ret.panicked);
    insta::assert_snapshot!(out, @"v.cow.location=16;");
}

#[test]
fn parenthesis_single() {
    let ret = nolana::parser::Parser::new("((((16))))").parse();
    let out = nolana::codegen::Codegen::default().build(&ret.program);
    assert!(ret.errors.is_empty());
    assert!(!ret.panicked);
    insta::assert_snapshot!(out, @"((((16))))");
}

#[test]
fn parenthesis_complex() {
    let ret = nolana::parser::Parser::new("(1; 2; (3; (4; 5;);););").parse();
    let out = nolana::codegen::Codegen::default().build(&ret.program);
    assert!(ret.errors.is_empty());
    assert!(!ret.panicked);
    insta::assert_snapshot!(out, @"(1;2;(3;(4;5;);););");
}

#[test]
fn block() {
    let ret = nolana::parser::Parser::new("{v.a = 0;};").parse();
    let out = nolana::codegen::Codegen::default().build(&ret.program);
    assert!(ret.errors.is_empty());
    assert!(!ret.panicked);
    insta::assert_snapshot!(out, @"{v.a=0;};");
}

#[test]
fn resource() {
    let ret = nolana::parser::Parser::new("geometry.foo; material.foo; texture.foo;").parse();
    let out = nolana::codegen::Codegen::default().build(&ret.program);
    assert!(ret.errors.is_empty());
    assert!(!ret.panicked);
    insta::assert_snapshot!(out, @"geometry.foo;material.foo;texture.foo;");
}

#[test]
fn array_access() {
    let ret = nolana::parser::Parser::new("array.foo[q.bar]").parse();
    let out = nolana::codegen::Codegen::default().build(&ret.program);
    assert!(ret.errors.is_empty());
    assert!(!ret.panicked);
    insta::assert_snapshot!(out, @"array.foo[q.bar]");
}

#[test]
fn arrow_access() {
    let ret = nolana::parser::Parser::new("v.foo->v.bar").parse();
    let out = nolana::codegen::Codegen::default().build(&ret.program);
    assert!(ret.errors.is_empty());
    assert!(!ret.panicked);
    insta::assert_snapshot!(out, @"v.foo->v.bar");
}

#[test]
fn r#loop() {
    let ret = nolana::parser::Parser::new("loop(10, {v.i = v.i + 1;});").parse();
    let out = nolana::codegen::Codegen::default().build(&ret.program);
    assert!(ret.errors.is_empty());
    assert!(!ret.panicked);
    insta::assert_snapshot!(out, @"loop(10,{v.i=v.i+1;});");
}

#[test]
fn for_each() {
    let ret = nolana::parser::Parser::new("for_each(v.a, q.foo, {v.b = v.a + 1;});").parse();
    let out = nolana::codegen::Codegen::default().build(&ret.program);
    assert!(ret.errors.is_empty());
    assert!(!ret.panicked);
    insta::assert_snapshot!(out, @"for_each(v.a,q.foo,{v.b=v.a+1;});");
}

#[test]
fn keywords() {
    let ret = nolana::parser::Parser::new("return v.a; break; continue; this;").parse();
    let out = nolana::codegen::Codegen::default().build(&ret.program);
    assert!(ret.errors.is_empty());
    assert!(!ret.panicked);
    insta::assert_snapshot!(out, @"return v.a;break;continue;this;");
}
