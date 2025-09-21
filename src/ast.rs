use crate::{span::Span, token::Kind};

/// Represents the root of a Molang expression AST, containing all the top-level
/// information.
#[derive(Debug, Clone, PartialEq)]
pub struct Program<'a> {
    pub span: Span,
    pub source: &'a str,
    pub body: ProgramBody<'a>,
}

/// A program is considered complex if it contains any statement.
#[derive(Debug, Clone, PartialEq)]
pub enum ProgramBody<'a> {
    Simple(Expression<'a>),
    Complex(Vec<Statement<'a>>),
    Empty,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement<'a> {
    Expression(Box<Expression<'a>>),
    Assignment(Box<AssignmentStatement<'a>>),
    Loop(Box<LoopStatement<'a>>),
    ForEach(Box<ForEachStatement<'a>>),
    Return(Box<ReturnStatement<'a>>),
    Break(Box<BreakStatement>),
    Continue(Box<ContinueStatement>),
    Empty(Box<EmptyStatement>),
}

/// `v.a = 0;`
#[derive(Debug, Clone, PartialEq)]
pub struct AssignmentStatement<'a> {
    pub span: Span,
    pub left: VariableExpression<'a>,
    pub right: Expression<'a>,
}

impl<'a> From<AssignmentStatement<'a>> for Statement<'a> {
    fn from(value: AssignmentStatement<'a>) -> Self {
        Self::Assignment(value.into())
    }
}

/// <https://bedrock.dev/docs/stable/Molang#loop>
///
/// `loop(10, { v.x = v.x + 1; });`
#[derive(Debug, Clone, PartialEq)]
pub struct LoopStatement<'a> {
    pub span: Span,
    pub count: Expression<'a>,
    pub block: BlockExpression<'a>,
}

impl<'a> From<LoopStatement<'a>> for Statement<'a> {
    fn from(value: LoopStatement<'a>) -> Self {
        Self::Loop(value.into())
    }
}

/// <https://bedrock.dev/docs/stable/Molang#for_each>
///
/// `for_each(t.foo, q.baz, { v.x = v.x + 1; });`
#[derive(Debug, Clone, PartialEq)]
pub struct ForEachStatement<'a> {
    pub span: Span,
    pub variable: VariableExpression<'a>,
    pub array: Expression<'a>,
    pub block: BlockExpression<'a>,
}

impl<'a> From<ForEachStatement<'a>> for Statement<'a> {
    fn from(value: ForEachStatement<'a>) -> Self {
        Self::ForEach(value.into())
    }
}

/// `return` in `v.a = 1; return v.a;`
#[derive(Debug, Clone, PartialEq)]
pub struct ReturnStatement<'a> {
    pub span: Span,
    pub argument: Expression<'a>,
}

impl<'a> From<ReturnStatement<'a>> for Statement<'a> {
    fn from(value: ReturnStatement<'a>) -> Self {
        Self::Return(value.into())
    }
}

/// <https://bedrock.dev/docs/stable/Molang#break>
///
/// `break` in `loop(10, { v.x = v.x + 1; (v.x > 20) ? break; });`
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BreakStatement {
    pub span: Span,
}

impl From<BreakStatement> for Statement<'_> {
    fn from(value: BreakStatement) -> Self {
        Self::Break(value.into())
    }
}

/// <https://bedrock.dev/docs/stable/Molang#continue>
///
/// `continue` in `loop(10, { (v.x > 5) ? continue; v.x = v.x + 1; });`
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ContinueStatement {
    pub span: Span,
}

impl From<ContinueStatement> for Statement<'_> {
    fn from(value: ContinueStatement) -> Self {
        Self::Continue(value.into())
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EmptyStatement {
    pub span: Span,
}

impl From<EmptyStatement> for Statement<'_> {
    fn from(value: EmptyStatement) -> Self {
        Self::Empty(value.into())
    }
}

