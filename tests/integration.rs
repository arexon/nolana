#[cfg(test)]
mod test {

    fn format_filename(s: &str) -> String {
        // Windows sucks
        // <>:"/\|?*
        s.replace("<", "")
            .replace(">", "")
            .replace(":", "")
            .replace("/", "")
            .replace("\\", "")
            .replace("|", "")
            .replace("?", "")
            .replace("*", "")
            .replace("\n", "")
    }
    mod codegen {

        // Determines how the result is handled in the codegen_test_helper function
        enum CodegenAssert<'a> {
            /// The output should be compared to this value
            Custom(&'a str),

            /// If the output should be compared to the source.
            Source,

            /// If the output should be compared to the source without whitespace included.
            SourceNoWhitespace,
        }

        fn codegen_test_helper<'a>(source: &str, result: CodegenAssert<'a>) {
            let ret = nolana::parser::Parser::new(source).parse();
            let out = nolana::codegen::Codegen::default().build(&ret.program);
            assert!(ret.errors.is_empty());
            assert!(!ret.panicked);

            let compare_str = match result {
                CodegenAssert::Custom(r) => r,
                CodegenAssert::Source => source,
                CodegenAssert::SourceNoWhitespace => &source.to_string().replace(" ", ""),
            };

            insta::assert_snapshot!(super::format_filename(&out), compare_str);
        }

        #[test]
        fn boolean() {
            codegen_test_helper("false; true;", CodegenAssert::SourceNoWhitespace);
        }

        #[test]
        fn string() {
            codegen_test_helper("'foo_bar123.-$#*()'", CodegenAssert::Source);
        }

        #[test]
        fn variable() {
            codegen_test_helper(
                "variable.foo; v.foo; temp.foo; t.foo; context.foo; c.foo;",
                CodegenAssert::SourceNoWhitespace,
            );
        }

        #[test]
        fn weird_variable_members() {
            codegen_test_helper(
                "variable.v.temp.t.context.c.query.q.math.a.b.c",
                CodegenAssert::Source,
            );
        }

        #[test]
        fn binary_and_unary_operations() {
            codegen_test_helper(
                "1 == (((2 != 3) < 4 <= 5 > 6) >= -7 + 8 - 9 * 10 / 11 || 12) && !(13 ?? 14)",
                CodegenAssert::SourceNoWhitespace,
            );
        }

        #[test]
        fn conditional() {
            codegen_test_helper("q.foo ? 1", CodegenAssert::SourceNoWhitespace);
        }

        #[test]
        fn ternary() {
            codegen_test_helper("q.foo ? 1 : 0", CodegenAssert::SourceNoWhitespace);
        }

        #[test]
        fn assignment() {
            codegen_test_helper("v.cow.location = 16;", CodegenAssert::SourceNoWhitespace);
        }

        #[test]
        fn parenthesis_single() {
            codegen_test_helper("((((16))))", CodegenAssert::Source);
        }

        #[test]
        fn parenthesis_complex() {
            codegen_test_helper("(1; 2; (3; (4; 5;);););", CodegenAssert::SourceNoWhitespace);
        }

        #[test]
        fn block() {
            codegen_test_helper("{v.a = 0;};", CodegenAssert::SourceNoWhitespace);
        }

        #[test]
        fn resource() {
            codegen_test_helper("geometry.foo; material.foo; texture.foo;", CodegenAssert::Source);
        }

        #[test]
        fn array_access() {
            codegen_test_helper("array.foo[q.bar]", CodegenAssert::Source);
        }

        #[test]
        fn arrow_access() {
            codegen_test_helper("v.foo->v.bar", CodegenAssert::Source);
        }

