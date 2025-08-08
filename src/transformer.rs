use replace_with::replace_with_or_abort;

use crate::{
    ast::*,
    span::SPAN,
    traverse::{Traverse, traverse},
};

#[derive(Default)]
pub struct MolangTransformer<'src> {
    scopes: Vec<Scope<'src>>,
    program_body_transformer: ProgramBodyTransformer,
}

impl<'src> MolangTransformer<'src> {
    pub fn transform(&mut self, program: &mut Program<'src>) {
        traverse(&mut self.program_body_transformer, program);
        traverse(self, program);
    }

    fn enter_scope(&mut self) {
        self.scopes.push(Scope::default());
    }

    fn exit_scope(&mut self) -> Scope<'src> {
        self.scopes.pop().unwrap()
    }

    fn scope(&mut self) -> &mut Scope<'src> {
        self.scopes.last_mut().unwrap()
    }

    fn transform_binary_expression(&mut self, expr: &mut Expression<'src>) {
        if let Expression::Binary(bin_expr) = expr
            && bin_expr.operator.is_custom()
        {
            let scope = self.scope();
            replace_with_or_abort(expr, |expr| {
                let Expression::Binary(bin_expr) = expr else { unreachable!() };
                let BinaryExpression { left, operator, right, .. } = *bin_expr;
                match operator {
                    BinaryOperator::Remainder => math_mod_expression(left, right),
                    BinaryOperator::Exponential => math_pow_expression(left, right),
                    BinaryOperator::ShiftLeft => shift_left_expression(left, right),
                    BinaryOperator::ShiftRight => shift_right_expression(left, right),
                    BinaryOperator::BitwiseOr
                    | BinaryOperator::BitwiseAnd
                    | BinaryOperator::BitwiseXor => {
                        let index = scope.new_statements.len() + scope.statement_count - 1;
                        let (or_stmt, or_var_expr) = bitwise_operation_statement(
                            left.clone(),
                            right,
                            operator.into(),
                            index,
                        );
                        scope.new_statements.push((index, or_stmt));
                        or_var_expr
                    }
                    _ => unreachable!(),
                }
            });
        }
    }

    fn transform_assignment_statement(&mut self, stmt: &mut Statement<'src>) {
        if let Statement::Assignment(assign_stmt) = stmt
            && assign_stmt.operator.is_custom()
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

            let scope = self.scope();
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
                AssignmentOperator::BitwiseOr
                | AssignmentOperator::BitwiseAnd
                | AssignmentOperator::BitwiseXor => {
                    replace_with_or_abort(&mut assign_stmt.right, |right| {
                        // TODO(@arexon): Method to calculate this.
                        let index = scope.new_statements.len() + scope.statement_count - 1;
                        let (or_stmt, or_var_expr) = bitwise_operation_statement(
                            left.clone(),
                            right,
                            operator.into(),
                            index,
                        );
                        scope.new_statements.push((index, or_stmt));
                        or_var_expr
                    })
                }
                AssignmentOperator::Assign => unreachable!(),
            }
        }
    }

    fn transform_update_expression(&mut self, expr: &mut Expression<'src>) {
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

    fn optimize_statements(&mut self, stmts: &mut Vec<Statement<'src>>) {
        if self.program_body_transformer.needs_complex {
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

    fn add_return_statement(&mut self, program: &mut Program<'src>) {
        if self.program_body_transformer.needs_complex
            && let ProgramBody::Complex(stmts) = &mut program.body
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
}

impl<'src> Traverse<'src> for MolangTransformer<'src> {
    fn exit_program(&mut self, it: &mut Program<'src>) {
        self.add_return_statement(it);
    }

    fn enter_statements(&mut self, _: &mut Vec<Statement<'src>>) {
        self.enter_scope();
    }

    fn exit_statements(&mut self, it: &mut Vec<Statement<'src>>) {
        let scope = self.exit_scope();
        for (index, stmt) in scope.new_statements {
            it.insert(index, stmt);
        }
        self.optimize_statements(it);
    }

    fn enter_statement(&mut self, it: &mut Statement<'src>) {
        self.scope().statement_count += 1;

        self.transform_assignment_statement(it);
    }

    fn enter_expression(&mut self, it: &mut Expression<'src>) {
        self.transform_update_expression(it);
        self.transform_binary_expression(it)
    }
}

/// Some expressions will expand into entire statements. When this happens in a
/// [`ProgramBody::Simple`], we must convert it into a [`ProgramBody::Complex`].
#[derive(Default)]
struct ProgramBodyTransformer {
    is_simple: bool,
    needs_complex: bool,
}

impl<'src> Traverse<'src> for ProgramBodyTransformer {
    fn enter_program(&mut self, it: &mut Program<'src>) {
        self.is_simple = it.body.is_simple();
    }

    fn exit_program(&mut self, it: &mut Program<'src>) {
        if self.needs_complex && self.is_simple {
            replace_with_or_abort(&mut it.body, |body| {
                let ProgramBody::Simple(expr) = body else { unreachable!() };
                ProgramBody::Complex(vec![Statement::Expression(expr.into())])
            });
        }
    }

    fn enter_binary_expression(&mut self, it: &mut BinaryExpression<'src>) {
        if matches!(
            it.operator,
            BinaryOperator::BitwiseOr | BinaryOperator::BitwiseAnd | BinaryOperator::BitwiseXor
        ) && self.is_simple
        {
            self.needs_complex = true
        }
    }

    fn enter_update_expression(&mut self, _: &mut UpdateExpression<'src>) {
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
struct Scope<'src> {
    statement_count: usize,
    new_statements: Vec<(usize, Statement<'src>)>,
}

#[inline]
fn binary_expression<'src>(
    left: Expression<'src>,
    operator: BinaryOperator,
    right: Expression<'src>,
) -> Expression<'src> {
    BinaryExpression { span: SPAN, left, operator, right }.into()
}

/// `v.x * math.pow(2, math.y)`
#[inline]
fn shift_left_expression<'src>(
    left: Expression<'src>,
    right: Expression<'src>,
) -> Expression<'src> {
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
fn shift_right_expression<'src>(
    left: Expression<'src>,
    right: Expression<'src>,
) -> Expression<'src> {
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