/// <https://bedrock.dev/docs/stable/Molang#Lexical%20Structure>
#[derive(Debug, Clone, PartialEq)]
pub enum Expression<'a> {
    NumericLiteral(Box<NumericLiteral<'a>>),
    BooleanLiteral(Box<BooleanLiteral>),
    StringLiteral(Box<StringLiteral<'a>>),
    Variable(Box<VariableExpression<'a>>),
    Parenthesized(Box<ParenthesizedExpression<'a>>),
    Block(Box<BlockExpression<'a>>),
    Binary(Box<BinaryExpression<'a>>),
    Unary(Box<UnaryExpression<'a>>),
    Ternary(Box<TernaryExpression<'a>>),
    Conditional(Box<ConditionalExpression<'a>>),
    Resource(Box<ResourceExpression<'a>>),
    ArrayAccess(Box<ArrayAccessExpression<'a>>),
    ArrowAccess(Box<ArrowAccessExpression<'a>>),
    Call(Box<CallExpression<'a>>),
    This(Box<ThisExpression>),
}

impl<'a> From<Expression<'a>> for Statement<'a> {
    fn from(value: Expression<'a>) -> Self {
        Self::Expression(value.into())
    }
}

/// `1.23` in `v.a = 1.23;`
#[derive(Debug, Clone, PartialEq)]
pub struct NumericLiteral<'a> {
    pub span: Span,
    pub value: f32,
    pub raw: &'a str,
}

impl<'a> From<NumericLiteral<'a>> for Expression<'a> {
    fn from(value: NumericLiteral<'a>) -> Self {
        Self::NumericLiteral(value.into())
    }
}

/// `true` or `false`
#[derive(Debug, Clone, PartialEq)]
pub struct BooleanLiteral {
    pub span: Span,
    pub value: bool,
}

impl From<BooleanLiteral> for Expression<'_> {
    fn from(value: BooleanLiteral) -> Self {
        Self::BooleanLiteral(value.into())
    }
}

impl BooleanLiteral {
    /// Returns `"true"` or `"false"` depending on this boolean's value.
    pub fn as_str(&self) -> &'static str {
        if self.value { "true" } else { "false" }
    }
}

/// <https://bedrock.dev/docs/stable/Molang#Strings>
///
/// `'foo bar'` in `v.a = 'foo bar';`
#[derive(Debug, Clone, PartialEq)]
pub struct StringLiteral<'a> {
    pub span: Span,
    pub value: &'a str,
}

impl<'a> From<StringLiteral<'a>> for Expression<'a> {
    fn from(value: StringLiteral<'a>) -> Self {
        Self::StringLiteral(value.into())
    }
}

/// `foo` in `v.foo.bar`
#[derive(Debug, Clone, PartialEq)]
pub struct IdentifierReference<'a> {
    pub span: Span,
    pub name: &'a str,
}

/// <https://bedrock.dev/docs/stable/Molang#Variables>
#[derive(Debug, Clone, PartialEq)]
pub struct VariableExpression<'a> {
    pub span: Span,
    pub lifetime: VariableLifetime,
    pub member: VariableMember<'a>,
}

impl<'a> From<VariableExpression<'a>> for Expression<'a> {
    fn from(value: VariableExpression<'a>) -> Self {
        Self::Variable(value.into())
    }
}

/// The variable lifetime associated with [`VariableExpression`].
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum VariableLifetime {
    /// `temp` in `temp.foo`
    Temporary,
    /// `variable` in `variable.foo`
    Variable,
    /// `context` in `context.foo`
    Context,
}

impl VariableLifetime {
    pub fn as_str_long(&self) -> &'static str {
        match self {
            Self::Temporary => "temp",
            Self::Variable => "variable",
            Self::Context => "context",
        }
    }

    pub fn as_str_short(&self) -> &'static str {
        match self {
            Self::Temporary => "t",
            Self::Variable => "v",
            Self::Context => "c",
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

/// <https://bedrock.dev/docs/stable/Molang#Structs>
#[derive(Debug, Clone, PartialEq)]
pub enum VariableMember<'a> {
    /// `foo.bar` in `v.foo.bar`
    Object { object: Box<VariableMember<'a>>, property: IdentifierReference<'a> },
    /// `foo` in `v.foo`
    Property { property: IdentifierReference<'a> },
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParenthesizedExpression<'a> {
    pub span: Span,
    pub body: ParenthesizedBody<'a>,
}

impl<'a> From<ParenthesizedExpression<'a>> for Expression<'a> {
    fn from(value: ParenthesizedExpression<'a>) -> Self {
        Self::Parenthesized(value.into())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParenthesizedBody<'a> {
    /// `(1 + 1)` in `(1 + 1) * 2`
    Single(Expression<'a>),
    /// `(v.a = 1;)` in `(v.b = 'B'; v.a = 1;);`
    Multiple(Vec<Statement<'a>>),
}

/// `{ v.a = 0; }` in `loop(10, { v.a = 0; })`
#[derive(Debug, Clone, PartialEq)]
pub struct BlockExpression<'a> {
    pub span: Span,
    pub statements: Vec<Statement<'a>>,
}

impl<'a> From<BlockExpression<'a>> for Expression<'a> {
    fn from(value: BlockExpression<'a>) -> Self {
        Self::Block(value.into())
    }
}

/// `1 + 1` in `v.a = 1 + 1;`
#[derive(Debug, Clone, PartialEq)]
pub struct BinaryExpression<'a> {
    pub span: Span,
    pub left: Expression<'a>,
    pub operator: BinaryOperator,
    pub right: Expression<'a>,
}

impl<'a> From<BinaryExpression<'a>> for Expression<'a> {
    fn from(value: BinaryExpression<'a>) -> Self {
        Self::Binary(value.into())
    }
}

/// Operators used in [`BinaryExpression`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOperator {
    /// `==`
    Equality,
    /// `!=`
    Inequality,
    /// `<`
    LessThan,
    /// `<=`
    LessEqualThan,
    /// `>`
    GreaterThan,
    /// `>=`
    GreaterEqualThan,
    /// `+`
    Addition,
    /// `-`
    Subtraction,
    /// `*`
    Multiplication,
    /// `/`
    Division,
    /// `||`
    Or,
    /// `&&`
    And,
    /// `??`
    Coalesce,
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
    fn from(kind: Kind) -> Self {
        match kind {
            Kind::Eq2 => Self::Equality,
            Kind::Neq => Self::Inequality,
            Kind::Lt => Self::LessThan,
            Kind::Gt => Self::GreaterThan,
            Kind::LtEq => Self::LessEqualThan,
            Kind::GtEq => Self::GreaterEqualThan,
            Kind::Pipe2 => Self::Or,
            Kind::Amp2 => Self::And,
            Kind::Question2 => Self::Coalesce,
            Kind::Minus => Self::Subtraction,
            Kind::Plus => Self::Addition,
            Kind::Star => Self::Multiplication,
            Kind::Slash => Self::Division,
            _ => unreachable!("Binary Operator: {kind:?}"),
        }
    }
}

