use std::mem;

use crate::{
    ast::*,
    span::SPAN,
    traverse::{Traverse, traverse},
};

#[derive(Default)]
pub struct Compiler {
    binary_operators: BinaryOperators,
    assignments_operators: AssignmentOperators,
}

impl Compiler {
    pub fn compile<'a>(&mut self, program: &mut Program<'a>) {
        traverse(self, program);
    }
}

impl<'a> Traverse<'a> for Compiler {
    fn enter_expression(&mut self, it: &mut Expression<'a>) {
        self.binary_operators.enter_expression(it);
    }

    fn enter_statement(&mut self, it: &mut Statement<'a>) {
        self.assignments_operators.enter_statement(it);
    }
}

#[inline]
fn math_call<'a>(
    name: &'static str,
    left: Expression<'a>,
    right: Expression<'a>,
) -> Expression<'a> {
    Expression::Call(
        CallExpression {
            span: SPAN,
            kind: CallKind::Math,
            callee: Identifier { span: SPAN, name: name.into() },
            arguments: Some(vec![left, right]),
        }
        .into(),
    )
}

#[derive(Default)]
struct BinaryOperators;

impl BinaryOperators {
    /// `v.left ** v.right;` -> `math.pow(v.left, v.right);`
    /// `v.left % v.right;` -> `math.mod(v.left, v.right);`
    #[inline]
    fn convert_binary_expression(&self, expr: &mut Expression<'_>) {
        let Expression::Binary(bin_expr) = expr else { unreachable!() };
        let math_fn_name = match bin_expr.operator {
            BinaryOperator::Remainder => "mod",
            BinaryOperator::Exponential => "pow",
            _ => return,
        };
        let bin_expr = mem::take(bin_expr);
        *expr = math_call(math_fn_name, bin_expr.left, bin_expr.right)
    }
}

impl Traverse<'_> for BinaryOperators {
    fn enter_expression(&mut self, it: &mut Expression<'_>) {
        if let Expression::Binary(_) = it {
            self.convert_binary_expression(it)
        }
    }
}

#[derive(Default)]
struct AssignmentOperators;

impl AssignmentOperators {
    /// `v.left += v.right;` -> `v.left = v.left + v.right;`
    /// `v.left -= v.right;` -> `v.left = v.left - v.right;`
    /// `v.left *= v.right;` -> `v.left = v.left * v.right;`
    /// `v.left /= v.right;` -> `v.left = v.left / v.right;`
    /// `v.left **= v.right;` -> `v.left = math.pow(v.left, v.right);`
    /// `v.left %= v.right;` -> `v.left = math.mod(v.left, v.right);`
    #[inline]
    fn convert_assignment_statement(&self, stmt: &mut Statement<'_>) {
        enum MathOrOp {
            Math(&'static str),
            Op(BinaryOperator),
        }

        let Statement::Assignment(assign_stmt) = stmt else { unreachable!() };
        dbg!(assign_stmt.operator);
        let math_or_op = match assign_stmt.operator {
            AssignmentOperator::Remainder => MathOrOp::Math("mod"),
            AssignmentOperator::Exponential => MathOrOp::Math("pow"),
            AssignmentOperator::Addition => MathOrOp::Op(BinaryOperator::Addition),
            AssignmentOperator::Subtraction => MathOrOp::Op(BinaryOperator::Subtraction),
            AssignmentOperator::Multiplication => MathOrOp::Op(BinaryOperator::Multiplication),
            AssignmentOperator::Division => MathOrOp::Op(BinaryOperator::Division),
            AssignmentOperator::Assign => return,
        };
        assign_stmt.operator = AssignmentOperator::Assign;
        match math_or_op {
            MathOrOp::Math(math_fn_name) => {
                let assign_expr = mem::take(assign_stmt);
                assign_stmt.right = math_call(
                    math_fn_name,
                    Expression::Variable(assign_expr.left.into()),
                    assign_expr.right,
                );
            }
            MathOrOp::Op(bin_op) => {
                assign_stmt.right = Expression::Binary(
                    BinaryExpression {
                        span: SPAN,
                        left: Expression::Variable(assign_stmt.left.clone().into()),
                        operator: bin_op,
                        right: mem::take(&mut assign_stmt.right),
                    }
                    .into(),
                );
            }
        }
    }
}

impl Traverse<'_> for AssignmentOperators {
    fn enter_statement(&mut self, it: &mut Statement<'_>) {
        if let Statement::Assignment(_) = it {
            self.convert_assignment_statement(it);
        }
    }
}