enum BitwiseOperation {
    Or,
    And,
    Xor,
}

impl From<BinaryOperator> for BitwiseOperation {
    fn from(op: BinaryOperator) -> Self {
        match op {
            BinaryOperator::BitwiseOr => Self::Or,
            BinaryOperator::BitwiseAnd => Self::And,
            BinaryOperator::BitwiseXor => Self::Xor,
            _ => unreachable!("Bitwise Operation: {op:?}"),
        }
    }
}

impl From<AssignmentOperator> for BitwiseOperation {
    fn from(op: AssignmentOperator) -> Self {
        match op {
            AssignmentOperator::BitwiseOr => Self::Or,
            AssignmentOperator::BitwiseAnd => Self::And,
            AssignmentOperator::BitwiseXor => Self::Xor,
            _ => unreachable!("Bitwise Operation: {op:?}"),
        }
    }
}

fn bitwise_operation_statement<'src>(
    left: Expression<'src>,
    right: Expression<'src>,
    operation: BitwiseOperation,
    index: usize,
) -> (Statement<'src>, Expression<'src>) {
    let result_var = variable_expression(format!("__{index}_result"));
    let bit_var = variable_expression(format!("__{index}_bit"));
    let left_bit_var = variable_expression(format!("__{index}_left_bit"));
    let right_bit_var = variable_expression(format!("__{index}_right_bit"));
    let num_0_expr: Expression = NumericLiteral { span: SPAN, value: 0.0, raw: "0" }.into();
    let num_1_expr: Expression = NumericLiteral { span: SPAN, value: 2.0, raw: "1" }.into();
    let num_2_expr: Expression = NumericLiteral { span: SPAN, value: 2.0, raw: "2" }.into();
    let extract_bit_expr = |input_var: Expression<'src>, bit_var: Expression<'src>| {
        math_mod_expression(
            math_floor_expression(binary_expression(
                input_var,
                BinaryOperator::Division,
                math_pow_expression(num_2_expr.clone(), bit_var),
            )),
            num_2_expr.clone(),
        )
    };
    let (op_bit_var, op_expr) = match operation {
        BitwiseOperation::Or => (
            variable_expression(format!("__{index}_or_bit")),
            math_min_expression(
                num_1_expr.clone(),
                binary_expression(
                    left_bit_var.clone().into(),
                    BinaryOperator::Addition,
                    right_bit_var.clone().into(),
                ),
            ),
        ),
        BitwiseOperation::And => (
            variable_expression(format!("__{index}_and_bit")),
            binary_expression(
                left_bit_var.clone().into(),
                BinaryOperator::Multiplication,
                right_bit_var.clone().into(),
            ),
        ),
        BitwiseOperation::Xor => (
            variable_expression(format!("__{index}_xor_bit")),
            math_mod_expression(
                binary_expression(
                    left_bit_var.clone().into(),
                    BinaryOperator::Addition,
                    right_bit_var.clone().into(),
                ),
                num_2_expr.clone(),
            ),
        ),
    };

    let loop_statements = vec![
        assignment_statement(
            left_bit_var.clone(),
            extract_bit_expr(left.clone(), bit_var.clone().into()),
        ),
        assignment_statement(
            right_bit_var.clone(),
            extract_bit_expr(right.clone(), bit_var.clone().into()),
        ),
        assignment_statement(op_bit_var.clone(), op_expr),
        assignment_statement(
            result_var.clone(),
            binary_expression(
                result_var.clone().into(),
                BinaryOperator::Addition,
                binary_expression(
                    op_bit_var.into(),
                    BinaryOperator::Multiplication,
                    math_pow_expression(num_2_expr.clone(), bit_var.clone().into()),
                ),
            ),
        ),
        assignment_statement(
            bit_var.clone(),
            binary_expression(bit_var.clone().into(), BinaryOperator::Addition, num_1_expr),
        ),
    ];
    let block_statements = vec![
        assignment_statement(result_var.clone(), num_0_expr.clone()),
        assignment_statement(bit_var, num_0_expr),
        LoopStatement {
            span: SPAN,
            count: NumericLiteral { span: SPAN, value: 24.0, raw: "24" }.into(),
            block: BlockExpression { span: SPAN, statements: loop_statements },
        }
        .into(),
    ];
    (
        Expression::Block(BlockExpression { span: SPAN, statements: block_statements }.into())
            .into(),
        result_var.into(),
    )
}

