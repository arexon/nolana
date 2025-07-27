use crate::ast::*;

/// Syntax tree traversal.
pub trait Visit<'a>: Sized {
    #[inline]
    #[allow(unused_variables)]
    fn enter_program(&mut self, it: &Program<'a>) {}

    #[inline]
    #[allow(unused_variables)]
    fn exit_program(&mut self, it: &Program<'a>) {}

    #[inline]
    #[allow(unused_variables)]
    fn enter_expressions(&mut self, it: &[Expression<'a>]) {}

    #[inline]
    #[allow(unused_variables)]
    fn exit_expressions(&mut self, it: &[Expression<'a>]) {}

    #[inline]
    #[allow(unused_variables)]
    fn enter_expression(&mut self, it: &Expression<'a>) {}

    #[inline]
    #[allow(unused_variables)]
    fn exit_expression(&mut self, it: &Expression<'a>) {}

    #[inline]
    #[allow(unused_variables)]
    fn enter_identifier_reference(&mut self, it: &IdentifierReference<'a>) {}

    #[inline]
    #[allow(unused_variables)]
    fn exit_identifier_reference(&mut self, it: &IdentifierReference<'a>) {}

    #[inline]
    #[allow(unused_variables)]
    fn enter_boolean_literal(&mut self, it: &BooleanLiteral) {}

    #[inline]
    #[allow(unused_variables)]
    fn exit_boolean_literal(&mut self, it: &BooleanLiteral) {}

    #[inline]
    #[allow(unused_variables)]
    fn enter_numeric_literal(&mut self, it: &NumericLiteral<'a>) {}

    #[inline]
    #[allow(unused_variables)]
    fn exit_numeric_literal(&mut self, it: &NumericLiteral<'a>) {}

    #[inline]
    #[allow(unused_variables)]
    fn enter_string_literal(&mut self, it: &StringLiteral<'a>) {}

    #[inline]
    #[allow(unused_variables)]
    fn exit_string_literal(&mut self, it: &StringLiteral<'a>) {}

    #[inline]
    #[allow(unused_variables)]
    fn enter_variable_expression(&mut self, it: &VariableExpression<'a>) {}

    #[inline]
    #[allow(unused_variables)]
    fn exit_variable_expression(&mut self, it: &VariableExpression<'a>) {}

    #[inline]
    #[allow(unused_variables)]
    fn enter_variable_member(&mut self, it: &VariableMember<'a>) {}

    #[inline]
    #[allow(unused_variables)]
    fn exit_variable_member(&mut self, it: &VariableMember<'a>) {}

    #[inline]
    #[allow(unused_variables)]
    fn enter_parenthesized_expression(&mut self, it: &ParenthesizedExpression<'a>) {}

    #[inline]
    #[allow(unused_variables)]
    fn exit_parenthesized_expression(&mut self, it: &ParenthesizedExpression<'a>) {}

    #[inline]
    #[allow(unused_variables)]
    fn enter_block_expression(&mut self, it: &BlockExpression<'a>) {}

    #[inline]
    #[allow(unused_variables)]
    fn exit_block_expression(&mut self, it: &BlockExpression<'a>) {}

    #[inline]
    #[allow(unused_variables)]
    fn enter_binary_expression(&mut self, it: &BinaryExpression<'a>) {}

    #[inline]
    #[allow(unused_variables)]
    fn exit_binary_expression(&mut self, it: &BinaryExpression<'a>) {}

    #[inline]
    #[allow(unused_variables)]
    fn enter_unary_expression(&mut self, it: &UnaryExpression<'a>) {}

    #[inline]
    #[allow(unused_variables)]
    fn exit_unary_expression(&mut self, it: &UnaryExpression<'a>) {}

    #[inline]
    #[allow(unused_variables)]
    fn enter_ternary_expression(&mut self, it: &TernaryExpression<'a>) {}

    #[inline]
    #[allow(unused_variables)]
    fn exit_ternary_expression(&mut self, it: &TernaryExpression<'a>) {}

    #[inline]
    #[allow(unused_variables)]
    fn enter_conditional_expression(&mut self, it: &ConditionalExpression<'a>) {}

    #[inline]
    #[allow(unused_variables)]
    fn exit_conditional_expression(&mut self, it: &ConditionalExpression<'a>) {}

    #[inline]
    #[allow(unused_variables)]
    fn enter_assignment_expression(&mut self, it: &AssignmentExpression<'a>) {}

    #[inline]
    #[allow(unused_variables)]
    fn exit_assignment_expression(&mut self, it: &AssignmentExpression<'a>) {}

    #[inline]
    #[allow(unused_variables)]
    fn enter_resource_expression(&mut self, it: &ResourceExpression<'a>) {}

    #[inline]
    #[allow(unused_variables)]
    fn exit_resource_expression(&mut self, it: &ResourceExpression<'a>) {}

    #[inline]
    #[allow(unused_variables)]
    fn enter_array_access_expression(&mut self, it: &ArrayAccessExpression<'a>) {}

    #[inline]
    #[allow(unused_variables)]
    fn exit_array_access_expression(&mut self, it: &ArrayAccessExpression<'a>) {}

    #[inline]
    #[allow(unused_variables)]
    fn enter_arrow_access_expression(&mut self, it: &ArrowAccessExpression<'a>) {}

    #[inline]
    #[allow(unused_variables)]
    fn exit_arrow_access_expression(&mut self, it: &ArrowAccessExpression<'a>) {}

    #[inline]
    #[allow(unused_variables)]
    fn enter_call_expression(&mut self, it: &CallExpression<'a>) {}

    #[inline]
    #[allow(unused_variables)]
    fn exit_call_expression(&mut self, it: &CallExpression<'a>) {}

    #[inline]
    #[allow(unused_variables)]
    fn enter_loop_expression(&mut self, it: &LoopExpression<'a>) {}

    #[inline]
    #[allow(unused_variables)]
    fn exit_loop_expression(&mut self, it: &LoopExpression<'a>) {}

    #[inline]
    #[allow(unused_variables)]
    fn enter_for_each_expression(&mut self, it: &ForEachExpression<'a>) {}

    #[inline]
    #[allow(unused_variables)]
    fn exit_for_each_expression(&mut self, it: &ForEachExpression<'a>) {}

    #[inline]
    #[allow(unused_variables)]
    fn enter_break(&mut self, it: &Break) {}

    #[inline]
    #[allow(unused_variables)]
    fn exit_break(&mut self, it: &Break) {}

    #[inline]
    #[allow(unused_variables)]
    fn enter_continue(&mut self, it: &Continue) {}

    #[inline]
    #[allow(unused_variables)]
    fn exit_continue(&mut self, it: &Continue) {}

    #[inline]
    #[allow(unused_variables)]
    fn enter_this(&mut self, it: &This) {}

    #[inline]
    #[allow(unused_variables)]
    fn exit_this(&mut self, it: &This) {}

    #[inline]
    #[allow(unused_variables)]
    fn enter_return(&mut self, it: &Return<'a>) {}

    #[inline]
    #[allow(unused_variables)]
    fn exit_return(&mut self, it: &Return<'a>) {}
}

pub mod walk {
    use super::*;

    #[inline]
    pub fn walk_program<'a>(visitor: &mut impl Visit<'a>, it: &Program<'a>) {
        visitor.enter_program(it);
        walk_expressions(visitor, &it.body);
        visitor.exit_program(it);
    }

    #[inline]
    pub fn walk_expressions<'a>(visitor: &mut impl Visit<'a>, it: &[Expression<'a>]) {
        visitor.enter_expressions(it);
        for expr in it {
            walk_expression(visitor, expr);
        }
        visitor.exit_expressions(it);
    }

    #[inline]
    pub fn walk_expression<'a>(visitor: &mut impl Visit<'a>, it: &Expression<'a>) {
        visitor.enter_expression(it);
        match it {
            Expression::BooleanLiteral(it) => walk_boolean_literal(visitor, it),
            Expression::NumericLiteral(it) => walk_numeric_literal(visitor, it),
            Expression::StringLiteral(it) => walk_string_literal(visitor, it),
            Expression::Variable(it) => walk_variable_expression(visitor, it),
            Expression::Parenthesized(it) => walk_parenthesized_expression(visitor, it),
            Expression::Block(it) => walk_block_expression(visitor, it),
            Expression::Binary(it) => walk_binary_expression(visitor, it),
            Expression::Unary(it) => walk_unary_expression(visitor, it),
            Expression::Ternary(it) => walk_ternary_expression(visitor, it),
            Expression::Conditional(it) => walk_conditional_expression(visitor, it),
            Expression::Assignment(it) => walk_assignment_expression(visitor, it),
            Expression::Resource(it) => walk_resource_expression(visitor, it),
            Expression::ArrayAccess(it) => walk_array_access_expression(visitor, it),
            Expression::ArrowAccess(it) => walk_arrow_access_expression(visitor, it),
            Expression::Call(it) => walk_call_expression(visitor, it),
            Expression::Loop(it) => walk_loop_expression(visitor, it),
            Expression::ForEach(it) => walk_for_each_expression(visitor, it),
            Expression::Break(it) => walk_break(visitor, it),
            Expression::Continue(it) => walk_continue(visitor, it),
            Expression::This(it) => walk_this(visitor, it),
            Expression::Return(it) => walk_return(visitor, it),
        }
        visitor.exit_expression(it);
    }

    #[inline]
    #[allow(unused_variables)]
    pub fn walk_identifier_reference<'a>(
        visitor: &mut impl Visit<'a>,
        it: &IdentifierReference<'a>,
    ) {
        visitor.enter_identifier_reference(it);
        visitor.exit_identifier_reference(it);
    }

    #[inline]
    #[allow(unused_variables)]
    pub fn walk_boolean_literal<'a>(visitor: &mut impl Visit<'a>, it: &BooleanLiteral) {
        visitor.enter_boolean_literal(it);
        visitor.exit_boolean_literal(it);
    }

    #[inline]
    #[allow(unused_variables)]
    pub fn walk_numeric_literal<'a>(visitor: &mut impl Visit<'a>, it: &NumericLiteral<'a>) {
        visitor.enter_numeric_literal(it);
        visitor.exit_numeric_literal(it);
    }

    #[inline]
    #[allow(unused_variables)]
    pub fn walk_string_literal<'a>(visitor: &mut impl Visit<'a>, it: &StringLiteral<'a>) {
        visitor.enter_string_literal(it);
        visitor.exit_string_literal(it);
    }

    #[inline]
    pub fn walk_variable_expression<'a>(visitor: &mut impl Visit<'a>, it: &VariableExpression<'a>) {
        visitor.enter_variable_expression(it);
        walk_variable_member(visitor, &it.member);
        visitor.exit_variable_expression(it);
    }

    #[inline]
    pub fn walk_variable_member<'a>(visitor: &mut impl Visit<'a>, it: &VariableMember<'a>) {
        visitor.enter_variable_member(it);
        match it {
            VariableMember::Object { object, property, .. } => {
                walk_variable_member(visitor, object);
                walk_identifier_reference(visitor, property);
            }
            VariableMember::Property { property, .. } => {
                walk_identifier_reference(visitor, property);
            }
        }
        visitor.exit_variable_member(it);
    }

    #[inline]
    pub fn walk_parenthesized_expression<'a>(
        visitor: &mut impl Visit<'a>,
        it: &ParenthesizedExpression<'a>,
    ) {
        visitor.enter_parenthesized_expression(it);
        match it {
            ParenthesizedExpression::Single { expression, .. } => {
                walk_expression(visitor, expression);
            }
            ParenthesizedExpression::Complex { expressions, .. } => {
                walk_expressions(visitor, expressions);
            }
        }
        visitor.exit_parenthesized_expression(it);
    }

    #[inline]
    pub fn walk_block_expression<'a>(visitor: &mut impl Visit<'a>, it: &BlockExpression<'a>) {
        visitor.enter_block_expression(it);
        walk_expressions(visitor, &it.expressions);
        visitor.exit_block_expression(it);
    }

    #[inline]
    pub fn walk_binary_expression<'a>(visitor: &mut impl Visit<'a>, it: &BinaryExpression<'a>) {
        visitor.enter_binary_expression(it);
        walk_expression(visitor, &it.left);
        walk_expression(visitor, &it.right);
        visitor.exit_binary_expression(it);
    }

    #[inline]
    pub fn walk_unary_expression<'a>(visitor: &mut impl Visit<'a>, it: &UnaryExpression<'a>) {
        visitor.enter_unary_expression(it);
        walk_expression(visitor, &it.argument);
        visitor.exit_unary_expression(it);
    }

    #[inline]
    pub fn walk_ternary_expression<'a>(visitor: &mut impl Visit<'a>, it: &TernaryExpression<'a>) {
        visitor.enter_ternary_expression(it);
        walk_expression(visitor, &it.test);
        walk_expression(visitor, &it.consequent);
        walk_expression(visitor, &it.alternate);
        visitor.exit_ternary_expression(it);
    }

    #[inline]
    pub fn walk_conditional_expression<'a>(
        visitor: &mut impl Visit<'a>,
        it: &ConditionalExpression<'a>,
    ) {
        visitor.enter_conditional_expression(it);
        walk_expression(visitor, &it.test);
        walk_expression(visitor, &it.consequent);
        visitor.exit_conditional_expression(it);
    }

    #[inline]
    pub fn walk_assignment_expression<'a>(
        visitor: &mut impl Visit<'a>,
        it: &AssignmentExpression<'a>,
    ) {
        visitor.enter_assignment_expression(it);
        walk_variable_expression(visitor, &it.left);
        walk_expression(visitor, &it.right);
        visitor.exit_assignment_expression(it);
    }

    #[inline]
    pub fn walk_resource_expression<'a>(visitor: &mut impl Visit<'a>, it: &ResourceExpression<'a>) {
        visitor.enter_resource_expression(it);
        walk_identifier_reference(visitor, &it.name);
        visitor.exit_resource_expression(it);
    }

    #[inline]
    pub fn walk_array_access_expression<'a>(
        visitor: &mut impl Visit<'a>,
        it: &ArrayAccessExpression<'a>,
    ) {
        visitor.enter_array_access_expression(it);
        walk_identifier_reference(visitor, &it.name);
        walk_expression(visitor, &it.index);
        visitor.exit_array_access_expression(it);
    }

    #[inline]
    pub fn walk_arrow_access_expression<'a>(
        visitor: &mut impl Visit<'a>,
        it: &ArrowAccessExpression<'a>,
    ) {
        visitor.enter_arrow_access_expression(it);
        walk_expression(visitor, &it.left);
        walk_expression(visitor, &it.right);
        visitor.exit_arrow_access_expression(it);
    }

    #[inline]
    pub fn walk_call_expression<'a>(visitor: &mut impl Visit<'a>, it: &CallExpression<'a>) {
        visitor.enter_call_expression(it);
        walk_identifier_reference(visitor, &it.callee);
        if let Some(args) = &it.arguments {
            walk_expressions(visitor, args);
        }
        visitor.exit_call_expression(it);
    }

    #[inline]
    pub fn walk_loop_expression<'a>(visitor: &mut impl Visit<'a>, it: &LoopExpression<'a>) {
        visitor.enter_loop_expression(it);
        walk_expression(visitor, &it.count);
        walk_block_expression(visitor, &it.expression);
        visitor.exit_loop_expression(it);
    }

    #[inline]
    pub fn walk_for_each_expression<'a>(visitor: &mut impl Visit<'a>, it: &ForEachExpression<'a>) {
        visitor.enter_for_each_expression(it);
        walk_variable_expression(visitor, &it.variable);
        walk_expression(visitor, &it.array);
        walk_block_expression(visitor, &it.expression);
        visitor.exit_for_each_expression(it);
    }

    #[inline]
    #[allow(unused_variables)]
    pub fn walk_break<'a>(visitor: &mut impl Visit<'a>, it: &Break) {
        visitor.enter_break(it);
        visitor.exit_break(it);
    }

    #[inline]
    #[allow(unused_variables)]
    pub fn walk_continue<'a>(visitor: &mut impl Visit<'a>, it: &Continue) {
        visitor.enter_continue(it);
        visitor.exit_continue(it);
    }

    #[inline]
    #[allow(unused_variables)]
    pub fn walk_this<'a>(visitor: &mut impl Visit<'a>, it: &This) {
        visitor.enter_this(it);
        visitor.exit_this(it);
    }

    #[inline]
    pub fn walk_return<'a>(visitor: &mut impl Visit<'a>, it: &Return<'a>) {
        visitor.enter_return(it);
        walk_expression(visitor, &it.argument);
        visitor.exit_return(it);
    }
}
