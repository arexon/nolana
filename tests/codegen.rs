macro_rules! test_codegen {
    ($name:ident, $source:literal, @$result:literal $(,)?) => {
        #[test]
        fn $name() {
            let ret = nolana::parser::Parser::new($source).parse();
            let out = nolana::codegen::Codegen::default().build(&ret.program);
            assert!(ret.errors.is_empty());
            assert!(!ret.panicked);
            insta::assert_snapshot!(out, @$result);
        }
    };
}

test_codegen!(boolean, "false; true;", @"false;true;");
test_codegen!(string, "'foo_bar123.-$#*()'", @"'foo_bar123.-$#*()'");
test_codegen!(
    variable,
    "variable.foo; v.foo; temp.foo; t.foo; context.foo; c.foo;",
    @"v.foo;v.foo;t.foo;t.foo;c.foo;c.foo;",
);
test_codegen!(
    weird_variable_members,
    "variable.v.temp.t.context.c.query.q.math.a.b.c",
    @"v.v.temp.t.context.c.query.q.math.a.b.c",
);

test_codegen!(
    binary_and_unary_operations,
    "1 == (((2 != 3) < 4 <= 5 > 6) >= -7 + 8 - 9 * 10 / 11 || 12) && !(13 ?? 14)",
    @"1==(((2!=3)<4<=5>6)>=-7+8-9*10/11||12)&&!(13??14)",
);

test_codegen!(conditional, "q.foo ? 1", @"q.foo?1");

test_codegen!(ternary, "q.foo ? 1 : 0", @"q.foo?1:0");

test_codegen!(
    assignment,
    "v.cow.location = 16;",
    @"v.cow.location=16;",
);

test_codegen!(parenthesis_single, "((((16))))", @"((((16))))");
test_codegen!(parenthesis_complex, "(1; 2; (3; (4; 5;);););", @"(1;2;(3;(4;5;);););");

test_codegen!(block, "{v.a = 0;};", @"{v.a=0;};");

test_codegen!(
    resource,
    "geometry.foo; material.foo; texture.foo;",
    @"geometry.foo;material.foo;texture.foo;",
);

test_codegen!(array_access, "array.foo[q.bar]", @"array.foo[q.bar]");

test_codegen!(arrow_access, "v.foo->v.bar", @"v.foo->v.bar");

test_codegen!(
    r#loop,
    "loop(10, {v.i = v.i + 1;});",
    @"loop(10,{v.i=v.i+1;});",
);

test_codegen!(
    for_each,
    "for_each(v.a, q.foo, {v.b = v.a + 1;});",
    @"for_each(v.a,q.foo,{v.b=v.a+1;});",
);

test_codegen!(
    keywords,
    "return v.a; break; continue; this;",
    @"return v.a;break;continue;this;",
);