#[inline]
fn variable_expression<'src>(name: String) -> VariableExpression<'src> {
    VariableExpression {
        span: SPAN,
        lifetime: VariableLifetime::Variable,
        member: VariableMember::Property { property: Identifier { span: SPAN, name: name.into() } },
    }
}

#[inline]
fn assignment_statement<'src>(
    left: VariableExpression<'src>,
    right: Expression<'src>,
) -> Statement<'src> {
    AssignmentStatement { span: SPAN, left, operator: AssignmentOperator::Assign, right }.into()
}

#[inline]
fn logical_or_assignment_statement<'src>(
    assign_stmt: AssignmentStatement<'src>,
) -> Statement<'src> {
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
fn logical_and_assignment_statement<'src>(
    assign_stmt: AssignmentStatement<'src>,
) -> Statement<'src> {
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
fn math_pow_expression<'src>(left: Expression<'src>, right: Expression<'src>) -> Expression<'src> {
    CallExpression {
        span: SPAN,
        kind: CallKind::Math,
        callee: Identifier { span: SPAN, name: "pow".into() },
        arguments: Some(vec![left, right]),
    }
    .into()
}

#[inline]
fn math_mod_expression<'src>(left: Expression<'src>, right: Expression<'src>) -> Expression<'src> {
    CallExpression {
        span: SPAN,
        kind: CallKind::Math,
        callee: Identifier { span: SPAN, name: "mod".into() },
        arguments: Some(vec![left, right]),
    }
    .into()
}

#[inline]
fn math_floor_expression<'src>(x: Expression<'src>) -> Expression<'src> {
    CallExpression {
        span: SPAN,
        kind: CallKind::Math,
        callee: Identifier { span: SPAN, name: "floor".into() },
        arguments: Some(vec![x]),
    }
    .into()
}

#[inline]
fn math_min_expression<'src>(left: Expression<'src>, right: Expression<'src>) -> Expression<'src> {
    CallExpression {
        span: SPAN,
        kind: CallKind::Math,
        callee: Identifier { span: SPAN, name: "min".into() },
        arguments: Some(vec![left, right]),
    }
    .into()
}
