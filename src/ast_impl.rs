use crate::{ast::*, span::Span, token::Kind};

impl<'a> Program<'a> {
    pub fn new(span: Span, source: &'a str, is_complex: bool, body: Vec<Expression<'a>>) -> Self {
        Self {
            span,
            source,
            is_complex,
            body,
        }
    }
}

impl<'a> Expression<'a> {
    pub fn new_boolean_literal(span: Span, value: bool) -> Self {
        Self::BooleanLiteral(BooleanLiteral::new(span, value).into())
    }

    pub fn new_numeric_literal(span: Span, value: f32, raw: &'a str) -> Self {
        Self::NumericLiteral(NumericLiteral::new(span, value, raw).into())
    }

    pub fn new_string_literal(span: Span, value: &'a str) -> Self {
        Self::StringLiteral(StringLiteral::new(span, value).into())
    }

    pub fn new_variable(
        span: Span,
        lifetime: VariableLifetime,
        member: VariableMember<'a>,
    ) -> Self {
        Self::Variable(VariableExpression::new(span, lifetime, member).into())
    }

    pub fn new_parenthesized_single(span: Span, expression: Expression<'a>) -> Self {
        Self::Parenthesized(ParenthesizedExpression::new_single(span, expression).into())
    }

    pub fn new_parenthesized_complex(span: Span, expressions: Vec<Expression<'a>>) -> Self {
        Self::Parenthesized(ParenthesizedExpression::new_complex(span, expressions).into())
    }

    pub fn new_block(span: Span, expressions: Vec<Expression<'a>>) -> Self {
        Self::Block(BlockExpression::new(span, expressions).into())
    }

    pub fn new_binary(
        span: Span,
        left: Expression<'a>,
        operator: BinaryOperator,
        right: Expression<'a>,
    ) -> Self {
        Self::Binary(BinaryExpression::new(span, left, operator, right).into())
    }

    pub fn new_unary(span: Span, operator: UnaryOperator, argument: Expression<'a>) -> Self {
        Self::Unary(UnaryExpression::new(span, operator, argument).into())
    }

    pub fn new_ternary(
        span: Span,
        test: Expression<'a>,
        consequent: Expression<'a>,
        alternate: Expression<'a>,
    ) -> Self {
        Self::Ternary(TernaryExpression::new(span, test, consequent, alternate).into())
    }

    pub fn new_conditional(span: Span, test: Expression<'a>, consequent: Expression<'a>) -> Self {
        Self::Conditional(ConditionalExpression::new(span, test, consequent).into())
    }

    pub fn new_assignment(span: Span, left: VariableExpression<'a>, right: Expression<'a>) -> Self {
        Self::Assignment(AssignmentExpression::new(span, left, right).into())
    }

    pub fn new_resource(
        span: Span,
        section: ResourceSection,
        name: IdentifierReference<'a>,
    ) -> Self {
        Self::Resource(ResourceExpression::new(span, section, name).into())
    }

    pub fn new_array_access(
        span: Span,
        name: IdentifierReference<'a>,
        index: Expression<'a>,
    ) -> Self {
        Self::ArrayAccess(ArrayAccessExpression::new(span, name, index).into())
    }

    pub fn new_arrow_access(span: Span, left: Expression<'a>, right: Expression<'a>) -> Self {
        Self::ArrowAccess(ArrowAccessExpression::new(span, left, right).into())
    }

    pub fn new_call(
        span: Span,
        kind: CallKind,
        callee: IdentifierReference<'a>,
        arguments: Option<Vec<Expression<'a>>>,
    ) -> Self {
        Self::Call(CallExpression::new(span, kind, callee, arguments).into())
    }

    pub fn new_loop(span: Span, count: Expression<'a>, expression: BlockExpression<'a>) -> Self {
        Self::Loop(LoopExpression::new(span, count, expression).into())
    }

    pub fn new_for_each(
        span: Span,
        variable: VariableExpression<'a>,
        array: Expression<'a>,
        expression: BlockExpression<'a>,
    ) -> Self {
        Self::ForEach(ForEachExpression::new(span, variable, array, expression).into())
    }

    pub fn new_break(span: Span) -> Self {
        Self::Break(Break::new(span).into())
    }

    pub fn new_continue(span: Span) -> Self {
        Self::Continue(Continue::new(span).into())
    }

    pub fn new_this(span: Span) -> Self {
        Self::This(This::new(span).into())
    }

    pub fn new_return(span: Span, argument: Expression<'a>) -> Self {
        Self::Return(Return::new(span, argument).into())
    }