/// `-1` in `q.foo(-1)`
#[derive(Debug, Clone, PartialEq)]
pub struct UnaryExpression<'a> {
    pub span: Span,
    pub operator: UnaryOperator,
    pub argument: Expression<'a>,
}

impl<'a> From<UnaryExpression<'a>> for Expression<'a> {
    fn from(value: UnaryExpression<'a>) -> Self {
        Self::Unary(value.into())
    }
}

/// Operators used in [`UnaryExpression`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOperator {
    /// `-`
    Negate,
    /// `!`
    Not,
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
    fn from(kind: Kind) -> Self {
        match kind {
            Kind::Minus => Self::Negate,
            Kind::Bang => Self::Not,
            _ => unreachable!("Unary Operator: {kind:?}"),
        }
    }
}

/// <https://bedrock.dev/docs/stable/Molang#Conditionals>
///
/// `q.foo ? 0 : 1`
#[derive(Debug, Clone, PartialEq)]
pub struct TernaryExpression<'a> {
    pub span: Span,
    pub test: Expression<'a>,
    pub consequent: Expression<'a>,
    pub alternate: Expression<'a>,
}

impl<'a> From<TernaryExpression<'a>> for Expression<'a> {
    fn from(value: TernaryExpression<'a>) -> Self {
        Self::Ternary(value.into())
    }
}

