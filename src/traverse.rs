use crate::ast::*;

/// Traverses the AST using an implementer of [`Traverse`].
pub fn traverse<'a>(traverser: &mut impl Traverse<'a>, program: &mut Program<'a>) {
    walk_program(traverser, program);
}

#[expect(unused_variables)]
pub trait Traverse<'a>: Sized {
    #[inline]
    fn enter_program(&mut self, it: &mut Program<'a>) {}

    #[inline]
    fn exit_program(&mut self, it: &mut Program<'a>) {}

    #[inline]
    fn enter_statements(&mut self, it: &mut Vec<Statement<'a>>) {}

    #[inline]
    fn exit_statements(&mut self, it: &mut Vec<Statement<'a>>) {}

    #[inline]
    fn enter_statement(&mut self, it: &mut Statement<'a>) {}

    #[inline]
    fn exit_statement(&mut self, it: &mut Statement<'a>) {}

    #[inline]
    fn enter_assignment_statement(&mut self, it: &mut AssignmentStatement<'a>) {}

    #[inline]
    fn exit_assignment_statement(&mut self, it: &mut AssignmentStatement<'a>) {}

    #[inline]
    fn enter_loop_statement(&mut self, it: &mut LoopStatement<'a>) {}

    #[inline]
    fn exit_loop_statement(&mut self, it: &mut LoopStatement<'a>) {}

    #[inline]
    fn enter_for_each_statement(&mut self, it: &mut ForEachStatement<'a>) {}

    #[inline]
    fn exit_for_each_statement(&mut self, it: &mut ForEachStatement<'a>) {}

    #[inline]
    fn enter_return_statement(&mut self, it: &mut ReturnStatement<'a>) {}

    #[inline]
    fn exit_return_statement(&mut self, it: &mut ReturnStatement<'a>) {}

    #[inline]
    fn enter_break_statement(&mut self, it: &mut BreakStatement) {}

    #[inline]
    fn exit_break_statement(&mut self, it: &mut BreakStatement) {}

    #[inline]
    fn enter_continue_statement(&mut self, it: &mut ContinueStatement) {}

    #[inline]
    fn exit_continue_statement(&mut self, it: &mut ContinueStatement) {}

    #[inline]
    fn enter_empty_statement(&mut self, it: &mut EmptyStatement) {}

    #[inline]
    fn exit_empty_statement(&mut self, it: &mut EmptyStatement) {}

    #[inline]
    fn enter_expression(&mut self, it: &mut Expression<'a>) {}

    #[inline]
    fn exit_expression(&mut self, it: &mut Expression<'a>) {}

    #[inline]
    fn enter_identifier_reference(&mut self, it: &mut Identifier<'a>) {}

    #[inline]
    fn exit_identifier_reference(&mut self, it: &mut Identifier<'a>) {}

    #[inline]
    fn enter_numeric_literal(&mut self, it: &mut NumericLiteral<'a>) {}

    #[inline]
    fn exit_numeric_literal(&mut self, it: &mut NumericLiteral<'a>) {}

    #[inline]
    fn enter_boolean_literal(&mut self, it: &mut BooleanLiteral) {}

    #[inline]
    fn exit_boolean_literal(&mut self, it: &mut BooleanLiteral) {}

    #[inline]
    fn enter_string_literal(&mut self, it: &mut StringLiteral<'a>) {}

    #[inline]
    fn exit_string_literal(&mut self, it: &mut StringLiteral<'a>) {}

    #[inline]
    fn enter_variable_expression(&mut self, it: &mut VariableExpression<'a>) {}

    #[inline]
    fn exit_variable_expression(&mut self, it: &mut VariableExpression<'a>) {}

    #[inline]
    fn enter_variable_member(&mut self, it: &mut VariableMember<'a>) {}

    #[inline]
    fn exit_variable_member(&mut self, it: &mut VariableMember<'a>) {}

    #[inline]
    fn enter_parenthesized_expression(&mut self, it: &mut ParenthesizedExpression<'a>) {}

    #[inline]
    fn exit_parenthesized_expression(&mut self, it: &mut ParenthesizedExpression<'a>) {}

    #[inline]
    fn enter_block_expression(&mut self, it: &mut BlockExpression<'a>) {}

    #[inline]
    fn exit_block_expression(&mut self, it: &mut BlockExpression<'a>) {}

    #[inline]
    fn enter_binary_expression(&mut self, it: &mut BinaryExpression<'a>) {}

    #[inline]
    fn exit_binary_expression(&mut self, it: &mut BinaryExpression<'a>) {}

    #[inline]
    fn enter_unary_expression(&mut self, it: &mut UnaryExpression<'a>) {}

    #[inline]
    fn exit_unary_expression(&mut self, it: &mut UnaryExpression<'a>) {}

    #[inline]
    fn enter_update_expression(&mut self, it: &mut UpdateExpression<'a>) {}

    #[inline]
    fn exit_update_expression(&mut self, it: &mut UpdateExpression<'a>) {}

    #[inline]
    fn enter_ternary_expression(&mut self, it: &mut TernaryExpression<'a>) {}

    #[inline]
    fn exit_ternary_expression(&mut self, it: &mut TernaryExpression<'a>) {}

    #[inline]
    fn enter_conditional_expression(&mut self, it: &mut ConditionalExpression<'a>) {}

    #[inline]
    fn exit_conditional_expression(&mut self, it: &mut ConditionalExpression<'a>) {}

    #[inline]
    fn enter_resource_expression(&mut self, it: &mut ResourceExpression<'a>) {}

    #[inline]
    fn exit_resource_expression(&mut self, it: &mut ResourceExpression<'a>) {}

    #[inline]
    fn enter_array_access_expression(&mut self, it: &mut ArrayAccessExpression<'a>) {}

    #[inline]
    fn exit_array_access_expression(&mut self, it: &mut ArrayAccessExpression<'a>) {}

    #[inline]
    fn enter_arrow_access_expression(&mut self, it: &mut ArrowAccessExpression<'a>) {}

    #[inline]
    fn exit_arrow_access_expression(&mut self, it: &mut ArrowAccessExpression<'a>) {}

    #[inline]
    fn enter_call_expression(&mut self, it: &mut CallExpression<'a>) {}

    #[inline]
    fn exit_call_expression(&mut self, it: &mut CallExpression<'a>) {}

    #[inline]
    fn enter_this_expression(&mut self, it: &mut ThisExpression) {}

    #[inline]
    fn exit_this_expression(&mut self, it: &mut ThisExpression) {}
}

fn walk_program<'a>(traverser: &mut impl Traverse<'a>, it: &mut Program<'a>) {
    traverser.enter_program(it);
    match &mut it.body {
        ProgramBody::Simple(expr) => walk_expression(traverser, expr),
        ProgramBody::Complex(stmts) => walk_statements(traverser, stmts),
        ProgramBody::Empty => (),
    }
    traverser.exit_program(it);
}

fn walk_statements<'a>(traverser: &mut impl Traverse<'a>, it: &mut Vec<Statement<'a>>) {
    traverser.enter_statements(it);
    for stmt in it.iter_mut() {
        walk_statement(traverser, stmt);
    }
    traverser.exit_statements(it);
}

fn walk_statement<'a>(traverser: &mut impl Traverse<'a>, it: &mut Statement<'a>) {
    traverser.enter_statement(it);
    match it {
        Statement::Expression(it) => walk_expression(traverser, it),
        Statement::Assignment(it) => walk_assignment_statement(traverser, it),
        Statement::Loop(it) => walk_loop_statement(traverser, it),
        Statement::ForEach(it) => walk_for_each_statement(traverser, it),
        Statement::Return(it) => walk_return_statement(traverser, it),
        Statement::Break(it) => walk_break_statement(traverser, it),
        Statement::Continue(it) => walk_continue_statement(traverser, it),
        Statement::Empty(it) => walk_empty_statement(traverser, it),
    }
    traverser.exit_statement(it);
}

fn walk_assignment_statement<'a>(
    traverser: &mut impl Traverse<'a>,
    it: &mut AssignmentStatement<'a>,
) {
    traverser.enter_assignment_statement(it);
    walk_variable_expression(traverser, &mut it.left);
    walk_expression(traverser, &mut it.right);
    traverser.exit_assignment_statement(it);
}

fn walk_loop_statement<'a>(traverser: &mut impl Traverse<'a>, it: &mut LoopStatement<'a>) {
    traverser.enter_loop_statement(it);
    walk_expression(traverser, &mut it.count);
    walk_block_expression(traverser, &mut it.block);
    traverser.exit_loop_statement(it);
}

fn walk_for_each_statement<'a>(traverser: &mut impl Traverse<'a>, it: &mut ForEachStatement<'a>) {
    traverser.enter_for_each_statement(it);
    walk_variable_expression(traverser, &mut it.variable);
    walk_expression(traverser, &mut it.array);
    walk_block_expression(traverser, &mut it.block);
    traverser.exit_for_each_statement(it);
}

fn walk_return_statement<'a>(traverser: &mut impl Traverse<'a>, it: &mut ReturnStatement<'a>) {
    traverser.enter_return_statement(it);
    walk_expression(traverser, &mut it.argument);
    traverser.exit_return_statement(it);
}

fn walk_break_statement<'a>(traverser: &mut impl Traverse<'a>, it: &mut BreakStatement) {
    traverser.enter_break_statement(it);
    traverser.exit_break_statement(it);
}

fn walk_continue_statement<'a>(traverser: &mut impl Traverse<'a>, it: &mut ContinueStatement) {
    traverser.enter_continue_statement(it);
    traverser.exit_continue_statement(it);
}

fn walk_empty_statement<'a>(traverser: &mut impl Traverse<'a>, it: &mut EmptyStatement) {
    traverser.enter_empty_statement(it);
    traverser.exit_empty_statement(it);
}

fn walk_expression<'a>(traverser: &mut impl Traverse<'a>, it: &mut Expression<'a>) {
    traverser.enter_expression(it);
    match it {
        Expression::NumericLiteral(it) => walk_numeric_literal(traverser, it),
        Expression::BooleanLiteral(it) => walk_boolean_literal(traverser, it),
        Expression::StringLiteral(it) => walk_string_literal(traverser, it),
        Expression::Variable(it) => walk_variable_expression(traverser, it),
        Expression::Parenthesized(it) => walk_parenthesized_expression(traverser, it),
        Expression::Block(it) => walk_block_expression(traverser, it),
        Expression::Binary(it) => walk_binary_expression(traverser, it),
        Expression::Unary(it) => walk_unary_expression(traverser, it),
        Expression::Update(it) => walk_update_expression(traverser, it),
        Expression::Ternary(it) => walk_ternary_expression(traverser, it),
        Expression::Conditional(it) => walk_conditional_expression(traverser, it),
        Expression::Resource(it) => walk_resource_expression(traverser, it),
        Expression::ArrayAccess(it) => walk_array_access_expression(traverser, it),
        Expression::ArrowAccess(it) => walk_arrow_access_expression(traverser, it),
        Expression::Call(it) => walk_call_expression(traverser, it),
        Expression::This(it) => walk_this_expression(traverser, it),
    }
    traverser.exit_expression(it);
}

fn walk_identifier_reference<'a>(traverser: &mut impl Traverse<'a>, it: &mut Identifier<'a>) {
    traverser.enter_identifier_reference(it);
    traverser.exit_identifier_reference(it);
}

fn walk_boolean_literal<'a>(traverser: &mut impl Traverse<'a>, it: &mut BooleanLiteral) {
    traverser.enter_boolean_literal(it);
    traverser.exit_boolean_literal(it);
}

fn walk_numeric_literal<'a>(traverser: &mut impl Traverse<'a>, it: &mut NumericLiteral<'a>) {
    traverser.enter_numeric_literal(it);
    traverser.exit_numeric_literal(it);
}

fn walk_string_literal<'a>(traverser: &mut impl Traverse<'a>, it: &mut StringLiteral<'a>) {
    traverser.enter_string_literal(it);
    traverser.exit_string_literal(it);
}

fn walk_variable_expression<'a>(
    traverser: &mut impl Traverse<'a>,
    it: &mut VariableExpression<'a>,
) {
    traverser.enter_variable_expression(it);
    walk_variable_member(traverser, &mut it.member);
    traverser.exit_variable_expression(it);
}

fn walk_variable_member<'a>(traverser: &mut impl Traverse<'a>, it: &mut VariableMember<'a>) {
    traverser.enter_variable_member(it);
    match it {
        VariableMember::Object { object, property, .. } => {
            walk_variable_member(traverser, object);
            walk_identifier_reference(traverser, property);
        }
        VariableMember::Property { property, .. } => {
            walk_identifier_reference(traverser, property);
        }
    }
    traverser.exit_variable_member(it);
}

fn walk_parenthesized_expression<'a>(
    traverser: &mut impl Traverse<'a>,
    it: &mut ParenthesizedExpression<'a>,
) {
    traverser.enter_parenthesized_expression(it);
    match &mut it.body {
        ParenthesizedBody::Single(expression) => {
            walk_expression(traverser, expression);
        }
        ParenthesizedBody::Multiple(statements) => {
            walk_statements(traverser, statements);
        }
    }
    traverser.exit_parenthesized_expression(it);
}

fn walk_block_expression<'a>(traverser: &mut impl Traverse<'a>, it: &mut BlockExpression<'a>) {
    traverser.enter_block_expression(it);
    walk_statements(traverser, &mut it.statements);
    traverser.exit_block_expression(it);
}

fn walk_binary_expression<'a>(traverser: &mut impl Traverse<'a>, it: &mut BinaryExpression<'a>) {
    traverser.enter_binary_expression(it);
    walk_expression(traverser, &mut it.left);
    walk_expression(traverser, &mut it.right);
    traverser.exit_binary_expression(it);
}

fn walk_unary_expression<'a>(traverser: &mut impl Traverse<'a>, it: &mut UnaryExpression<'a>) {
    traverser.enter_unary_expression(it);
    walk_expression(traverser, &mut it.argument);
    traverser.exit_unary_expression(it);
}

fn walk_update_expression<'a>(visitor: &mut impl Traverse<'a>, it: &mut UpdateExpression<'a>) {
    visitor.enter_update_expression(it);
    walk_variable_expression(visitor, &mut it.variable);
    visitor.exit_update_expression(it);
}

fn walk_ternary_expression<'a>(traverser: &mut impl Traverse<'a>, it: &mut TernaryExpression<'a>) {
    traverser.enter_ternary_expression(it);
    walk_expression(traverser, &mut it.test);
    walk_expression(traverser, &mut it.consequent);
    walk_expression(traverser, &mut it.alternate);
    traverser.exit_ternary_expression(it);
}

fn walk_conditional_expression<'a>(
    traverser: &mut impl Traverse<'a>,
    it: &mut ConditionalExpression<'a>,
) {
    traverser.enter_conditional_expression(it);
    walk_expression(traverser, &mut it.test);
    walk_expression(traverser, &mut it.consequent);
    traverser.exit_conditional_expression(it);
}

fn walk_resource_expression<'a>(
    traverser: &mut impl Traverse<'a>,
    it: &mut ResourceExpression<'a>,
) {
    traverser.enter_resource_expression(it);
    walk_identifier_reference(traverser, &mut it.name);
    traverser.exit_resource_expression(it);
}

fn walk_array_access_expression<'a>(
    traverser: &mut impl Traverse<'a>,
    it: &mut ArrayAccessExpression<'a>,
) {
    traverser.enter_array_access_expression(it);
    walk_identifier_reference(traverser, &mut it.name);
    walk_expression(traverser, &mut it.index);
    traverser.exit_array_access_expression(it);
}

fn walk_arrow_access_expression<'a>(
    traverser: &mut impl Traverse<'a>,
    it: &mut ArrowAccessExpression<'a>,
) {
    traverser.enter_arrow_access_expression(it);
    walk_expression(traverser, &mut it.left);
    walk_expression(traverser, &mut it.right);
    traverser.exit_arrow_access_expression(it);
}

fn walk_call_expression<'a>(traverser: &mut impl Traverse<'a>, it: &mut CallExpression<'a>) {
    traverser.enter_call_expression(it);
    walk_identifier_reference(traverser, &mut it.callee);
    if let Some(args) = &mut it.arguments {
        for arg in args {
            walk_expression(traverser, arg);
        }
    }
    traverser.exit_call_expression(it);
}

fn walk_this_expression<'a>(traverser: &mut impl Traverse<'a>, it: &mut ThisExpression) {
    traverser.enter_this_expression(it);
    traverser.exit_this_expression(it);
}
