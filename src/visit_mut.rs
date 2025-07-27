use crate::ast::*;

/// Syntax tree traversal.
pub trait VisitMut<'a>: Sized {
    #[inline]
    #[allow(unused_variables)]
    fn enter_program(&mut self, it: &mut Program<'a>) {}

    #[inline]
    #[allow(unused_variables)]
    fn exit_program(&mut self, it: &mut Program<'a>) {}

    #[inline]
    #[allow(unused_variables)]
    fn enter_expressions(&mut self, it: &mut Vec<Expression<'a>>) {}

    #[inline]
    #[allow(unused_variables)]
    fn exit_expressions(&mut self, it: &mut Vec<Expression<'a>>) {}

    #[inline]
    #[allow(unused_variables)]
    fn enter_expression(&mut self, it: &mut Expression<'a>) {}

    #[inline]
    #[allow(unused_variables)]
    fn exit_expression(&mut self, it: &mut Expression<'a>) {}

    #[inline]
    #[allow(unused_variables)]
    fn enter_identifier_reference(&mut self, it: &mut IdentifierReference<'a>) {}

    #[inline]
    #[allow(unused_variables)]
    fn exit_identifier_reference(&mut self, it: &mut IdentifierReference<'a>) {}

    #[inline]
    #[allow(unused_variables)]
    fn enter_boolean_literal(&mut self, it: &mut BooleanLiteral) {}

    #[inline]
    #[allow(unused_variables)]
    fn exit_boolean_literal(&mut self, it: &mut BooleanLiteral) {}

    #[inline]
    #[allow(unused_variables)]
    fn enter_numeric_literal(&mut self, it: &mut NumericLiteral<'a>) {}

    #[inline]
    #[allow(unused_variables)]
    fn exit_numeric_literal(&mut self, it: &mut NumericLiteral<'a>) {}

    #[inline]
    #[allow(unused_variables)]
    fn enter_string_literal(&mut self, it: &mut StringLiteral<'a>) {}

    #[inline]
    #[allow(unused_variables)]
    fn exit_string_literal(&mut self, it: &mut StringLiteral<'a>) {}

    #[inline]
    #[allow(unused_variables)]
    fn enter_variable_expression(&mut self, it: &mut VariableExpression<'a>) {}

    #[inline]
    #[allow(unused_variables)]
    fn exit_variable_expression(&mut self, it: &mut VariableExpression<'a>) {}

    #[inline]
    #[allow(unused_variables)]
    fn enter_variable_member(&mut self, it: &mut VariableMember<'a>) {}

    #[inline]
    #[allow(unused_variables)]
    fn exit_variable_member(&mut self, it: &mut VariableMember<'a>) {}

    #[inline]
    #[allow(unused_variables)]
    fn enter_parenthesized_expression(&mut self, it: &mut ParenthesizedExpression<'a>) {}

    #[inline]
    #[allow(unused_variables)]
    fn exit_parenthesized_expression(&mut self, it: &mut ParenthesizedExpression<'a>) {}

    #[inline]
    #[allow(unused_variables)]
    fn enter_block_expression(&mut self, it: &mut BlockExpression<'a>) {}

    #[inline]
    #[allow(unused_variables)]
    fn exit_block_expression(&mut self, it: &mut BlockExpression<'a>) {}

    #[inline]
    #[allow(unused_variables)]
    fn enter_binary_expression(&mut self, it: &mut BinaryExpression<'a>) {}

    #[inline]
    #[allow(unused_variables)]
    fn exit_binary_expression(&mut self, it: &mut BinaryExpression<'a>) {}

    #[inline]
    #[allow(unused_variables)]
    fn enter_unary_expression(&mut self, it: &mut UnaryExpression<'a>) {}

    #[inline]
    #[allow(unused_variables)]
    fn exit_unary_expression(&mut self, it: &mut UnaryExpression<'a>) {}

    #[inline]
    #[allow(unused_variables)]
    fn enter_ternary_expression(&mut self, it: &mut TernaryExpression<'a>) {}

    #[inline]
    #[allow(unused_variables)]
    fn exit_ternary_expression(&mut self, it: &mut TernaryExpression<'a>) {}

    #[inline]
    #[allow(unused_variables)]
    fn enter_conditional_expression(&mut self, it: &mut ConditionalExpression<'a>) {}

    #[inline]
    #[allow(unused_variables)]
    fn exit_conditional_expression(&mut self, it: &mut ConditionalExpression<'a>) {}

    #[inline]
    #[allow(unused_variables)]
    fn enter_assignment_expression(&mut self, it: &mut AssignmentExpression<'a>) {}

    #[inline]
    #[allow(unused_variables)]
    fn exit_assignment_expression(&mut self, it: &mut AssignmentExpression<'a>) {}

    #[inline]
    #[allow(unused_variables)]
    fn enter_resource_expression(&mut self, it: &mut ResourceExpression<'a>) {}

    #[inline]
    #[allow(unused_variables)]
    fn exit_resource_expression(&mut self, it: &mut ResourceExpression<'a>) {}

    #[inline]
    #[allow(unused_variables)]
    fn enter_array_access_expression(&mut self, it: &mut ArrayAccessExpression<'a>) {}

    #[inline]
    #[allow(unused_variables)]
    fn exit_array_access_expression(&mut self, it: &mut ArrayAccessExpression<'a>) {}

    #[inline]
    #[allow(unused_variables)]
    fn enter_arrow_access_expression(&mut self, it: &mut ArrowAccessExpression<'a>) {}

    #[inline]
    #[allow(unused_variables)]
    fn exit_arrow_access_expression(&mut self, it: &mut ArrowAccessExpression<'a>) {}

    #[inline]
    #[allow(unused_variables)]
    fn enter_call_expression(&mut self, it: &mut CallExpression<'a>) {}

    #[inline]
    #[allow(unused_variables)]
    fn exit_call_expression(&mut self, it: &mut CallExpression<'a>) {}

    #[inline]
    #[allow(unused_variables)]
    fn enter_loop_expression(&mut self, it: &mut LoopExpression<'a>) {}

    #[inline]
    #[allow(unused_variables)]
    fn exit_loop_expression(&mut self, it: &mut LoopExpression<'a>) {}

    #[inline]
    #[allow(unused_variables)]
    fn enter_for_each_expression(&mut self, it: &mut ForEachExpression<'a>) {}

    #[inline]
    #[allow(unused_variables)]
    fn exit_for_each_expression(&mut self, it: &mut ForEachExpression<'a>) {}

    #[inline]
    #[allow(unused_variables)]
    fn enter_break(&mut self, it: &mut Break) {}

    #[inline]
    #[allow(unused_variables)]
    fn exit_break(&mut self, it: &mut Break) {}

    #[inline]
    #[allow(unused_variables)]
    fn enter_continue(&mut self, it: &mut Continue) {}

    #[inline]
    #[allow(unused_variables)]
    fn exit_continue(&mut self, it: &mut Continue) {}

    #[inline]
    #[allow(unused_variables)]
    fn enter_this(&mut self, it: &mut This) {}

    #[inline]
    #[allow(unused_variables)]
    fn exit_this(&mut self, it: &mut This) {}

    #[inline]
    #[allow(unused_variables)]
    fn enter_return(&mut self, it: &mut Return<'a>) {}

    #[inline]
    #[allow(unused_variables)]
    fn exit_return(&mut self, it: &mut Return<'a>) {}
}

pub mod walk_mut {
    use super::*;

    #[inline]
    pub fn walk_program<'a>(visitor: &mut impl VisitMut<'a>, it: &mut Program<'a>) {
        visitor.enter_program(it);
        walk_expressions(visitor, &mut it.body);
        visitor.exit_program(it);
    }

    #[inline]
    pub fn walk_expressions<'a>(visitor: &mut impl VisitMut<'a>, it: &mut Vec<Expression<'a>>) {
        visitor.enter_expressions(it);
        for expr in it.iter_mut() {
            walk_expression(visitor, expr);
        }
        visitor.exit_expressions(it);
    }

    #[inline]
    pub fn walk_expression<'a>(visitor: &mut impl VisitMut<'a>, it: &mut Expression<'a>) {
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
        visitor: &mut impl VisitMut<'a>,
        it: &mut IdentifierReference<'a>,
    ) {
        visitor.enter_identifier_reference(it);
        visitor.exit_identifier_reference(it);
    }

    #[inline]
    #[allow(unused_variables)]
    pub fn walk_boolean_literal<'a>(visitor: &mut impl VisitMut<'a>, it: &mut BooleanLiteral) {
        visitor.enter_boolean_literal(it);
        visitor.exit_boolean_literal(it);
    }

    #[inline]
    #[allow(unused_variables)]
    pub fn walk_numeric_literal<'a>(visitor: &mut impl VisitMut<'a>, it: &mut NumericLiteral<'a>) {
        visitor.enter_numeric_literal(it);
        visitor.exit_numeric_literal(it);
    }

    #[inline]
    #[allow(unused_variables)]
    pub fn walk_string_literal<'a>(visitor: &mut impl VisitMut<'a>, it: &mut StringLiteral<'a>) {
        visitor.enter_string_literal(it);
        visitor.exit_string_literal(it);
    }

    #[inline]
    pub fn walk_variable_expression<'a>(
        visitor: &mut impl VisitMut<'a>,
        it: &mut VariableExpression<'a>,
    ) {
        visitor.enter_variable_expression(it);
        walk_variable_member(visitor, &mut it.member);
        visitor.exit_variable_expression(it);
    }

    #[inline]
    pub fn walk_variable_member<'a>(visitor: &mut impl VisitMut<'a>, it: &mut VariableMember<'a>) {
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
        visitor: &mut impl VisitMut<'a>,
        it: &mut ParenthesizedExpression<'a>,
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
    pub fn walk_block_expression<'a>(
        visitor: &mut impl VisitMut<'a>,
        it: &mut BlockExpression<'a>,
    ) {
        visitor.enter_block_expression(it);
        walk_expressions(visitor, &mut it.expressions);
        visitor.exit_block_expression(it);
    }

    #[inline]
    pub fn walk_binary_expression<'a>(
        visitor: &mut impl VisitMut<'a>,
        it: &mut BinaryExpression<'a>,
    ) {
        visitor.enter_binary_expression(it);
        walk_expression(visitor, &mut it.left);
        walk_expression(visitor, &mut it.right);
        visitor.exit_binary_expression(it);
    }

    #[inline]
    pub fn walk_unary_expression<'a>(
        visitor: &mut impl VisitMut<'a>,
        it: &mut UnaryExpression<'a>,
    ) {
        visitor.enter_unary_expression(it);
        walk_expression(visitor, &mut it.argument);
        visitor.exit_unary_expression(it);
    }

    #[inline]
    pub fn walk_ternary_expression<'a>(
        visitor: &mut impl VisitMut<'a>,
        it: &mut TernaryExpression<'a>,
    ) {
        visitor.enter_ternary_expression(it);
        walk_expression(visitor, &mut it.test);
        walk_expression(visitor, &mut it.consequent);
        walk_expression(visitor, &mut it.alternate);
        visitor.exit_ternary_expression(it);
    }

    #[inline]
    pub fn walk_conditional_expression<'a>(
        visitor: &mut impl VisitMut<'a>,
        it: &mut ConditionalExpression<'a>,
    ) {
        visitor.enter_conditional_expression(it);
        walk_expression(visitor, &mut it.test);
        walk_expression(visitor, &mut it.consequent);
        visitor.exit_conditional_expression(it);
    }

    #[inline]
    pub fn walk_assignment_expression<'a>(
        visitor: &mut impl VisitMut<'a>,
        it: &mut AssignmentExpression<'a>,
    ) {
        visitor.enter_assignment_expression(it);
        walk_variable_expression(visitor, &mut it.left);
        walk_expression(visitor, &mut it.right);
        visitor.exit_assignment_expression(it);
    }

    #[inline]
    pub fn walk_resource_expression<'a>(
        visitor: &mut impl VisitMut<'a>,
        it: &mut ResourceExpression<'a>,
    ) {
        visitor.enter_resource_expression(it);
        walk_identifier_reference(visitor, &mut it.name);
        visitor.exit_resource_expression(it);
    }

    #[inline]
    pub fn walk_array_access_expression<'a>(
        visitor: &mut impl VisitMut<'a>,
        it: &mut ArrayAccessExpression<'a>,
    ) {
        visitor.enter_array_access_expression(it);
        walk_identifier_reference(visitor, &mut it.name);
        walk_expression(visitor, &mut it.index);
        visitor.exit_array_access_expression(it);
    }

    #[inline]
    pub fn walk_arrow_access_expression<'a>(
        visitor: &mut impl VisitMut<'a>,
        it: &mut ArrowAccessExpression<'a>,
    ) {
        visitor.enter_arrow_access_expression(it);
        walk_expression(visitor, &mut it.left);
        walk_expression(visitor, &mut it.right);
        visitor.exit_arrow_access_expression(it);
    }

    #[inline]
    pub fn walk_call_expression<'a>(visitor: &mut impl VisitMut<'a>, it: &mut CallExpression<'a>) {
        visitor.enter_call_expression(it);
        walk_identifier_reference(visitor, &mut it.callee);
        if let Some(args) = &mut it.arguments {
            walk_expressions(visitor, args);
        }
        visitor.exit_call_expression(it);
    }

    #[inline]
    pub fn walk_loop_expression<'a>(visitor: &mut impl VisitMut<'a>, it: &mut LoopExpression<'a>) {
        visitor.enter_loop_expression(it);
        walk_expression(visitor, &mut it.count);
        walk_block_expression(visitor, &mut it.expression);
        visitor.exit_loop_expression(it);
    }

    #[inline]
    pub fn walk_for_each_expression<'a>(
        visitor: &mut impl VisitMut<'a>,
        it: &mut ForEachExpression<'a>,
    ) {
        visitor.enter_for_each_expression(it);
        walk_variable_expression(visitor, &mut it.variable);
        walk_expression(visitor, &mut it.array);
        walk_block_expression(visitor, &mut it.expression);
        visitor.exit_for_each_expression(it);
    }

    #[inline]
    #[allow(unused_variables)]
    pub fn walk_break<'a>(visitor: &mut impl VisitMut<'a>, it: &mut Break) {
        visitor.enter_break(it);
        visitor.exit_break(it);
    }

    #[inline]
    #[allow(unused_variables)]
    pub fn walk_continue<'a>(visitor: &mut impl VisitMut<'a>, it: &mut Continue) {
        visitor.enter_continue(it);
        visitor.exit_continue(it);
    }

    #[inline]
    #[allow(unused_variables)]
    pub fn walk_this<'a>(visitor: &mut impl VisitMut<'a>, it: &mut This) {
        visitor.enter_this(it);
        visitor.exit_this(it);
    }

    #[inline]
    pub fn walk_return<'a>(visitor: &mut impl VisitMut<'a>, it: &mut Return<'a>) {
        visitor.enter_return(it);
        walk_expression(visitor, &mut it.argument);
        visitor.exit_return(it);
    }
}
