use std::borrow::Cow;

use crate::{ast::*, ir::*};

#[derive(Default)]
pub struct Lowerer<'src> {
    uid: usize,
    statements: Vec<IrStatement<'src>>,
    is_complex: bool,
}

impl<'src> Lowerer<'src> {
    pub fn lower(mut self, program: &Program<'src>) -> IrProgram<'src> {
        let body = match &program.body {
            ProgramBody::Simple(expr) => {
                if let Some(result) = self.lower_expression(expr) {
                    self.is_complex = self.is_complex || !self.statements.is_empty();
                    self.statements.push(if self.is_complex {
                        IrStatement::Return(result.into())
                    } else {
                        IrStatement::Expression(result.into())
                    });
                }
                self.statements
            }
            ProgramBody::Complex(stmts) => {
                self.is_complex = true;
                self.lower_statements(stmts);
                self.statements
            }
            ProgramBody::Empty => Vec::new(),
        };

        IrProgram { body, is_complex: self.is_complex }
    }

    fn lower_statements(&mut self, stmts: &[Statement<'src>]) {
        for stmt in stmts {
            let Some(result) = self.lower_statement(stmt) else { continue };
            self.statements.push(result);
        }
    }

    fn lower_statement(&mut self, stmt: &Statement<'src>) -> Option<IrStatement<'src>> {
        match stmt {
            Statement::Expression(stmt) => self.lower_expression(stmt).map(Into::into),
            Statement::Assignment(stmt) => self.lower_assignment_statement(stmt).map(Into::into),
            Statement::Loop(stmt) => self.lower_loop_statement(stmt),
            Statement::ForEach(stmt) => self.lower_for_each_statement(stmt),
            Statement::Update(stmt) => Some(self.lower_update_statement(stmt).into()),
            Statement::Return(stmt) => self.lower_return_statement(stmt),
            Statement::Break(_) => Some(IrStatement::Break),
            Statement::Continue(_) => Some(IrStatement::Continue),
            Statement::Empty(_) => None,
        }
    }

    fn lower_expression(&mut self, expr: &Expression<'src>) -> Option<IrExpression<'src>> {
        match expr {
            Expression::NumericLiteral(expr) => Some(IrExpression::Number(expr.value)),
            Expression::BooleanLiteral(expr) => Some(IrExpression::Boolean(expr.value)),
            Expression::StringLiteral(expr) => Some(IrExpression::String(expr.value)),
            Expression::Variable(expr) => Some(self.lower_variable_expression(expr).into()),
            Expression::Parenthesized(expr) => self.lower_parenthesized_expression(expr),
            Expression::Block(expr) => Some(self.lower_block_expression(expr)),
            Expression::Binary(expr) => self.lower_binary_expression(expr),
            Expression::Unary(expr) => self.lower_unary_expression(expr),
            Expression::Ternary(expr) => self.lower_ternary_expression(expr),
            Expression::Conditional(expr) => self.lower_conditional_expression(expr),
            Expression::Resource(expr) => self.lower_resource_expression(expr),
            Expression::ArrayAccess(expr) => self.lower_array_access_expression(expr),
            Expression::ArrowAccess(expr) => self.lower_arrow_access_expression(expr),
            Expression::Call(expr) => Some(self.lower_call_expression(expr)),
            Expression::Function(expr) => None,
            Expression::This(_) => Some(IrExpression::This),
        }
    }

    fn lower_assignment_statement(
        &mut self,
        stmt: &AssignmentStatement<'src>,
    ) -> Option<IrAssignment<'src>> {
        let target = self.lower_variable_expression(&stmt.left);
        let target_expr: IrExpression = target.clone().into();
        let value = self.lower_expression(&stmt.right)?;
        let mut assignment = IrAssignment { target, value: value.clone() };

        assignment.value = match stmt.operator {
            AssignmentOperator::Assign => value,
            AssignmentOperator::Addition => IrBinary {
                left: target_expr.clone(),
                operator: IrBinaryOperator::Addition,
                right: value,
            }
            .into(),
            AssignmentOperator::Subtraction => IrBinary {
                left: target_expr.clone(),
                operator: IrBinaryOperator::Subtraction,
                right: value,
            }
            .into(),
            AssignmentOperator::Multiplication => IrBinary {
                left: target_expr.clone(),
                operator: IrBinaryOperator::Multiplication,
                right: value,
            }
            .into(),
            AssignmentOperator::Division => IrBinary {
                left: target_expr.clone(),
                operator: IrBinaryOperator::Division,
                right: value,
            }
            .into(),
            AssignmentOperator::Exponential => ir_math("pow", &[target_expr, value]),
            AssignmentOperator::Remainder => ir_math("mod", &[target_expr, value]),
            AssignmentOperator::LogicalOr => {
                self.statements.push(
                    IrExpression::Conditional(
                        IrConditional {
                            test: IrUnary { operator: IrUnaryOperator::Not, argument: target_expr }
                                .into(),
                            consequent: IrExpression::Block(vec![assignment.clone().into()]),
                        }
                        .into(),
                    )
                    .into(),
                );
                return None;
            }
            AssignmentOperator::LogicalAnd => {
                self.statements.push(
                    IrExpression::Conditional(
                        IrConditional {
                            test: target_expr,
                            consequent: IrExpression::Block(vec![assignment.clone().into()]),
                        }
                        .into(),
                    )
                    .into(),
                );
                return None;
            }
            AssignmentOperator::ShiftLeft => self.lower_left_shift_operation(target_expr, value),
            AssignmentOperator::ShiftRight => self.lower_right_shift_operation(target_expr, value),
            AssignmentOperator::BitwiseOr => self
                .lower_bitwise_operation(BitwiseOperation::Or { left: target_expr, right: value }),
            AssignmentOperator::BitwiseAnd => self
                .lower_bitwise_operation(BitwiseOperation::And { left: target_expr, right: value }),
            AssignmentOperator::BitwiseXor => self
                .lower_bitwise_operation(BitwiseOperation::Xor { left: target_expr, right: value }),
        };

        Some(assignment)
    }

    fn lower_loop_statement(&mut self, stmt: &LoopStatement<'src>) -> Option<IrStatement<'src>> {
        let count = self.lower_expression(&stmt.count)?;
        let body =
            stmt.block.statements.iter().filter_map(|stmt| self.lower_statement(stmt)).collect();
        Some(IrLoop { count, body }.into())
    }

    fn lower_for_each_statement(
        &mut self,
        stmt: &ForEachStatement<'src>,
    ) -> Option<IrStatement<'src>> {
        let variable = self.lower_variable_expression(&stmt.variable);
        let array = self.lower_expression(&stmt.array)?;
        let body =
            stmt.block.statements.iter().filter_map(|stmt| self.lower_statement(stmt)).collect();
        Some(IrForEach { variable, array, body }.into())
    }

    fn lower_update_statement(&mut self, expr: &UpdateStatement<'src>) -> IrExpression<'src> {
        let var = self.lower_variable_expression(&expr.variable);
        let update_stmt = IrAssignment {
            target: var.clone(),
            value: IrBinary {
                left: var.clone().into(),
                operator: match expr.operator {
                    UpdateOperator::Increment => IrBinaryOperator::Addition,
                    UpdateOperator::Decrement => IrBinaryOperator::Subtraction,
                },
                right: IrExpression::Number(1.0),
            }
            .into(),
        };
        IrExpression::Block(vec![update_stmt.into()])
    }

    fn lower_return_statement(
        &mut self,
        expr: &ReturnStatement<'src>,
    ) -> Option<IrStatement<'src>> {
        let arg = self.lower_expression(&expr.argument)?;
        Some(IrStatement::Return(arg.into()))
    }

    fn lower_variable_expression(&mut self, expr: &VariableExpression<'src>) -> IrVariable<'src> {
        let name = self.lower_variable_member(&expr.member);
        let scope = match expr.scope {
            VariableScope::Temporary => IrVariableScope::Temporary,
            VariableScope::Variable => IrVariableScope::Variable,
            VariableScope::Context => IrVariableScope::Context,
            VariableScope::Math => todo!("{expr:#?}"),
            VariableScope::Query => todo!("{expr:#?}"),
        };
        IrVariable { scope, name }
    }

    fn lower_variable_member(&self, member: &VariableMember<'src>) -> Vec<Cow<'src, str>> {
        let mut name = Vec::new();
        let mut stack = vec![member];
        while let Some(member) = stack.pop() {
            match member {
                VariableMember::Object { object, property } => {
                    name.push(property.name.clone());
                    stack.push(object);
                }
                VariableMember::Property { property } => {
                    name.push(property.name.clone());
                }
            }
        }
        name.reverse();
        name
    }

    fn lower_parenthesized_expression(
        &mut self,
        expr: &ParenthesizedExpression<'src>,
    ) -> Option<IrExpression<'src>> {
        match &expr.body {
            ParenthesizedBody::Single(expr) => {
                self.lower_expression(expr).map(|expr| IrExpression::Parenthesized(expr.into()))
            }
            ParenthesizedBody::Multiple(stmts) => {
                self.lower_statements(stmts);
                None
            }
        }
    }

    fn lower_binary_expression(
        &mut self,
        expr: &BinaryExpression<'src>,
    ) -> Option<IrExpression<'src>> {
        let left = self.lower_expression(&expr.left)?;
        let right = self.lower_expression(&expr.right)?;

        let operator = match expr.operator {
            BinaryOperator::Equality => IrBinaryOperator::Equality,
            BinaryOperator::Inequality => IrBinaryOperator::Inequality,
            BinaryOperator::LessThan => IrBinaryOperator::LessThan,
            BinaryOperator::LessEqualThan => IrBinaryOperator::LessEqualThan,
            BinaryOperator::GreaterThan => IrBinaryOperator::GreaterThan,
            BinaryOperator::GreaterEqualThan => IrBinaryOperator::GreaterEqualThan,
            BinaryOperator::Addition => IrBinaryOperator::Addition,
            BinaryOperator::Subtraction => IrBinaryOperator::Subtraction,
            BinaryOperator::Multiplication => IrBinaryOperator::Multiplication,
            BinaryOperator::Division => IrBinaryOperator::Division,
            BinaryOperator::Or => IrBinaryOperator::Or,
            BinaryOperator::And => IrBinaryOperator::And,
            BinaryOperator::Coalesce => IrBinaryOperator::Coalesce,
            BinaryOperator::Exponential => return Some(ir_math("pow", &[left, right])),
            BinaryOperator::Remainder => return Some(ir_math("mod", &[left, right])),
            BinaryOperator::ShiftLeft => return Some(self.lower_left_shift_operation(left, right)),
            BinaryOperator::ShiftRight => {
                return Some(self.lower_right_shift_operation(left, right));
            }
            BinaryOperator::BitwiseOr => {
                return Some(self.lower_bitwise_operation(BitwiseOperation::Or { left, right }));
            }
            BinaryOperator::BitwiseAnd => {
                return Some(self.lower_bitwise_operation(BitwiseOperation::And { left, right }));
            }
            BinaryOperator::BitwiseXor => {
                return Some(self.lower_bitwise_operation(BitwiseOperation::Xor { left, right }));
            }
        };

        Some(IrBinary { left, operator, right }.into())
    }

    fn lower_unary_expression(
        &mut self,
        expr: &UnaryExpression<'src>,
    ) -> Option<IrExpression<'src>> {
        let argument = self.lower_expression(&expr.argument)?;
        let operator = match expr.operator {
            UnaryOperator::Negate => IrUnaryOperator::Negate,
            UnaryOperator::Not => IrUnaryOperator::Not,
            UnaryOperator::BitwiseNot => {
                let op = BitwiseOperation::Not { right: argument };
                return Some(self.lower_bitwise_operation(op));
            }
        };

        Some(IrUnary { operator, argument }.into())
    }

    fn lower_ternary_expression(
        &mut self,
        expr: &TernaryExpression<'src>,
    ) -> Option<IrExpression<'src>> {
        let test = self.lower_expression(&expr.test)?;
        let consequent = self.lower_expression(&expr.consequent)?;
        let alternate = self.lower_expression(&expr.alternate)?;
        Some(IrTernary { test, consequent, alternate }.into())
    }

    fn lower_conditional_expression(
        &mut self,
        expr: &ConditionalExpression<'src>,
    ) -> Option<IrExpression<'src>> {
        let test = self.lower_expression(&expr.test)?;
        let consequent = self.lower_expression(&expr.consequent)?;
        Some(IrConditional { test, consequent }.into())
    }

