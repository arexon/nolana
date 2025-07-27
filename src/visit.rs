use crate::ast::*;

use walk::*;

/// Syntax tree traversal.
pub trait Visit<'a>: Sized {
    #[inline]
    #[allow(unused_variables)]
    fn enter_node(&mut self, kind: AstKind) {}

    #[inline]
    #[allow(unused_variables)]
    fn leave_node(&mut self, kind: AstKind) {}

    #[inline]
    fn visit_program(&mut self, it: &Program<'a>) {
        walk_program(self, it);
    }

    #[inline]
    fn visit_expressions(&mut self, it: &Vec<Expression<'a>>) {
        walk_expressions(self, it);
    }

    #[inline]
    fn visit_expression(&mut self, it: &Expression<'a>) {
        walk_expression(self, it);
    }

    #[inline]
    fn visit_identifier_reference(&mut self, it: &IdentifierReference<'a>) {
        walk_identifier_reference(self, it)
    }

    #[inline]
    fn visit_boolean_literal(&mut self, it: &BooleanLiteral) {
        walk_boolean_literal(self, it);
    }

    #[inline]
    fn visit_numeric_literal(&mut self, it: &NumericLiteral<'a>) {
        walk_numeric_literal(self, it);
    }

    #[inline]
    fn visit_string_literal(&mut self, it: &StringLiteral<'a>) {
        walk_string_literal(self, it);
    }

    #[inline]
    fn visit_variable_expression(&mut self, it: &VariableExpression<'a>) {
        walk_variable_expression(self, it);
    }

    #[inline]
    fn visit_variable_member(&mut self, it: &VariableMember<'a>) {
        walk_variable_member(self, it);
    }

    #[inline]
    fn visit_parenthesized_expression(&mut self, it: &ParenthesizedExpression<'a>) {
        walk_parenthesized_expression(self, it);
    }

    #[inline]
    fn visit_block_expression(&mut self, it: &BlockExpression<'a>) {
        walk_block_expression(self, it);
    }

    #[inline]
    fn visit_binary_expression(&mut self, it: &BinaryExpression<'a>) {
        walk_binary_expression(self, it);
    }

    #[inline]
    fn visit_unary_expression(&mut self, it: &UnaryExpression<'a>) {
        walk_unary_expression(self, it);
    }

    #[inline]
    fn visit_ternary_expression(&mut self, it: &TernaryExpression<'a>) {
        walk_ternary_expression(self, it);
    }

    #[inline]
    fn visit_conditional_expression(&mut self, it: &ConditionalExpression<'a>) {
        walk_conditional_expression(self, it);
    }

    #[inline]
    fn visit_assignment_expression(&mut self, it: &AssignmentExpression<'a>) {
        walk_assignment_expression(self, it);
    }

    #[inline]
    fn visit_resource_expression(&mut self, it: &ResourceExpression<'a>) {
        walk_resource_expression(self, it);
    }

    #[inline]
    fn visit_array_access_expression(&mut self, it: &ArrayAccessExpression<'a>) {
        walk_array_access_expression(self, it);
    }

    #[inline]
    fn visit_arrow_access_expression(&mut self, it: &ArrowAccessExpression<'a>) {
        walk_arrow_access_expression(self, it);
    }

    #[inline]
    fn visit_call_expression(&mut self, it: &CallExpression<'a>) {
        walk_call_expression(self, it);
    }

    #[inline]
    fn visit_loop_expression(&mut self, it: &LoopExpression<'a>) {
        walk_loop_expression(self, it);
    }

    #[inline]
    fn visit_for_each_expression(&mut self, it: &ForEachExpression<'a>) {
        walk_for_each_expression(self, it);
    }

    #[inline]
    fn visit_break(&mut self, it: &Break) {
        walk_break(self, it);
    }

    #[inline]
    fn visit_continue(&mut self, it: &Continue) {
        walk_continue(self, it);
    }

    #[inline]
    fn visit_this(&mut self, it: &This) {
        walk_this(self, it);
    }

    #[inline]
    fn visit_return(&mut self, it: &Return<'a>) {
        walk_return(self, it);
    }
}

pub mod walk {
    use super::*;

