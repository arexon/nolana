fn test_parser_helper(source: &str) -> String {
    // This debug format is being used here to remove the need for
    // insta::assert_debug_snapshot! and the need to fill in the filename,
    // as the function name should give the filename for the snapshot
    // this makes it follow the same filename layout as the rest of
    // snapshots in snapshots directory
    format!("{:?}", nolana::parser::Parser::new(source).parse())
}

#[test]
fn boolean_false() {
    let out = test_parser_helper("false");
    insta::assert_snapshot!(out)
}

#[test]
fn boolean_true() {
    let out = test_parser_helper("true");
    insta::assert_snapshot!(out)
}

#[test]
fn string() {
    let out = test_parser_helper("'foo_bar123.-$#*()'");
    insta::assert_snapshot!(out)
}

#[test]
fn unterminated_string() {
    let out = test_parser_helper("'hello wor-");
    insta::assert_snapshot!(out)
}

#[test]
fn variable_variable() {
    let out = test_parser_helper("variable.foo");
    insta::assert_snapshot!(out)
}
#[test]
fn variable_v() {
    let out = test_parser_helper("v.foo");
    insta::assert_snapshot!(out)
}
#[test]
fn variable_temp() {
    let out = test_parser_helper("temp.foo");
    insta::assert_snapshot!(out)
}
#[test]
fn variable_t() {
    let out = test_parser_helper("t.foo");
    insta::assert_snapshot!(out)
}
#[test]
fn variable_context() {
    let out = test_parser_helper("context.foo");
    insta::assert_snapshot!(out)
}
#[test]
fn variable_c() {
    let out = test_parser_helper("c.foo");
    insta::assert_snapshot!(out);
}
#[test]
fn weird_variable_members() {
    let out = test_parser_helper("variable.v.temp.t.context.c.query.q.math.a.b.c");
    insta::assert_snapshot!(out)
}

#[test]
fn binary_operation() {
    let out = test_parser_helper("1 + 2 * 3");
    insta::assert_snapshot!(out)
}
#[test]
fn parenthesized_binary_operation() {
    let out = test_parser_helper("(1 + 1) * (1 + 1)");
    insta::assert_snapshot!(out)
}
#[test]
fn parenthesized_binary_operation_alt() {
    let out = test_parser_helper("((2 * 3) + 1) / 2");
    insta::assert_snapshot!(out)
}

#[test]
fn negate_operation() {
    let out = test_parser_helper("-(1 + 1)");
    insta::assert_snapshot!(out)
}
#[test]
fn not_operation() {
    let out = test_parser_helper("!(1 && 0)");
    insta::assert_snapshot!(out)
}

#[test]
fn null_operation() {
    let out = test_parser_helper("v.a ?? 1.2");
    insta::assert_snapshot!(out)
}

#[test]
fn ternary_double_left() {
    let out = test_parser_helper("q.foo ? v.bar == 13 ? 1 : 2 : 3");
    insta::assert_snapshot!(out)
}
#[test]
fn ternary_double_right() {
    let out = test_parser_helper("q.foo ? 1 : v.bar == 13 ? 2 : 3");
    insta::assert_snapshot!(out)
}

#[test]
fn conditional() {
    let out = test_parser_helper("q.foo ? 1");
    insta::assert_snapshot!(out)
}

#[test]
fn assignment() {
    let out = test_parser_helper(
        "v.cow.location.x = 204.31; v.cow.location.y = 87; v.cow.location.z = 48.933;",
    );
    insta::assert_snapshot!(out)
}

#[test]
fn complex_expression() {
    let out = test_parser_helper("0; 0; 0;");
    insta::assert_snapshot!(out);
}

#[test]
fn complex_parenthesized_expression() {
    let out = test_parser_helper("(v.a = 1; v.b = 2;);");

    insta::assert_snapshot!(out);
}
#[test]
fn empty_parenthesized_expression() {
    let out = test_parser_helper("()");
    insta::assert_snapshot!(out);
}
#[test]
fn nested_parenthesis() {
    let out = test_parser_helper("((((16))))");

    insta::assert_snapshot!(out);
}

#[test]
fn block() {
    let out = test_parser_helper("{1;};");

    insta::assert_snapshot!(out);
}

#[test]
fn block_undelimited() {
    let out = test_parser_helper("{1}");

    insta::assert_snapshot!(out);
}

#[test]
fn unclosed_parenthesis_in_call() {
    let out = test_parser_helper("q.a(1");

    insta::assert_snapshot!(out);
}
#[test]
fn unclosed_parenthesis_in_parenthesized_expression() {
    let out = test_parser_helper("(1+1");

    insta::assert_snapshot!(out);
}

#[test]
fn resource_geometry() {
    let out = test_parser_helper("geometry.foo");

    insta::assert_snapshot!(out);
}
#[test]
fn resource_material() {
    let out = test_parser_helper("material.bar");
    insta::assert_snapshot!(out);
}
#[test]
fn resource_texture() {
    let out = test_parser_helper("texture.baz");

    insta::assert_snapshot!(out);
}

#[test]
fn array_access() {
    let out = test_parser_helper("array.foo[q.bar]");

    insta::assert_snapshot!(out);
}

#[test]
fn arrow_access() {
    let out = test_parser_helper("v.foo->v.bar");

    insta::assert_snapshot!(out);
}

#[test]
fn r#loop() {
    let out = test_parser_helper("loop(10, {v.i = v.i + 1;});");

    insta::assert_snapshot!(out);
}

#[test]
fn for_each() {
    let out = test_parser_helper("for_each(v.a, q.foo, {v.b = v.a + 1;});");
}

#[test]
fn for_each_wrong_first_arg() {
    let out = test_parser_helper("for_each(1, q.foo, {v.b = v.a + 1;});");

    insta::assert_snapshot!(out);
}

#[test]
fn r#return() {
    let out = test_parser_helper("return v.a");

    insta::assert_snapshot!(out);
}

#[test]
fn r#break() {
    let out = test_parser_helper("break");

    insta::assert_snapshot!(out);
}

#[test]
fn r#continue() {
    let out = test_parser_helper("continue");

    insta::assert_snapshot!(out);
}

#[test]
fn this() {
    let out = test_parser_helper("this");
    insta::assert_snapshot!(out);
}

#[test]
fn missing_semi_with_semi() {
    let out = test_parser_helper("0; 0");

    insta::assert_snapshot!(out);
}
#[test]
fn missing_semi_with_assignment() {
    let out = test_parser_helper("v.a = 0; v.a");

    insta::assert_snapshot!(out);
}

#[test]
fn semisemisemisemi() {
    let out = test_parser_helper(
        "
    ;;;;;;; ;;;;;;; ;;;    ;;; ;;
    ;;      ;;      ;;;;  ;;;; ;;
    ;;;;;;; ;;;;;   ;; ;;;; ;; ;;
         ;; ;;      ;;  ;;  ;; ;;
    ;;;;;;; ;;;;;;; ;;      ;; ;;
    ",
    );

    insta::assert_snapshot!(out);
}
