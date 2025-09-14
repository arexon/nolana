use crate::{
    ast::*,
    diagnostic::Diagnostic,
    span::Span,
    traverse::{Traverse, traverse},
};

/// Traverses an AST and checks the Molang program for any semantic errors.
#[derive(Default)]
pub struct SemanticChecker {
    /// `loop` and `for_each` level.
    loop_depth: u32,
    errors: Vec<Diagnostic>,
}

impl SemanticChecker {
    pub fn check(mut self, program: &mut Program) -> Vec<Diagnostic> {
        traverse(&mut self, program);
        self.errors
    }
}

impl<'a> Traverse<'a> for SemanticChecker {
    fn enter_loop_statement(&mut self, _: &mut LoopStatement<'a>) {
        self.loop_depth += 1;
    }

    fn exit_loop_statement(&mut self, _: &mut LoopStatement<'a>) {
        self.loop_depth -= 1;
    }

    fn enter_for_each_statement(&mut self, it: &mut ForEachStatement<'a>) {
        self.loop_depth += 1;
        if it.variable.lifetime == VariableLifetime::Context {
            self.errors.push(for_each_wrong_first_arg(it.variable.span));
        }
    }

    fn exit_for_each_statement(&mut self, _: &mut ForEachStatement<'a>) {
        self.loop_depth -= 1;
    }

    fn enter_block_expression(&mut self, it: &mut BlockExpression<'a>) {
        if it.statements.is_empty() {
            self.errors.push(empty_block(it.span));
        }
    }

    fn enter_binary_expression(&mut self, it: &mut BinaryExpression<'a>) {
        use BinaryOperator::*;
        use Expression::*;
        match (&it.left, it.operator, &it.right) {
            (StringLiteral(_), op, StringLiteral(_)) if !matches!(op, Equality | Inequality) => (),
            (left, _, StringLiteral(_)) if !matches!(left, StringLiteral(_)) => (),
            (StringLiteral(_), _, right) if !matches!(right, StringLiteral(_)) => (),
            _ => return,
        }
        self.errors.push(illegal_string_binary(it.span));
    }

    fn enter_assignment_statement(&mut self, it: &mut AssignmentStatement<'a>) {
        if it.left.lifetime == VariableLifetime::Context {
            self.errors.push(context_readonly(it.span))
        }
    }

    fn enter_break_statement(&mut self, it: &mut BreakStatement) {
        if self.loop_depth == 0 {
            self.errors.push(break_outside_loop(it.span));
        }
    }

    fn enter_continue_statement(&mut self, it: &mut ContinueStatement) {
        if self.loop_depth == 0 {
            self.errors.push(continue_outside_loop(it.span));
        }
    }

    fn enter_update_expression(&mut self, it: &mut UpdateExpression<'a>) {
        if it.variable.lifetime == VariableLifetime::Context {
            self.errors.push(context_readonly(it.span))
        }
    }
}

fn empty_block(span: Span) -> Diagnostic {
    Diagnostic::error("block statement must contain at least one statement").with_label(span)
}

fn illegal_string_binary(span: Span) -> Diagnostic {
    Diagnostic::error("strings only support `==` and `!=` operators").with_label(span)
}

fn break_outside_loop(span: Span) -> Diagnostic {
    Diagnostic::error("`break` is only supported inside `loop` and `for_each` statements")
        .with_label(span)
}

fn continue_outside_loop(span: Span) -> Diagnostic {
    Diagnostic::error("`continue` is only supported inside `loop` and `for_each` statements")
        .with_label(span)
}

fn context_readonly(span: Span) -> Diagnostic {
    Diagnostic::error("`context.*` variables are read-only")
        .with_help("try using `variable.*` or `temp.*` instead")
        .with_label(span)
}

fn for_each_wrong_first_arg(span: Span) -> Diagnostic {
    Diagnostic::error("`for_each` first argument must be either `variable.*` or `temp.*`")
        .with_label(span)
}
