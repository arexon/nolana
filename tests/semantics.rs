macro_rules! test_semantics {
    ($name:ident, $source:literal) => {
        #[test]
        fn $name() {
            let allocator = nolana::allocator::Allocator::default();
            let ret = nolana::parser::Parser::new(&allocator, $source).parse();
            let errors = nolana::semantic::SemanticChecker::default().check(&ret.program);
            insta::with_settings!({ omit_expression => true }, {
                insta::assert_debug_snapshot!(errors);
            });
        }
    };
}

test_semantics!(empty_block_expression, "{}");
test_semantics!(filled_block_expression, "{1;};");

test_semantics!(illegal_string_operation_both, "'foo' + 'bar'");
test_semantics!(illegal_string_operation_left, "'foo' == 1");
test_semantics!(illegal_string_operation_right, "1 + 'bar'");
test_semantics!(unequals_string_operation, "'bar' != 'bar'");
test_semantics!(equals_string_operation, "'bar' == 'bar'");

test_semantics!(assigning_context, "context.foo = 0;");

test_semantics!(break_outside_loop, "break;");
test_semantics!(break_inside_loop, "loop(1, {break;});");

test_semantics!(continue_outside_loop, "continue;");
test_semantics!(continue_inside_loop, "loop(1, {continue;});");