    pub fn span(&self) -> Span {
        match self {
            Expression::BooleanLiteral(expr) => expr.span,
            Expression::NumericLiteral(expr) => expr.span,
            Expression::StringLiteral(expr) => expr.span,
            Expression::Variable(expr) => expr.span,
            Expression::Parenthesized(expr) => expr.span(),
            Expression::Block(expr) => expr.span,
            Expression::Binary(expr) => expr.span,
            Expression::Unary(expr) => expr.span,
            Expression::Ternary(expr) => expr.span,
            Expression::Conditional(expr) => expr.span,
            Expression::Assignment(expr) => expr.span,
            Expression::Resource(expr) => expr.span,
            Expression::ArrayAccess(expr) => expr.span,
            Expression::ArrowAccess(expr) => expr.span,
            Expression::Call(expr) => expr.span,
            Expression::Loop(expr) => expr.span,
            Expression::ForEach(expr) => expr.span,
            Expression::Break(expr) => expr.span,
            Expression::Continue(expr) => expr.span,
            Expression::This(expr) => expr.span,
            Expression::Return(expr) => expr.span,
        }
    }
}

impl BooleanLiteral {
    pub fn new(span: Span, value: bool) -> Self {
        Self { span, value }
    }

    /// Returns `"true"` or `"false"` depending on this boolean's value.
    pub fn as_str(&self) -> &'static str {
        if self.value {
            "true"
        } else {
            "false"
        }
    }
}

impl<'a> NumericLiteral<'a> {
    pub fn new(span: Span, value: f32, raw: &'a str) -> Self {
        Self { span, value, raw }
    }
}

impl<'a> StringLiteral<'a> {
    pub fn new(span: Span, value: &'a str) -> Self {
        Self { span, value }
    }
}

impl<'a> IdentifierReference<'a> {
    pub fn new(span: Span, name: &'a str) -> Self {
        Self { span, name }
    }
}

impl<'a> VariableExpression<'a> {
    pub fn new(span: Span, lifetime: VariableLifetime, member: VariableMember<'a>) -> Self {
        Self {
            span,
            lifetime,
            member,
        }
    }
}

impl VariableLifetime {
    /// String representation of the call kind ("temp", "variable", or "context").
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Temporary => "temp",
            Self::Variable => "variable",
            Self::Context => "context",
        }
    }
}

impl From<Kind> for VariableLifetime {
    fn from(kind: Kind) -> Self {
        match kind {
            Kind::Temporary => Self::Temporary,
            Kind::Variable => Self::Variable,
            Kind::Context => Self::Context,
            _ => unreachable!("Variable Lifetime: {kind:?}"),
        }
    }
}

impl<'a> VariableMember<'a> {
    pub fn new_object(
        span: Span,
        object: Box<VariableMember<'a>>,
        property: IdentifierReference<'a>,
    ) -> Self {
        Self::Object {
            span,
            object,
            property,
        }
    }

    pub fn new_property(span: Span, property: IdentifierReference<'a>) -> Self {
        Self::Property { span, property }
    }

    pub fn span(&self) -> Span {
        match self {
            VariableMember::Object { span, .. } => *span,
            VariableMember::Property { span, .. } => *span,
        }
    }
}

impl<'a> ParenthesizedExpression<'a> {
    pub fn new_single(span: Span, expression: Expression<'a>) -> Self {
        Self::Single { span, expression }
    }

    pub fn new_complex(span: Span, expressions: Vec<Expression<'a>>) -> Self {
        Self::Complex { span, expressions }
    }

    pub fn span(&self) -> Span {
        match self {
            ParenthesizedExpression::Single { span, .. } => *span,
            ParenthesizedExpression::Complex { span, .. } => *span,
        }
    }
}

impl<'a> BlockExpression<'a> {
    pub fn new(span: Span, expressions: Vec<Expression<'a>>) -> Self {
        Self { span, expressions }
    }
}

impl<'a> BinaryExpression<'a> {
    pub fn new(
        span: Span,
        left: Expression<'a>,
        operator: BinaryOperator,
        right: Expression<'a>,
    ) -> Self {
        Self {
            span,
            left,
            operator,
            right,
        }
    }
}

impl BinaryOperator {
    /// The string representation of this operator as it appears in source code.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Equality => "==",
            Self::Inequality => "!=",
            Self::LessThan => "<",
            Self::LessEqualThan => "<=",
            Self::GreaterThan => ">",
            Self::GreaterEqualThan => ">=",
            Self::Addition => "+",
            Self::Subtraction => "-",
            Self::Multiplication => "*",
            Self::Division => "/",
            Self::Or => "||",
            Self::And => "&&",
            Self::Coalesce => "??",
        }
    }
}

