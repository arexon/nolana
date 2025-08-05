use crate::{
    ast::*,
    diagnostic::{Diagnostic, errors},
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

    fn enter_for_each_statement(&mut self, _: &mut ForEachStatement<'a>) {
        self.loop_depth += 1;
    }

    fn exit_for_each_statement(&mut self, _: &mut ForEachStatement<'a>) {
        self.loop_depth -= 1;
    }

    fn enter_block_expression(&mut self, it: &mut BlockExpression<'a>) {
        if it.statements.is_empty() {
            self.errors.push(errors::empty_block_expression(it.span));
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
        self.errors.push(errors::illegal_string_operators(it.span));
    }

    fn enter_assignment_statement(&mut self, it: &mut AssignmentStatement<'a>) {
        if it.left.lifetime == VariableLifetime::Context {
            self.errors.push(errors::assigning_context(it.span))
        }
    }

    fn enter_break_statement(&mut self, it: &mut BreakStatement) {
        if self.loop_depth == 0 {
            self.errors.push(errors::break_outside_loop(it.span));
        }
    }

    fn enter_continue_statement(&mut self, it: &mut ContinueStatement) {
        if self.loop_depth == 0 {
            self.errors.push(errors::continue_outside_loop(it.span));
        }
    }
}
