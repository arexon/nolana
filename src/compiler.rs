use replace_with::replace_with_or_abort;

use crate::{
    ast::*,
    span::SPAN,
    traverse::{Traverse, traverse},
};

#[derive(Default)]
pub struct Compiler<'a> {
    scopes: Vec<Scope<'a>>,
    /// See [`ProgramBodyTransformer`] for details.
    has_transformed_into_complex: bool,
}

impl<'a> Compiler<'a> {
    pub fn compile(&mut self, program: &mut Program<'a>) {
        let mut program_body_ts = ProgramBodyTransformer::default();
        traverse(&mut program_body_ts, program);
        self.has_transformed_into_complex = program_body_ts.needs_complex;
        traverse(self, program);
    }

    fn enter_scope(&mut self) {
        self.scopes.push(Scope::default());
    }

    fn exit_scope(&mut self) -> Scope<'a> {
        self.scopes.pop().unwrap()
    }

    fn scope(&mut self) -> &mut Scope<'a> {
        self.scopes.last_mut().unwrap()
    }

    fn compile_binary_expression(&self, expr: &mut Expression<'a>) {
        if let Expression::Binary(bin_expr) = expr
            && matches!(
                bin_expr.operator,
                BinaryOperator::Remainder
                    | BinaryOperator::Exponential
                    | BinaryOperator::ShiftLeft
                    | BinaryOperator::ShiftRight
            )
        {
            replace_with_or_abort(expr, |expr| {
                let Expression::Binary(bin_expr) = expr else { unreachable!() };
                let BinaryExpression { left, operator, right, .. } = *bin_expr;
                match operator {
                    BinaryOperator::Remainder => math_mod_expression(left, right),
                    BinaryOperator::Exponential => math_pow_expression(left, right),
                    BinaryOperator::ShiftLeft => shift_left_expression(left, right),
                    BinaryOperator::ShiftRight => shift_right_expression(left, right),
                    _ => unreachable!(),
                }
            });
        }
    }

    fn compile_assignment_statement(&self, stmt: &mut Statement<'a>) {
        if let Statement::Assignment(assign_stmt) = stmt
            && assign_stmt.operator != AssignmentOperator::Assign
        {
            let mut left = assign_stmt.left.clone().into();
            if !assign_stmt.left.is_struct() {
                left = binary_expression(
                    assign_stmt.left.clone().into(),
                    BinaryOperator::Coalesce,
                    NumericLiteral { span: SPAN, value: 0.0, raw: "0" }.into(),
                );
            }

            let operator = assign_stmt.operator;
            assign_stmt.operator = AssignmentOperator::Assign;

            match operator {
                AssignmentOperator::Addition
                | AssignmentOperator::Subtraction
                | AssignmentOperator::Multiplication
                | AssignmentOperator::Division => {
                    replace_with_or_abort(&mut assign_stmt.right, |right| {
                        binary_expression(left, operator.into(), right)
                    })
                }
                AssignmentOperator::Exponential => {
                    replace_with_or_abort(&mut assign_stmt.right, |right| {
                        math_pow_expression(left, right)
                    })
                }
                AssignmentOperator::Remainder => {
                    replace_with_or_abort(&mut assign_stmt.right, |right| {
                        math_mod_expression(left, right)
                    })
                }
                AssignmentOperator::LogicalOr => replace_with_or_abort(stmt, |stmt| {
                    let Statement::Assignment(assign_stmt) = stmt else { unreachable!() };
                    logical_or_assignment_statement(*assign_stmt)
                }),
                AssignmentOperator::LogicalAnd => replace_with_or_abort(stmt, |stmt| {
                    let Statement::Assignment(assign_stmt) = stmt else { unreachable!() };
                    logical_and_assignment_statement(*assign_stmt)
                }),
                AssignmentOperator::ShiftLeft => {
                    replace_with_or_abort(&mut assign_stmt.right, |right| {
                        shift_left_expression(left, right)
                    })
                }
                AssignmentOperator::ShiftRight => {
                    replace_with_or_abort(&mut assign_stmt.right, |right| {
                        shift_right_expression(left, right)
                    })
                }
                AssignmentOperator::Assign => unreachable!(),
            }
        }
    }

    fn compile_update_expression(&mut self, expr: &mut Expression<'a>) {
        let Expression::Update(update_expr) = expr else { return };

        let scope = self.scope();
        let update_stmt = AssignmentStatement {
            span: SPAN,
            left: update_expr.variable.clone(),
            operator: AssignmentOperator::Assign,
            right: BinaryExpression {
                span: SPAN,
                left: update_expr.variable.clone().into(),
                operator: update_expr.operator.into(),
                right: NumericLiteral { span: SPAN, value: 1.0, raw: "1" }.into(),
            }
            .into(),
        }
        .into();
        let index = scope.new_statements.len() + scope.statement_count - 1;
        scope.new_statements.push((index, update_stmt));

        replace_with_or_abort(expr, |expr| {
            let Expression::Update(update_expr) = expr else { unreachable!() };
            update_expr.variable.into()
        });
    }

    fn optimize_statements(&mut self, stmts: &mut Vec<Statement<'a>>) {
        if self.has_transformed_into_complex {
            return;
        }
        for stmt in stmts {
            if let Statement::Expression(expr) = stmt
                && matches!(expr.as_ref(), Expression::Variable(_))
            {
                *stmt = EmptyStatement { span: SPAN }.into()
            }
        }
    }
}