    fn lower_call_expression(&mut self, expr: &CallExpression<'src>) -> IrExpression<'src> {
        let arguments =
            expr.arguments.iter().filter_map(|arg| self.lower_expression(arg)).collect();
        let name = match &expr.callee.member {
            VariableMember::Object { .. } => todo!(),
            VariableMember::Property { property } => property.name.clone(),
        };
        match expr.callee.scope {
            VariableScope::Temporary => todo!(),
            VariableScope::Variable => todo!(),
            VariableScope::Context => todo!(),
            VariableScope::Math => IrMath { name, arguments }.into(),
            VariableScope::Query => IrQuery { name, arguments }.into(),
        }
    }

    fn lower_block_expression(&mut self, expr: &BlockExpression<'src>) -> IrExpression<'src> {
        let stmts: Vec<_> =
            expr.statements.iter().filter_map(|stmt| self.lower_statement(stmt)).collect();
        IrExpression::Block(stmts)
    }

    fn lower_resource_expression(
        &mut self,
        expr: &ResourceExpression<'src>,
    ) -> Option<IrExpression<'src>> {
        let kind = match expr.section {
            ResourceSection::Geometry => IrResourceSection::Geometry,
            ResourceSection::Material => IrResourceSection::Material,
            ResourceSection::Texture => IrResourceSection::Texture,
        };
        let name = expr.name.name.clone();
        Some(IrResource { section: kind, name }.into())
    }

    fn lower_array_access_expression(
        &mut self,
        expr: &ArrayAccessExpression<'src>,
    ) -> Option<IrExpression<'src>> {
        let index = self.lower_expression(&expr.index)?;
        let name = expr.name.name.clone();
        Some(IrArrayAccess { name, index }.into())
    }

