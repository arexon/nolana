use oxc_allocator::{Allocator, Box, IntoIn, Vec};

use crate::{ast::*, span::Span};

/// Builder for creating AST nodes.
#[derive(Clone, Copy)]
pub struct AstBuilder<'a> {
    /// The arena allocator used to store AST nodes.
    pub allocator: &'a Allocator,
}

impl<'a> AstBuilder<'a> {
    #[inline]
    pub fn new(allocator: &'a Allocator) -> Self {
        Self { allocator }
    }

    #[inline]
    pub fn alloc<T>(self, value: T) -> Box<'a, T> {
        Box::new_in(value, self.allocator)
    }

    #[inline]
    pub fn vec<T>(self) -> Vec<'a, T> {
        Vec::new_in(self.allocator)
    }

    #[inline]
    pub fn program(
        self,
        span: Span,
        source: &'a str,
        is_complex: bool,
        body: Vec<'a, Expression<'a>>,
    ) -> Program<'a> {
        Program {
            span,
            source,
            is_complex,
            body,
        }
    }

    #[inline]
    pub fn identifier_reference<S>(self, span: Span, name: S) -> IdentifierReference<'a>
    where
        S: IntoIn<'a, &'a str>,
    {
        IdentifierReference {
            span,
            name: name.into_in(self.allocator),
        }
    }

    #[inline]
    pub fn expression_boolean_literal(self, span: Span, value: bool) -> Expression<'a> {
        Expression::BooleanLiteral(self.alloc(self.boolean_literal(span, value)))
    }

    #[inline]
    pub fn expression_numeric_literal<S>(self, span: Span, value: f32, raw: S) -> Expression<'a>
    where
        S: IntoIn<'a, &'a str>,
    {
        Expression::NumericLiteral(self.alloc(self.numeric_literal(span, value, raw)))
    }

    #[inline]
    pub fn expression_string_literal<S>(self, span: Span, value: S) -> Expression<'a>
    where
        S: IntoIn<'a, &'a str>,
    {
        Expression::StringLiteral(self.alloc(self.string_literal(span, value)))
    }

    #[inline]
    pub fn expression_variable(
        self,
        span: Span,
        lifetime: VariableLifetime,
        member: VariableMember<'a>,
    ) -> Expression<'a> {
        Expression::Variable(self.alloc(self.variable_expression(span, lifetime, member)))
    }

    #[inline]
    pub fn expression_parenthesized_single(
        self,
        span: Span,
        expression: Expression<'a>,
    ) -> Expression<'a> {
        Expression::Parenthesized(
            self.alloc(self.parenthesized_single_expression(span, expression)),
        )
    }

    #[inline]
    pub fn expression_parenthesized_complex(
        self,
        span: Span,
        expressions: Vec<'a, Expression<'a>>,
    ) -> Expression<'a> {
        Expression::Parenthesized(
            self.alloc(self.parenthesized_complex_expression(span, expressions)),
        )
    }

    #[inline]
    pub fn expression_block(
        self,
        span: Span,
        expressions: Vec<'a, Expression<'a>>,
    ) -> Expression<'a> {
        Expression::Block(self.alloc(self.block_expression(span, expressions)))
    }

    #[inline]
    pub fn expression_binary(
        self,
        span: Span,
        left: Expression<'a>,
        operator: BinaryOperator,
        right: Expression<'a>,
    ) -> Expression<'a> {
        Expression::Binary(self.alloc(self.binary_expression(span, left, operator, right)))
    }

    #[inline]
    pub fn expression_unary(
        self,
        span: Span,
        operator: UnaryOperator,
        argument: Expression<'a>,
    ) -> Expression<'a> {
        Expression::Unary(self.alloc(self.unary_expression(span, operator, argument)))
    }

    #[inline]
    pub fn expression_ternary(
        self,
        span: Span,
        test: Expression<'a>,
        consequent: Expression<'a>,
        alternate: Expression<'a>,
    ) -> Expression<'a> {
        Expression::Ternary(self.alloc(self.ternary_expression(span, test, consequent, alternate)))
    }

    #[inline]
    pub fn expression_conditional(
        self,
        span: Span,
        test: Expression<'a>,
        consequent: Expression<'a>,
    ) -> Expression<'a> {
        Expression::Conditional(self.alloc(self.conditional_expression(span, test, consequent)))
    }

    #[inline]
    pub fn expression_assignment(
        self,
        span: Span,
        left: VariableExpression<'a>,
        right: Expression<'a>,
    ) -> Expression<'a> {
        Expression::Assignment(self.alloc(self.assignment_expression(span, left, right)))
    }

    #[inline]
    pub fn expression_resource(
        self,
        span: Span,
        section: ResourceSection,
        name: IdentifierReference<'a>,
    ) -> Expression<'a> {
        Expression::Resource(self.alloc(self.resource_expression(span, section, name)))
    }

    #[inline]
    pub fn expression_array_access(
        self,
        span: Span,
        name: IdentifierReference<'a>,
        index: Expression<'a>,
    ) -> Expression<'a> {
        Expression::ArrayAccess(self.alloc(self.array_access_expression(span, name, index)))
    }

    #[inline]
    pub fn expression_arrow_access(
        self,
        span: Span,
        left: Expression<'a>,
        right: Expression<'a>,
    ) -> Expression<'a> {
        Expression::ArrowAccess(self.alloc(self.arrow_access_expression(span, left, right)))
    }

    #[inline]
    pub fn expression_call(
        self,
        span: Span,
        kind: CallKind,
        callee: IdentifierReference<'a>,
        arguments: Option<Vec<'a, Expression<'a>>>,
    ) -> Expression<'a> {
        Expression::Call(self.alloc(self.call_expression(span, kind, callee, arguments)))
    }

    #[inline]
    pub fn expression_loop(
        self,
        span: Span,
        count: Expression<'a>,
        expression: BlockExpression<'a>,
    ) -> Expression<'a> {
        Expression::Loop(self.alloc(self.loop_expression(span, count, expression)))
    }

    #[inline]
    pub fn expression_for_each(
        self,
        span: Span,
        variable: VariableExpression<'a>,
        array: Expression<'a>,
        expression: BlockExpression<'a>,
    ) -> Expression<'a> {
        Expression::ForEach(self.alloc(self.for_each_expression(span, variable, array, expression)))
    }

    #[inline]
    pub fn expression_break(self, span: Span) -> Expression<'a> {
        Expression::Break(self.alloc(self.r#break(span)))
    }

    #[inline]
    pub fn expression_continue(self, span: Span) -> Expression<'a> {
        Expression::Continue(self.alloc(self.r#continue(span)))
    }

    #[inline]
    pub fn expression_this(self, span: Span) -> Expression<'a> {
        Expression::This(self.alloc(self.this(span)))
    }

    #[inline]
    pub fn expression_return(self, span: Span, argument: Expression<'a>) -> Expression<'a> {
        Expression::Return(self.alloc(self.r#return(span, argument)))
    }

    #[inline]
    pub fn boolean_literal(self, span: Span, value: bool) -> BooleanLiteral {
        BooleanLiteral { span, value }
    }

    #[inline]
    pub fn numeric_literal<S>(self, span: Span, value: f32, raw: S) -> NumericLiteral<'a>
    where
        S: IntoIn<'a, &'a str>,
    {
        NumericLiteral {
            span,
            value,
            raw: raw.into_in(self.allocator),
        }
    }

    #[inline]
    pub fn string_literal<S>(self, span: Span, value: S) -> StringLiteral<'a>
    where
        S: IntoIn<'a, &'a str>,
    {
        StringLiteral {
            span,
            value: value.into_in(self.allocator),
        }
    }

    #[inline]
    pub fn variable_expression(
        self,
        span: Span,
        lifetime: VariableLifetime,
        member: VariableMember<'a>,
    ) -> VariableExpression<'a> {
        VariableExpression {
            span,
            lifetime,
            member,
        }
    }

    #[inline]
    pub fn variable_member_object(
        self,
        span: Span,
        object: VariableMember<'a>,
        property: IdentifierReference<'a>,
    ) -> VariableMember<'a> {
        VariableMember::Object {
            span,
            object: object.into_in(self.allocator),
            property,
        }
    }

    #[inline]
    pub fn variable_member_property(
        self,
        span: Span,
        property: IdentifierReference<'a>,
    ) -> VariableMember<'a> {
        VariableMember::Property { span, property }
    }

    #[inline]
    pub fn parenthesized_single_expression(
        self,
        span: Span,
        expression: Expression<'a>,
    ) -> ParenthesizedExpression<'a> {
        ParenthesizedExpression::Single { span, expression }
    }

    #[inline]
    pub fn parenthesized_complex_expression(
        self,
        span: Span,
        expressions: Vec<'a, Expression<'a>>,
    ) -> ParenthesizedExpression<'a> {
        ParenthesizedExpression::Complex { span, expressions }
    }

    #[inline]
    pub fn block_expression(
        self,
        span: Span,
        expressions: Vec<'a, Expression<'a>>,
    ) -> BlockExpression<'a> {
        BlockExpression { span, expressions }
    }

    #[inline]
    pub fn binary_expression(
        self,
        span: Span,
        left: Expression<'a>,
        operator: BinaryOperator,
        right: Expression<'a>,
    ) -> BinaryExpression<'a> {
        BinaryExpression {
            span,
            left,
            operator,
            right,
        }
    }

    #[inline]
    pub fn unary_expression(
        self,
        span: Span,
        operator: UnaryOperator,
        argument: Expression<'a>,
    ) -> UnaryExpression<'a> {
        UnaryExpression {
            span,
            operator,
            argument,
        }
    }

    #[inline]
    pub fn ternary_expression(
        self,
        span: Span,
        test: Expression<'a>,
        consequent: Expression<'a>,
        alternate: Expression<'a>,
    ) -> TernaryExpression<'a> {
        TernaryExpression {
            span,
            test,
            consequent,
            alternate,
        }
    }

    #[inline]
    pub fn conditional_expression(
        self,
        span: Span,
        test: Expression<'a>,
        consequent: Expression<'a>,
    ) -> ConditionalExpression<'a> {
        ConditionalExpression {
            span,
            test,
            consequent,
        }
    }

    #[inline]
    pub fn assignment_expression(
        self,
        span: Span,
        left: VariableExpression<'a>,
        right: Expression<'a>,
    ) -> AssignmentExpression<'a> {
        AssignmentExpression { span, left, right }
    }

    #[inline]
    pub fn resource_expression(
        self,
        span: Span,
        section: ResourceSection,
        name: IdentifierReference<'a>,
    ) -> ResourceExpression<'a> {
        ResourceExpression {
            span,
            section,
            name,
        }
    }

    #[inline]
    pub fn array_access_expression(
        self,
        span: Span,
        name: IdentifierReference<'a>,
        index: Expression<'a>,
    ) -> ArrayAccessExpression<'a> {
        ArrayAccessExpression { span, name, index }
    }

    pub fn arrow_access_expression(
        self,
        span: Span,
        left: Expression<'a>,
        right: Expression<'a>,
    ) -> ArrowAccessExpression<'a> {
        ArrowAccessExpression { span, left, right }
    }

    #[inline]
    pub fn call_expression(
        self,
        span: Span,
        kind: CallKind,
        callee: IdentifierReference<'a>,
        arguments: Option<Vec<'a, Expression<'a>>>,
    ) -> CallExpression<'a> {
        CallExpression {
            span,
            kind,
            callee,
            arguments,
        }
    }

    #[inline]
    pub fn loop_expression(
        self,
        span: Span,
        count: Expression<'a>,
        expression: BlockExpression<'a>,
    ) -> LoopExpression<'a> {
        LoopExpression {
            span,
            count,
            expression,
        }
    }

    #[inline]
    pub fn for_each_expression(
        self,
        span: Span,
        variable: VariableExpression<'a>,
        array: Expression<'a>,
        expression: BlockExpression<'a>,
    ) -> ForEachExpression<'a> {
        ForEachExpression {
            span,
            variable,
            array,
            expression,
        }
    }

    #[inline]
    pub fn r#break(self, span: Span) -> Break {
        Break { span }
    }

    #[inline]
    pub fn r#continue(self, span: Span) -> Continue {
        Continue { span }
    }

    #[inline]
    pub fn this(self, span: Span) -> This {
        This { span }
    }

    #[inline]
    pub fn r#return(self, span: Span, argument: Expression<'a>) -> Return<'a> {
        Return { span, argument }
    }
}