impl<'a> Traverse<'a> for Compiler<'a> {
    fn exit_program(&mut self, it: &mut Program<'a>) {
        if self.has_transformed_into_complex
            && let ProgramBody::Complex(stmts) = &mut it.body
        {
            replace_with_or_abort(
                stmts.last_mut().expect("must have at least two statements"),
                |stmt| {
                    let Statement::Expression(expr) = stmt else {
                        unreachable!(
                            "simple to complex transition implies the last statement is an expression"
                        );
                    };
                    ReturnStatement { span: SPAN, argument: *expr }.into()
                },
            );
        }
    }

    fn enter_statements(&mut self, _: &mut Vec<Statement<'a>>) {
        self.enter_scope();
    }

    fn exit_statements(&mut self, it: &mut Vec<Statement<'a>>) {
        let scope = self.exit_scope();
        for (index, stmt) in scope.new_statements {
            it.insert(index, stmt);
        }
        self.optimize_statements(it);
    }

    fn enter_statement(&mut self, it: &mut Statement<'a>) {
        self.scope().statement_count += 1;

        self.compile_assignment_statement(it);
    }

    fn enter_expression(&mut self, it: &mut Expression<'a>) {
        self.compile_update_expression(it);
        self.compile_binary_expression(it)
    }
}

/// Some expressions will expand into entire statements. When this happens in a
/// [`ProgramBody::Simple`], we must convert it into a [`ProgramBody::Complex`].
#[derive(Default)]
struct ProgramBodyTransformer {
    is_simple: bool,
    needs_complex: bool,
}

impl<'a> Traverse<'a> for ProgramBodyTransformer {
    fn enter_program(&mut self, it: &mut Program<'a>) {
        self.is_simple = it.body.is_simple();
    }

    fn exit_program(&mut self, it: &mut Program<'a>) {
        if self.needs_complex && self.is_simple {
            replace_with_or_abort(&mut it.body, |body| {
                let ProgramBody::Simple(expr) = body else { unreachable!() };
                ProgramBody::Complex(vec![Statement::Expression(expr.into())])
            });
        }
    }

    fn enter_update_expression(&mut self, _: &mut UpdateExpression<'a>) {
        if self.is_simple {
            self.needs_complex = true;
        }
    }
}

/// Contextual info about the current scope.
///
/// Mainly stores extra statements to be added to the statement list upon
/// exiting the scope.
#[derive(Default)]
struct Scope<'a> {
    statement_count: usize,
    new_statements: Vec<(usize, Statement<'a>)>,
}

#[inline]
fn binary_expression<'a>(
    left: Expression<'a>,
    operator: BinaryOperator,
    right: Expression<'a>,
) -> Expression<'a> {
    BinaryExpression { span: SPAN, left, operator, right }.into()
}

/// `v.x * math.pow(2, math.y)`
#[inline]
fn shift_left_expression<'a>(left: Expression<'a>, right: Expression<'a>) -> Expression<'a> {
    BinaryExpression {
        span: SPAN,
        left,
        operator: BinaryOperator::Multiplication,
        right: math_pow_expression(
            NumericLiteral { span: SPAN, value: 2.0, raw: "2" }.into(),
            right,
        ),
    }
    .into()
}

/// `math.floor(v.x / math.pow(2, math.y))`
#[inline]
fn shift_right_expression<'a>(left: Expression<'a>, right: Expression<'a>) -> Expression<'a> {
    math_floor_expression(
        BinaryExpression {
            span: SPAN,
            left,
            operator: BinaryOperator::Division,
            right: math_pow_expression(
                NumericLiteral { span: SPAN, value: 2.0, raw: "2" }.into(),
                right,
            ),
        }
        .into(),
    )
}

#[inline]
fn logical_or_assignment_statement<'a>(assign_stmt: AssignmentStatement<'a>) -> Statement<'a> {
    Expression::Conditional(
        ConditionalExpression {
            span: SPAN,
            test: UnaryExpression {
                span: SPAN,
                operator: UnaryOperator::Not,
                argument: assign_stmt.left.clone().into(),
            }
            .into(),
            consequent: BlockExpression { span: SPAN, statements: vec![assign_stmt.into()] }.into(),
        }
        .into(),
    )
    .into()
}

#[inline]
fn logical_and_assignment_statement<'a>(assign_stmt: AssignmentStatement<'a>) -> Statement<'a> {
    Expression::Conditional(
        ConditionalExpression {
            span: SPAN,
            test: assign_stmt.left.clone().into(),
            consequent: BlockExpression { span: SPAN, statements: vec![assign_stmt.into()] }.into(),
        }
        .into(),
    )
    .into()
}

#[inline]
fn math_pow_expression<'a>(left: Expression<'a>, right: Expression<'a>) -> Expression<'a> {
    CallExpression {
        span: SPAN,
        kind: CallKind::Math,
        callee: Identifier { span: SPAN, name: "pow".into() },
        arguments: Some(vec![left, right]),
    }
    .into()
}

#[inline]
fn math_mod_expression<'a>(left: Expression<'a>, right: Expression<'a>) -> Expression<'a> {
    CallExpression {
        span: SPAN,
        kind: CallKind::Math,
        callee: Identifier { span: SPAN, name: "mod".into() },
        arguments: Some(vec![left, right]),
    }
    .into()
}

#[inline]
fn math_floor_expression<'a>(x: Expression<'a>) -> Expression<'a> {
    CallExpression {
        span: SPAN,
        kind: CallKind::Math,
        callee: Identifier { span: SPAN, name: "floor".into() },
        arguments: Some(vec![x]),
    }
    .into()
}