    fn lower_arrow_access_expression(
        &mut self,
        expr: &ArrowAccessExpression<'src>,
    ) -> Option<IrExpression<'src>> {
        let left = self.lower_expression(&expr.left)?;
        let right = self.lower_expression(&expr.right)?;
        Some(IrArrowAccess { left, right }.into())
    }

    fn lower_bitwise_operation(&mut self, operation: BitwiseOperation<'src>) -> IrExpression<'src> {
        let right = match &operation {
            BitwiseOperation::Or { right, .. }
            | BitwiseOperation::And { right, .. }
            | BitwiseOperation::Xor { right, .. }
            | BitwiseOperation::Not { right } => right.clone(),
        };
        let left = match &operation {
            BitwiseOperation::Or { left, .. }
            | BitwiseOperation::And { left, .. }
            | BitwiseOperation::Xor { left, .. } => Some(left.clone()),
            BitwiseOperation::Not { .. } => None,
        };

        let result_var = self.new_variable();
        let bit_var = self.new_variable();
        let left_bit_var = left.is_some().then(|| self.new_variable());
        let right_bit_var = self.new_variable();
        let op_bit_var = self.new_variable();

        fn extract_bit<'src>(
            input: IrExpression<'src>,
            bit: IrExpression<'src>,
        ) -> IrExpression<'src> {
            ir_math(
                "mod",
                &[
                    ir_math(
                        "floor",
                        &[ir_binary(
                            input,
                            IrBinaryOperator::Division,
                            ir_math("pow", &[IrExpression::Number(2.0), bit]),
                        )],
                    ),
                    IrExpression::Number(2.0),
                ],
            )
        }

        let (op_bit_var, op_expr) = match (&operation, left_bit_var.clone()) {
            (BitwiseOperation::Or { .. }, Some(left_bit_var)) => (
                op_bit_var,
                ir_math(
                    "min",
                    &[
                        IrExpression::Number(1.0),
                        ir_binary(
                            left_bit_var.into(),
                            IrBinaryOperator::Addition,
                            right_bit_var.clone().into(),
                        ),
                    ],
                ),
            ),
            (BitwiseOperation::And { .. }, Some(left_bit_var)) => (
                op_bit_var,
                ir_binary(
                    left_bit_var.into(),
                    IrBinaryOperator::Multiplication,
                    right_bit_var.clone().into(),
                ),
            ),
            (BitwiseOperation::Xor { .. }, Some(left_bit_var)) => (
                op_bit_var,
                ir_math(
                    "mod",
                    &[
                        ir_binary(
                            left_bit_var.into(),
                            IrBinaryOperator::Addition,
                            right_bit_var.clone().into(),
                        ),
                        IrExpression::Number(2.0),
                    ],
                ),
            ),
            (BitwiseOperation::Not { .. }, None) => (
                op_bit_var,
                ir_binary(
                    IrExpression::Number(1.0),
                    IrBinaryOperator::Subtraction,
                    right_bit_var.clone().into(),
                ),
            ),
            _ => unreachable!(),
        };

        let mut loop_body = Vec::with_capacity(5);
        if let (Some(left), Some(left_bit_var)) = (left, left_bit_var) {
            loop_body.push(ir_assignment(
                left_bit_var.clone(),
                extract_bit(left, bit_var.clone().into()),
            ));
        }
        loop_body.extend([
            ir_assignment(right_bit_var.clone(), extract_bit(right, bit_var.clone().into())),
            ir_assignment(op_bit_var.clone(), op_expr),
            ir_assignment(
                result_var.clone(),
                ir_binary(
                    result_var.clone().into(),
                    IrBinaryOperator::Addition,
                    ir_binary(
                        op_bit_var.into(),
                        IrBinaryOperator::Multiplication,
                        ir_math("pow", &[IrExpression::Number(2.0), (bit_var.clone().into())]),
                    ),
                ),
            ),
            ir_assignment(
                bit_var.clone(),
                ir_binary(
                    bit_var.clone().into(),
                    IrBinaryOperator::Addition,
                    IrExpression::Number(1.0),
                ),
            ),
        ]);

        self.statements.extend([
            ir_assignment(result_var.clone(), IrExpression::Number(0.0)),
            ir_assignment(bit_var, IrExpression::Number(0.0)),
            IrLoop { count: IrExpression::Number(24.0), body: loop_body }.into(),
        ]);

        IrExpression::Variable(result_var.into())
    }

    fn lower_left_shift_operation(
        &self,
        left: IrExpression<'src>,
        right: IrExpression<'src>,
    ) -> IrExpression<'src> {
        IrBinary {
            left,
            operator: IrBinaryOperator::Multiplication,
            right: ir_math("pow", &[IrExpression::Number(2.0), right]),
        }
        .into()
    }

    fn lower_right_shift_operation(
        &self,
        left: IrExpression<'src>,
        right: IrExpression<'src>,
    ) -> IrExpression<'src> {
        ir_math(
            "floor",
            &[IrBinary {
                left,
                operator: IrBinaryOperator::Division,
                right: ir_math("pow", &[IrExpression::Number(2.0), right]),
            }
            .into()],
        )
    }

    fn new_variable(&mut self) -> IrVariable<'src> {
        let uid = self.uid;
        self.uid += 1;
        IrVariable { scope: IrVariableScope::Variable, name: vec![format!("__{uid}").into()] }
    }
}

enum BitwiseOperation<'src> {
    Or { left: IrExpression<'src>, right: IrExpression<'src> },
    And { left: IrExpression<'src>, right: IrExpression<'src> },
    Xor { left: IrExpression<'src>, right: IrExpression<'src> },
    Not { right: IrExpression<'src> },
}

fn ir_assignment<'src>(target: IrVariable<'src>, value: IrExpression<'src>) -> IrStatement<'src> {
    IrAssignment { target, value }.into()
}

fn ir_binary<'src>(
    left: IrExpression<'src>,
    operator: IrBinaryOperator,
    right: IrExpression<'src>,
) -> IrExpression<'src> {
    IrBinary { left, operator, right }.into()
}

fn ir_math<'src>(name: &'src str, arguments: &[IrExpression<'src>]) -> IrExpression<'src> {
    IrMath { name: name.into(), arguments: arguments.into() }.into()
}
