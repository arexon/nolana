use crate::{
    ast::*,
    diagnostic::{errors, Diagnostic},
    visit::{walk, Visit},
};

/// Traverses an AST and checks the Molang program for any semantic errors.
#[derive(Default)]
pub struct SemanticChecker {
    /// `loop` and `for_each` level.
    loop_depth: u32,
    errors: Vec<Diagnostic>,
}

impl SemanticChecker {
    pub fn check(mut self, program: &Program) -> Vec<Diagnostic> {
        walk::walk_program(&mut self, program);
        self.errors
    }
}

impl<'a> Visit<'a> for SemanticChecker {
    fn enter_loop_expression(&mut self, _: &LoopExpression<'a>) {
        self.loop_depth += 1;
    }

    fn exit_loop_expression(&mut self, _: &LoopExpression<'a>) {
        self.loop_depth -= 1;
    }

    fn enter_for_each_expression(&mut self, _: &ForEachExpression<'a>) {
        self.loop_depth += 1;
    }

    fn exit_for_each_expression(&mut self, _: &ForEachExpression<'a>) {
        self.loop_depth -= 1;
    }

    fn enter_block_expression(&mut self, it: &BlockExpression<'a>) {
        if it.expressions.is_empty() {
            self.errors.push(errors::empty_block_expression(it.span));
        }
    }

    fn enter_binary_expression(&mut self, it: &BinaryExpression<'a>) {
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

    fn enter_assignment_expression(&mut self, it: &AssignmentExpression<'a>) {
        if it.left.lifetime == VariableLifetime::Context {
            self.errors.push(errors::assigning_context(it.span))
        }
    }

    fn enter_break(&mut self, it: &Break) {
        if self.loop_depth == 0 {
            self.errors.push(errors::break_outside_loop(it.span));
        }
    }

    fn enter_continue(&mut self, it: &Continue) {
        if self.loop_depth == 0 {
            self.errors.push(errors::continue_outside_loop(it.span));
        }
    }
}
