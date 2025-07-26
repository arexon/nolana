use crate::ast::*;

use walk_mut::*;

/// Syntax tree traversal.
pub trait VisitMut<'a>: Sized {
    #[inline]
    #[allow(unused_variables)]
    fn enter_node(&mut self, kind: AstKind) {}

    #[inline]
    #[allow(unused_variables)]
    fn leave_node(&mut self, kind: AstKind) {}

    #[inline]
    fn visit_program(&mut self, it: &mut Program<'a>) {
        walk_program(self, it);
    }

    #[inline]
    fn visit_expressions(&mut self, it: &mut Vec<Expression<'a>>) {
        walk_expressions(self, it);
    }

    #[inline]
    fn visit_expression(&mut self, it: &mut Expression<'a>) {
        walk_expression(self, it);
    }

    #[inline]
    fn visit_identifier_reference(&mut self, it: &mut IdentifierReference<'a>) {
        walk_identifier_reference(self, it)
    }

    #[inline]
    fn visit_boolean_literal(&mut self, it: &mut BooleanLiteral) {
        walk_boolean_literal(self, it);
    }

    #[inline]
    fn visit_numeric_literal(&mut self, it: &mut NumericLiteral<'a>) {
        walk_numeric_literal(self, it);
    }

    #[inline]
    fn visit_string_literal(&mut self, it: &mut StringLiteral<'a>) {
        walk_string_literal(self, it);
    }

    #[inline]
    fn visit_variable_expression(&mut self, it: &mut VariableExpression<'a>) {
        walk_variable_expression(self, it);
    }

    #[inline]
    fn visit_variable_member(&mut self, it: &mut VariableMember<'a>) {
        walk_variable_member(self, it);
    }

    #[inline]
    fn visit_parenthesized_expression(&mut self, it: &mut ParenthesizedExpression<'a>) {
        walk_parenthesized_expression(self, it);
    }

    #[inline]
    fn visit_block_expression(&mut self, it: &mut BlockExpression<'a>) {
        walk_block_expression(self, it);
    }

    #[inline]
    fn visit_binary_expression(&mut self, it: &mut BinaryExpression<'a>) {
        walk_binary_expression(self, it);
    }

    #[inline]
    fn visit_unary_expression(&mut self, it: &mut UnaryExpression<'a>) {
        walk_unary_expression(self, it);
    }

    #[inline]
    fn visit_ternary_expression(&mut self, it: &mut TernaryExpression<'a>) {
        walk_ternary_expression(self, it);
    }

    #[inline]
    fn visit_conditional_expression(&mut self, it: &mut ConditionalExpression<'a>) {
        walk_conditional_expression(self, it);
    }

    #[inline]
    fn visit_assignment_expression(&mut self, it: &mut AssignmentExpression<'a>) {
        walk_assignment_expression(self, it);
    }

    #[inline]
    fn visit_resource_expression(&mut self, it: &mut ResourceExpression<'a>) {
        walk_resource_expression(self, it);
    }

    #[inline]
    fn visit_array_access_expression(&mut self, it: &mut ArrayAccessExpression<'a>) {
        walk_array_access_expression(self, it);
    }

    #[inline]
    fn visit_arrow_access_expression(&mut self, it: &mut ArrowAccessExpression<'a>) {
        walk_arrow_access_expression(self, it);
    }

    #[inline]
    fn visit_call_expression(&mut self, it: &mut CallExpression<'a>) {
        walk_call_expression(self, it);
    }

    #[inline]
    fn visit_loop_expression(&mut self, it: &mut LoopExpression<'a>) {
        walk_loop_expression(self, it);
    }

    #[inline]
    fn visit_for_each_expression(&mut self, it: &mut ForEachExpression<'a>) {
        walk_for_each_expression(self, it);
    }

    #[inline]
    fn visit_break(&mut self, it: &mut Break) {
        walk_break(self, it);
    }

    #[inline]
    fn visit_continue(&mut self, it: &mut Continue) {
        walk_continue(self, it);
    }

    #[inline]
    fn visit_this(&mut self, it: &mut This) {
        walk_this(self, it);
    }

    #[inline]
    fn visit_return(&mut self, it: &mut Return<'a>) {
        walk_return(self, it);
    }
}

pub mod walk_mut {
    use super::*;