    #[inline]
    pub fn walk_program<'a>(visitor: &mut impl Visit<'a>, it: &Program<'a>) {
        let kind = AstKind::Program;
        visitor.enter_node(kind);
        visitor.visit_expressions(&it.body);
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_expressions<'a>(visitor: &mut impl Visit<'a>, it: &Vec<Expression<'a>>) {
        for expr in it {
            visitor.visit_expression(expr);
        }
    }

    #[inline]
    pub fn walk_expression<'a>(visitor: &mut impl Visit<'a>, it: &Expression<'a>) {
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
        visitor: &mut impl Visit<'a>,
        it: &IdentifierReference<'a>,
    ) {
        let kind = AstKind::IdentifierReference;
        visitor.enter_node(kind);
        visitor.leave_node(kind);
    }

    #[inline]
    #[allow(unused_variables)]
    pub fn walk_boolean_literal<'a>(visitor: &mut impl Visit<'a>, it: &BooleanLiteral) {
        let kind = AstKind::BooleanLiteral;
        visitor.enter_node(kind);
        visitor.leave_node(kind);
    }

    #[inline]
    #[allow(unused_variables)]
    pub fn walk_numeric_literal<'a>(visitor: &mut impl Visit<'a>, it: &NumericLiteral<'a>) {
        let kind = AstKind::NumericLiteral;
        visitor.enter_node(kind);
        visitor.leave_node(kind);
    }

    #[inline]
    #[allow(unused_variables)]
    pub fn walk_string_literal<'a>(visitor: &mut impl Visit<'a>, it: &StringLiteral<'a>) {
        let kind = AstKind::StringLiteral;
        visitor.enter_node(kind);
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_variable_expression<'a>(visitor: &mut impl Visit<'a>, it: &VariableExpression<'a>) {
        let kind = AstKind::VariableExpression;
        visitor.enter_node(kind);
        visitor.visit_variable_member(&it.member);
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_variable_member<'a>(visitor: &mut impl Visit<'a>, it: &VariableMember<'a>) {
        let kind = AstKind::VariableMember;
        visitor.enter_node(kind);
        match it {
            VariableMember::Object { object, property, .. } => {
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
        visitor: &mut impl Visit<'a>,
        it: &ParenthesizedExpression<'a>,
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
    pub fn walk_block_expression<'a>(visitor: &mut impl Visit<'a>, it: &BlockExpression<'a>) {
        let kind = AstKind::BlockExpression;
        visitor.enter_node(kind);
        visitor.visit_expressions(&it.expressions);
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_binary_expression<'a>(visitor: &mut impl Visit<'a>, it: &BinaryExpression<'a>) {
        let kind = AstKind::BinaryExpression;
        visitor.enter_node(kind);
        visitor.visit_expression(&it.left);
        visitor.visit_expression(&it.right);
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_unary_expression<'a>(visitor: &mut impl Visit<'a>, it: &UnaryExpression<'a>) {
        let kind = AstKind::UnaryExpression;
        visitor.enter_node(kind);
        visitor.visit_expression(&it.argument);
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_ternary_expression<'a>(visitor: &mut impl Visit<'a>, it: &TernaryExpression<'a>) {
        let kind = AstKind::TernaryExpression;
        visitor.enter_node(kind);
        visitor.visit_expression(&it.test);
        visitor.visit_expression(&it.consequent);
        visitor.visit_expression(&it.alternate);
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_conditional_expression<'a>(
        visitor: &mut impl Visit<'a>,
        it: &ConditionalExpression<'a>,
    ) {
        let kind = AstKind::ConditionalExpression;
        visitor.enter_node(kind);
        visitor.visit_expression(&it.test);
        visitor.visit_expression(&it.consequent);
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_assignment_expression<'a>(
        visitor: &mut impl Visit<'a>,
        it: &AssignmentExpression<'a>,
    ) {
        let kind = AstKind::AssignmentExpression;
        visitor.enter_node(kind);
        visitor.visit_variable_expression(&it.left);
        visitor.visit_expression(&it.right);
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_resource_expression<'a>(visitor: &mut impl Visit<'a>, it: &ResourceExpression<'a>) {
        let kind = AstKind::ResourceExpression;
        visitor.enter_node(kind);
        visitor.visit_identifier_reference(&it.name);
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_array_access_expression<'a>(
        visitor: &mut impl Visit<'a>,
        it: &ArrayAccessExpression<'a>,
    ) {
        let kind = AstKind::ArrayAccessExpression;
        visitor.enter_node(kind);
        visitor.visit_identifier_reference(&it.name);
        visitor.visit_expression(&it.index);
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_arrow_access_expression<'a>(
        visitor: &mut impl Visit<'a>,
        it: &ArrowAccessExpression<'a>,
    ) {
        let kind = AstKind::ArrowAccessExpression;
        visitor.enter_node(kind);
        visitor.visit_expression(&it.left);
        visitor.visit_expression(&it.right);
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_call_expression<'a>(visitor: &mut impl Visit<'a>, it: &CallExpression<'a>) {
        let kind = AstKind::CallExpression;
        visitor.enter_node(kind);
        visitor.visit_identifier_reference(&it.callee);
        if let Some(args) = &it.arguments {
            visitor.visit_expressions(args);
        }
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_loop_expression<'a>(visitor: &mut impl Visit<'a>, it: &LoopExpression<'a>) {
        let kind = AstKind::LoopExpression;
        visitor.enter_node(kind);
        visitor.visit_expression(&it.count);
        visitor.visit_block_expression(&it.expression);
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_for_each_expression<'a>(visitor: &mut impl Visit<'a>, it: &ForEachExpression<'a>) {
        let kind = AstKind::ForEachExpression;
        visitor.enter_node(kind);
        visitor.visit_variable_expression(&it.variable);
        visitor.visit_expression(&it.array);
        visitor.visit_block_expression(&it.expression);
        visitor.leave_node(kind);
    }

    #[inline]
    #[allow(unused_variables)]
    pub fn walk_break<'a>(visitor: &mut impl Visit<'a>, it: &Break) {
        let kind = AstKind::Break;
        visitor.enter_node(kind);
        visitor.leave_node(kind);
    }

    #[inline]
    #[allow(unused_variables)]
    pub fn walk_continue<'a>(visitor: &mut impl Visit<'a>, it: &Continue) {
        let kind = AstKind::Continue;
        visitor.enter_node(kind);
        visitor.leave_node(kind);
    }

    #[inline]
    #[allow(unused_variables)]
    pub fn walk_this<'a>(visitor: &mut impl Visit<'a>, it: &This) {
        let kind = AstKind::This;
        visitor.enter_node(kind);
        visitor.leave_node(kind);
    }

    #[inline]
    pub fn walk_return<'a>(visitor: &mut impl Visit<'a>, it: &Return<'a>) {
        let kind = AstKind::Return;
        visitor.enter_node(kind);
        visitor.visit_expression(&it.argument);
        visitor.leave_node(kind);
    }
}