impl From<Kind> for BinaryOperator {
    fn from(token: Kind) -> Self {
        match token {
            Kind::Eq => Self::Equality,
            Kind::NotEq => Self::Inequality,
            Kind::Lt => Self::LessThan,
            Kind::Gt => Self::GreaterThan,
            Kind::LtEq => Self::LessEqualThan,
            Kind::GtEq => Self::GreaterEqualThan,
            Kind::Or => Self::Or,
            Kind::And => Self::And,
            Kind::NullCoal => Self::Coalesce,
            Kind::Minus => Self::Subtraction,
            Kind::Plus => Self::Addition,
            Kind::Star => Self::Multiplication,
            Kind::Slash => Self::Division,
            _ => unreachable!("Binary Operator: {token:?}"),
        }
    }
}

impl<'a> UnaryExpression<'a> {
    pub fn new(span: Span, operator: UnaryOperator, argument: Expression<'a>) -> Self {
        Self {
            span,
            operator,
            argument,
        }
    }
}

impl UnaryOperator {
    /// The string representation of this operator as it appears in source code.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Negate => "-",
            Self::Not => "!",
        }
    }
}

impl From<Kind> for UnaryOperator {
    fn from(token: Kind) -> Self {
        match token {
            Kind::Minus => Self::Negate,
            Kind::Not => Self::Not,
            _ => unreachable!("Unary Operator: {token:?}"),
        }
    }
}

impl<'a> TernaryExpression<'a> {
    pub fn new(
        span: Span,
        test: Expression<'a>,
        consequent: Expression<'a>,
        alternate: Expression<'a>,
    ) -> Self {
        Self {
            span,
            test,
            consequent,
            alternate,
        }
    }
}

impl<'a> ConditionalExpression<'a> {
    pub fn new(span: Span, test: Expression<'a>, consequent: Expression<'a>) -> Self {
        Self {
            span,
            test,
            consequent,
        }
    }
}

impl<'a> AssignmentExpression<'a> {
    pub fn new(span: Span, left: VariableExpression<'a>, right: Expression<'a>) -> Self {
        Self { span, left, right }
    }
}

impl<'a> ResourceExpression<'a> {
    pub fn new(span: Span, section: ResourceSection, name: IdentifierReference<'a>) -> Self {
        Self {
            span,
            section,
            name,
        }
    }
}

impl ResourceSection {
    /// String representation of the resource section ("geometry", "material", or "texture").
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Geometry => "geometry",
            Self::Material => "material",
            Self::Texture => "texture",
        }
    }
}

impl From<Kind> for ResourceSection {
    fn from(kind: Kind) -> Self {
        match kind {
            Kind::Geometry => Self::Geometry,
            Kind::Material => Self::Material,
            Kind::Texture => Self::Texture,
            _ => unreachable!("Resource Section: {kind:?}"),
        }
    }
}

impl<'a> ArrayAccessExpression<'a> {
    pub fn new(span: Span, name: IdentifierReference<'a>, index: Expression<'a>) -> Self {
        Self { span, name, index }
    }
}

impl<'a> ArrowAccessExpression<'a> {
    pub fn new(span: Span, left: Expression<'a>, right: Expression<'a>) -> Self {
        Self { span, left, right }
    }
}

impl<'a> CallExpression<'a> {
    pub fn new(
        span: Span,
        kind: CallKind,
        callee: IdentifierReference<'a>,
        arguments: Option<Vec<Expression<'a>>>,
    ) -> Self {
        Self {
            span,
            kind,
            callee,
            arguments,
        }
    }
}

impl CallKind {
    /// String representation of the call kind ("math" or "query").
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Math => "math",
            Self::Query => "query",
        }
    }
}

impl From<Kind> for CallKind {
    fn from(kind: Kind) -> Self {
        match kind {
            Kind::Math => Self::Math,
            Kind::Query => Self::Query,
            _ => unreachable!("Call Kind: {kind:?}"),
        }
    }
}

impl<'a> LoopExpression<'a> {
    pub fn new(span: Span, count: Expression<'a>, expression: BlockExpression<'a>) -> Self {
        Self {
            span,
            count,
            expression,
        }
    }
}

impl<'a> ForEachExpression<'a> {
    pub fn new(
        span: Span,
        variable: VariableExpression<'a>,
        array: Expression<'a>,
        expression: BlockExpression<'a>,
    ) -> Self {
        Self {
            span,
            variable,
            array,
            expression,
        }
    }
}

impl Break {
    pub fn new(span: Span) -> Self {
        Self { span }
    }
}

impl Continue {
    pub fn new(span: Span) -> Self {
        Self { span }
    }
}

impl This {
    pub fn new(span: Span) -> Self {
        Self { span }
    }
}

impl<'a> Return<'a> {
    pub fn new(span: Span, argument: Expression<'a>) -> Self {
        Self { span, argument }
    }
}