    #[inline]
    pub fn walk_program<'a>(visitor: &mut impl VisitMut<'a>, it: &mut Program<'a>) {
        let kind = AstKind::Program;
        visitor.enter_node(kind);
        visitor.visit_expressions(&mut it.body);
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_expressions<'a>(visitor: &mut impl VisitMut<'a>, it: &mut Vec<Expression<'a>>) {
        for expr in it.iter_mut() {
            visitor.visit_expression(expr);
        }
    }

    #[inline]
    pub fn walk_expression<'a>(visitor: &mut impl VisitMut<'a>, it: &mut Expression<'a>) {
        match it {
            Expression::BooleanLiteral(it) => visitor.visit_boolean_literal(it),
            Expression::NumericLiteral(it) => visitor.visit_numeric_literal(it),
            Expression::StringLiteral(it) => visitor.visit_string_literal(it),
            Expression::Variable(it) => visitor.visit_variable_expression(it),
            Expression::Parenthesized(it) => visitor.visit_parenthesized_expression(it),
            Expression::Block(it) => visitor.visit_block_expression(it),
            Expression::Binary(it) => visitor.visit_binary_expression(it),
            Expression::Unary(it) => visitor.visit_unary_expression(it),
            Expression::Ternary(it) => visitor.visit_ternary_expression(it),
            Expression::Conditional(it) => visitor.visit_conditional_expression(it),
            Expression::Assignment(it) => visitor.visit_assignment_expression(it),
            Expression::Resource(it) => visitor.visit_resource_expression(it),
            Expression::ArrayAccess(it) => visitor.visit_array_access_expression(it),
            Expression::ArrowAccess(it) => visitor.visit_arrow_access_expression(it),
            Expression::Call(it) => visitor.visit_call_expression(it),
            Expression::Loop(it) => visitor.visit_loop_expression(it),
            Expression::ForEach(it) => visitor.visit_for_each_expression(it),
            Expression::Break(it) => visitor.visit_break(it),
            Expression::Continue(it) => visitor.visit_continue(it),
            Expression::This(it) => visitor.visit_this(it),
            Expression::Return(it) => visitor.visit_return(it),
        }
    }

    #[inline]
    #[allow(unused_variables)]
    pub fn walk_identifier_reference<'a>(
        visitor: &mut impl VisitMut<'a>,
        it: &mut IdentifierReference<'a>,
    ) {
        let kind = AstKind::IdentifierReference;
        visitor.enter_node(kind);
        visitor.leave_node(kind);
    }

    #[inline]
    #[allow(unused_variables)]
    pub fn walk_boolean_literal<'a>(visitor: &mut impl VisitMut<'a>, it: &mut BooleanLiteral) {
        let kind = AstKind::BooleanLiteral;
        visitor.enter_node(kind);
        visitor.leave_node(kind);
    }

    #[inline]
    #[allow(unused_variables)]
    pub fn walk_numeric_literal<'a>(visitor: &mut impl VisitMut<'a>, it: &mut NumericLiteral<'a>) {
        let kind = AstKind::NumericLiteral;
        visitor.enter_node(kind);
        visitor.leave_node(kind);
    }

    #[inline]
    #[allow(unused_variables)]
    pub fn walk_string_literal<'a>(visitor: &mut impl VisitMut<'a>, it: &mut StringLiteral<'a>) {
        let kind = AstKind::StringLiteral;
        visitor.enter_node(kind);
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_variable_expression<'a>(
        visitor: &mut impl VisitMut<'a>,
        it: &mut VariableExpression<'a>,
    ) {
        let kind = AstKind::VariableExpression;
        visitor.enter_node(kind);
        visitor.visit_variable_member(&mut it.member);
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_variable_member<'a>(visitor: &mut impl VisitMut<'a>, it: &mut VariableMember<'a>) {
        let kind = AstKind::VariableMember;
        visitor.enter_node(kind);
        match it {
            VariableMember::Object {
                object, property, ..
            } => {
                visitor.visit_variable_member(object);
                visitor.visit_identifier_reference(property);
            }
            VariableMember::Property { property, .. } => {
                visitor.visit_identifier_reference(property);
            }
        }
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_parenthesized_expression<'a>(
        visitor: &mut impl VisitMut<'a>,
        it: &mut ParenthesizedExpression<'a>,
    ) {
        let kind = AstKind::ParenthesizedExpression;
        visitor.enter_node(kind);
        match it {
            ParenthesizedExpression::Single { expression, .. } => {
                visitor.visit_expression(expression);
            }
            ParenthesizedExpression::Complex { expressions, .. } => {
                visitor.visit_expressions(expressions);
            }
        }
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_block_expression<'a>(
        visitor: &mut impl VisitMut<'a>,
        it: &mut BlockExpression<'a>,
    ) {
        let kind = AstKind::BlockExpression;
        visitor.enter_node(kind);
        visitor.visit_expressions(&mut it.expressions);
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_binary_expression<'a>(
        visitor: &mut impl VisitMut<'a>,
        it: &mut BinaryExpression<'a>,
    ) {
        let kind = AstKind::BinaryExpression;
        visitor.enter_node(kind);
        visitor.visit_expression(&mut it.left);
        visitor.visit_expression(&mut it.right);
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_unary_expression<'a>(
        visitor: &mut impl VisitMut<'a>,
        it: &mut UnaryExpression<'a>,
    ) {
        let kind = AstKind::UnaryExpression;
        visitor.enter_node(kind);
        visitor.visit_expression(&mut it.argument);
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_ternary_expression<'a>(
        visitor: &mut impl VisitMut<'a>,
        it: &mut TernaryExpression<'a>,
    ) {
        let kind = AstKind::TernaryExpression;
        visitor.enter_node(kind);
        visitor.visit_expression(&mut it.test);
        visitor.visit_expression(&mut it.consequent);
        visitor.visit_expression(&mut it.alternate);
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_conditional_expression<'a>(
        visitor: &mut impl VisitMut<'a>,
        it: &mut ConditionalExpression<'a>,
    ) {
        let kind = AstKind::ConditionalExpression;
        visitor.enter_node(kind);
        visitor.visit_expression(&mut it.test);
        visitor.visit_expression(&mut it.consequent);
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_assignment_expression<'a>(
        visitor: &mut impl VisitMut<'a>,
        it: &mut AssignmentExpression<'a>,
    ) {
        let kind = AstKind::AssignmentExpression;
        visitor.enter_node(kind);
        visitor.visit_variable_expression(&mut it.left);
        visitor.visit_expression(&mut it.right);
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_resource_expression<'a>(
        visitor: &mut impl VisitMut<'a>,
        it: &mut ResourceExpression<'a>,
    ) {
        let kind = AstKind::ResourceExpression;
        visitor.enter_node(kind);
        visitor.visit_identifier_reference(&mut it.name);
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_array_access_expression<'a>(
        visitor: &mut impl VisitMut<'a>,
        it: &mut ArrayAccessExpression<'a>,
    ) {
        let kind = AstKind::ArrayAccessExpression;
        visitor.enter_node(kind);
        visitor.visit_identifier_reference(&mut it.name);
        visitor.visit_expression(&mut it.index);
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_arrow_access_expression<'a>(
        visitor: &mut impl VisitMut<'a>,
        it: &mut ArrowAccessExpression<'a>,
    ) {
        let kind = AstKind::ArrowAccessExpression;
        visitor.enter_node(kind);
        visitor.visit_expression(&mut it.left);
        visitor.visit_expression(&mut it.right);
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_call_expression<'a>(visitor: &mut impl VisitMut<'a>, it: &mut CallExpression<'a>) {
        let kind = AstKind::CallExpression;
        visitor.enter_node(kind);
        visitor.visit_identifier_reference(&mut it.callee);
        if let Some(args) = &mut it.arguments {
            visitor.visit_expressions(args);
        }
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_loop_expression<'a>(visitor: &mut impl VisitMut<'a>, it: &mut LoopExpression<'a>) {
        let kind = AstKind::LoopExpression;
        visitor.enter_node(kind);
        visitor.visit_expression(&mut it.count);
        visitor.visit_block_expression(&mut it.expression);
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_for_each_expression<'a>(
        visitor: &mut impl VisitMut<'a>,
        it: &mut ForEachExpression<'a>,
    ) {
        let kind = AstKind::ForEachExpression;
        visitor.enter_node(kind);
        visitor.visit_variable_expression(&mut it.variable);
        visitor.visit_expression(&mut it.array);
        visitor.visit_block_expression(&mut it.expression);
        visitor.leave_node(kind);
    }

    #[inline]
    #[allow(unused_variables)]
    pub fn walk_break<'a>(visitor: &mut impl VisitMut<'a>, it: &mut Break) {
        let kind = AstKind::Break;
        visitor.enter_node(kind);
        visitor.leave_node(kind);
    }

    #[inline]
    #[allow(unused_variables)]
    pub fn walk_continue<'a>(visitor: &mut impl VisitMut<'a>, it: &mut Continue) {
        let kind = AstKind::Continue;
        visitor.enter_node(kind);
        visitor.leave_node(kind);
    }

    #[inline]
    #[allow(unused_variables)]
    pub fn walk_this<'a>(visitor: &mut impl VisitMut<'a>, it: &mut This) {
        let kind = AstKind::This;
        visitor.enter_node(kind);
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_return<'a>(visitor: &mut impl VisitMut<'a>, it: &mut Return<'a>) {
        let kind = AstKind::Return;
        visitor.enter_node(kind);
        visitor.visit_expression(&mut it.argument);
        visitor.leave_node(kind);
    }
}