/// <https://bedrock.dev/docs/stable/Molang#Conditionals>
///
/// `q.foo ? 0`
#[derive(Debug, Clone, PartialEq)]
pub struct ConditionalExpression<'a> {
    pub span: Span,
    pub test: Expression<'a>,
    pub consequent: Expression<'a>,
}

impl<'a> From<ConditionalExpression<'a>> for Expression<'a> {
    fn from(value: ConditionalExpression<'a>) -> Self {
        Self::Conditional(value.into())
    }
}

/// <https://bedrock.dev/docs/stable/Molang#Resource%20Expression>
#[derive(Debug, Clone, PartialEq)]
pub struct ResourceExpression<'a> {
    pub span: Span,
    pub section: ResourceSection,
    pub name: IdentifierReference<'a>,
}

impl<'a> From<ResourceExpression<'a>> for Expression<'a> {
    fn from(value: ResourceExpression<'a>) -> Self {
        Self::Resource(value.into())
    }
}

/// The resource section in [`ResourceExpression`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourceSection {
    /// `geometry` in `geometry.foo`
    Geometry,
    /// `material` in `material.foo`
    Material,
    /// `texture` in `texture.foo`
    Texture,
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

/// <https://bedrock.dev/docs/stable/Molang#Array%20Expressions>
///
/// `array.foo[0]`
#[derive(Debug, Clone, PartialEq)]
pub struct ArrayAccessExpression<'a> {
    pub span: Span,
    pub name: IdentifierReference<'a>,
    pub index: Expression<'a>,
}

impl<'a> From<ArrayAccessExpression<'a>> for Expression<'a> {
    fn from(value: ArrayAccessExpression<'a>) -> Self {
        Self::ArrayAccess(value.into())
    }
}

/// <https://bedrock.dev/docs/stable/Molang#-%3E%20%20Arrow%20Operator>
///
/// `v.foo->q.bar`
#[derive(Debug, Clone, PartialEq)]
pub struct ArrowAccessExpression<'a> {
    pub span: Span,
    pub left: Expression<'a>,
    pub right: Expression<'a>,
}

impl<'a> From<ArrowAccessExpression<'a>> for Expression<'a> {
    fn from(value: ArrowAccessExpression<'a>) -> Self {
        Self::ArrowAccess(value.into())
    }
}

/// <https://bedrock.dev/docs/stable/Molang#Lexical%20Structure>
/// <https://bedrock.dev/docs/stable/Molang#Math%20Functions>
///
/// `math.random(1, 2)` or `math.random`
#[derive(Debug, Clone, PartialEq)]
pub struct CallExpression<'a> {
    pub span: Span,
    pub kind: CallKind,
    pub callee: IdentifierReference<'a>,
    pub arguments: Option<Vec<Expression<'a>>>,
}

impl<'a> From<CallExpression<'a>> for Expression<'a> {
    fn from(value: CallExpression<'a>) -> Self {
        Self::Call(value.into())
    }
}

/// The call kind for [`CallExpression`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CallKind {
    /// `math` in `math.foo`
    Math,
    /// `query` in `query.foo`
    Query,
}

impl CallKind {
    pub fn as_str_long(&self) -> &'static str {
        match self {
            Self::Math => "math",
            Self::Query => "query",
        }
    }

    pub fn as_str_short(&self) -> &'static str {
        match self {
            Self::Math => "math",
            Self::Query => "q",
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

/// `this` in `q.foo(this)`
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ThisExpression {
    pub span: Span,
}

impl From<ThisExpression> for Expression<'_> {
    fn from(value: ThisExpression) -> Self {
        Self::This(value.into())
    }
}
