use crate::{
    ast::*,
    diagnostic::{errors, Diagnostic},
    visit::{walk::*, Visit},
};

/// Traverses an AST and checks the Molang program for any semantic errors.
#[derive(Default)]
pub struct SemanticChecker {
    /// `loop` and `for_each` level.
    loop_depth: u32,
    errors: Vec<Diagnostic>,
}

impl SemanticChecker {
    /// Main entry point.
    pub fn check(mut self, program: &Program) -> Vec<Diagnostic> {
        self.visit_program(program);
        self.errors
    }

    const fn in_loop(kind: AstKind) -> bool {
        matches!(kind, AstKind::LoopExpression | AstKind::ForEachExpression)
    }
}

impl<'a> Visit<'a> for SemanticChecker {
    fn enter_node(&mut self, kind: AstKind) {
        if Self::in_loop(kind) {
            self.loop_depth += 1;
        }
    }

    fn leave_node(&mut self, kind: AstKind) {
        if Self::in_loop(kind) {
            self.loop_depth -= 1;
        }
    }

    fn visit_block_expression(&mut self, it: &BlockExpression<'a>) {
        if it.expressions.is_empty() {
            self.errors.push(errors::empty_block_expression(it.span));
        }
        walk_block_expression(self, it);
    }

    fn visit_binary_expression(&mut self, it: &BinaryExpression<'a>) {
        use BinaryOperator::*;
        use Expression::*;
        match (&it.left, it.operator, &it.right) {
            (StringLiteral(_), op, StringLiteral(_)) if !matches!(op, Equality | Inequality) => (),
            (left, _, StringLiteral(_)) if !matches!(left, StringLiteral(_)) => (),
            (StringLiteral(_), _, right) if !matches!(right, StringLiteral(_)) => (),
            _ => {
                walk_binary_expression(self, it);
                return;
            }
        }
        self.errors.push(errors::illegal_string_operators(it.span));
        walk_binary_expression(self, it);
    }

    fn visit_assignment_expression(&mut self, it: &AssignmentExpression<'a>) {
        if it.left.lifetime == VariableLifetime::Context {
            self.errors.push(errors::assigning_context(it.span))
        }
        walk_assignment_expression(self, it);
    }

    fn visit_break(&mut self, it: &Break) {
        if self.loop_depth == 0 {
            self.errors.push(errors::break_outside_loop(it.span));
        }
    }

    fn visit_continue(&mut self, it: &Continue) {
        if self.loop_depth == 0 {
            self.errors.push(errors::continue_outside_loop(it.span));
        }
    }
}