        #[test]
        fn r#loop() {
            codegen_test_helper("loop(10, {v.i = v.i + 1;});", CodegenAssert::SourceNoWhitespace);
        }

        #[test]
        fn for_each() {
            codegen_test_helper(
                "for_each(v.a, q.foo, {v.b = v.a + 1;});",
                CodegenAssert::SourceNoWhitespace,
            );
        }

        #[test]
        fn keywords() {
            codegen_test_helper(
                "return v.a; break; continue; this;",
                CodegenAssert::Custom("return v.a;break;continue;this;"),
            );
        }
    }

    mod parser {
        fn test_parser_helper(source: &str) {
            let ret = nolana::parser::Parser::new(source).parse();
            insta::with_settings!({ omit_expression => true }, {
                insta::assert_debug_snapshot!(super::format_filename(source),ret);
            });
        }

        #[test]
        fn boolean_false() {
            test_parser_helper("false")
        }
        #[test]
        fn boolean_true() {
            test_parser_helper("true")
        }

        #[test]
        fn string() {
            test_parser_helper("'foo_bar123.-$#*()'")
        }

        #[test]
        fn unterminated_string() {
            test_parser_helper("'hello wor-")
        }

        #[test]
        fn variable_variable() {
            test_parser_helper("variable.foo")
        }
        #[test]
        fn variable_v() {
            test_parser_helper("v.foo")
        }
        #[test]
        fn variable_temp() {
            test_parser_helper("temp.foo")
        }
        #[test]
        fn variable_t() {
            test_parser_helper("t.foo")
        }
        #[test]
        fn variable_context() {
            test_parser_helper("context.foo")
        }
        #[test]
        fn variable_c() {
            test_parser_helper("c.foo")
        }
        #[test]
        fn weird_variable_members() {
            test_parser_helper("variable.v.temp.t.context.c.query.q.math.a.b.c")
        }

        #[test]
        fn binary_operation() {
            test_parser_helper("1 + 2 * 3")
        }
        #[test]
        fn parenthesized_binary_operation() {
            test_parser_helper("(1 + 1) * (1 + 1)")
        }
        #[test]
        fn parenthesized_binary_operation_alt() {
            test_parser_helper("((2 * 3) + 1) / 2")
        }

        #[test]
        fn negate_operation() {
            test_parser_helper("-(1 + 1)")
        }
        #[test]
        fn not_operation() {
            test_parser_helper("!(1 && 0)")
        }

        #[test]
        fn null_operation() {
            test_parser_helper("v.a ?? 1.2")
        }

        #[test]
        fn ternary_double_left() {
            test_parser_helper("q.foo ? v.bar == 13 ? 1 : 2 : 3")
        }
        #[test]
        fn ternary_double_right() {
            test_parser_helper("q.foo ? 1 : v.bar == 13 ? 2 : 3")
        }

        #[test]
        fn conditional() {
            test_parser_helper("q.foo ? 1")
        }

        #[test]
        fn assignment() {
            test_parser_helper(
                "v.cow.location.x = 204.31; v.cow.location.y = 87; v.cow.location.z = 48.933;",
            )
        }

        #[test]
        fn complex_expression() {
            test_parser_helper("0; 0; 0;")
        }

        #[test]
        fn complex_parenthesized_expression() {
            test_parser_helper("(v.a = 1; v.b = 2;);")
        }
        #[test]
        fn empty_parenthesized_expression() {
            test_parser_helper("()")
        }
        #[test]
        fn nested_parenthesis() {
            test_parser_helper("((((16))))")
        }

        #[test]
        fn block() {
            test_parser_helper("{1;};")
        }
        #[test]
        fn block_undelimited() {
            test_parser_helper("{1}")
        }

        #[test]
        fn unclosed_parenthesis_in_call() {
            test_parser_helper("q.a(1")
        }
        #[test]
        fn unclosed_parenthesis_in_parenthesized_expression() {
            test_parser_helper("(1+1")
        }

        #[test]
        fn resource_geometry() {
            test_parser_helper("geometry.foo")
        }
        #[test]
        fn resource_material() {
            test_parser_helper("material.bar")
        }
        #[test]
        fn resource_texture() {
            test_parser_helper("texture.baz")
        }

        #[test]
        fn array_access() {
            test_parser_helper("array.foo[q.bar]")
        }

        #[test]
        fn arrow_access() {
            test_parser_helper("v.foo->v.bar")
        }

        #[test]
        fn r#loop() {
            test_parser_helper("loop(10, {v.i = v.i + 1;});")
        }

        #[test]
        fn for_each() {
            test_parser_helper("for_each(v.a, q.foo, {v.b = v.a + 1;});")
        }
        #[test]
        fn for_each_wrong_first_arg() {
            test_parser_helper("for_each(1, q.foo, {v.b = v.a + 1;});")
        }

        #[test]
        fn r#return() {
            test_parser_helper("return v.a")
        }

        #[test]
        fn r#break() {
            test_parser_helper("break")
        }

        #[test]
        fn r#continue() {
            test_parser_helper("continue")
        }

        #[test]
        fn this() {
            test_parser_helper("this")
        }

        #[test]
        fn missing_semi_with_semi() {
            test_parser_helper("0; 0")
        }
        #[test]
        fn missing_semi_with_assignment() {
            test_parser_helper("v.a = 0; v.a")
        }

        #[test]
        fn semisemisemisemi() {
            test_parser_helper(
                "
    ;;;;;;; ;;;;;;; ;;;    ;;; ;;
    ;;      ;;      ;;;;  ;;;; ;;
    ;;;;;;; ;;;;;   ;; ;;;; ;; ;;
         ;; ;;      ;;  ;;  ;; ;;
    ;;;;;;; ;;;;;;; ;;      ;; ;;
    ",
            )
        }
    }

    mod semantics {
        fn test_semantics_helper(source: &str) {
            let ret = nolana::parser::Parser::new(source).parse();
            let errors = nolana::semantic::SemanticChecker::default().check(&ret.program);
            insta::with_settings!({ omit_expression => true }, {
                insta::assert_debug_snapshot!(super::format_filename(source),errors);
            });
        }

        #[test]
        fn empty_block_expression() {
            test_semantics_helper("{}")
        }

        #[test]
        fn filled_block_expression() {
            test_semantics_helper("{1;};")
        }

        #[test]
        fn illegal_string_operation_both() {
            test_semantics_helper("'foo' + 'bar'")
        }

        #[test]
        fn illegal_string_operation_left() {
            test_semantics_helper("'foo' == 1")
        }

        #[test]
        fn illegal_string_operation_right() {
            test_semantics_helper("1 + 'bar'")
        }

        #[test]
        fn unequals_string_operation() {
            test_semantics_helper("'bar' != 'bar'")
        }

        #[test]
        fn equals_string_operation() {
            test_semantics_helper("'bar' == 'bar'")
        }

        #[test]
        fn assigning_context() {
            test_semantics_helper("context.foo = 0;")
        }

        #[test]
        fn break_outside_loop() {
            test_semantics_helper("break;")
        }

        #[test]
        fn break_inside_loop() {
            test_semantics_helper("loop(1, {break;});")
        }

        #[test]
        fn continue_outside_loop() {
            test_semantics_helper("continue;")
        }

        #[test]
        fn continue_inside_loop() {
            test_semantics_helper("loop(1, {continue;});")
        }
    }
}
