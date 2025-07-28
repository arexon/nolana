use crate::{span::Span, token::Kind};

/// Represents the root of a Molang expression AST, containing all the top-level
/// information.
#[derive(Debug, Clone, PartialEq)]
pub struct Program<'a> {
    pub span: Span,
    pub source: &'a str,
    /// Determines whether the expression is complex or simple. If it contains
    /// at least one `;` or `=`, it is considered a complex expression.
    pub is_complex: bool,
    pub body: Vec<Statement<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement<'a> {
    Expression(Box<Expression<'a>>),
    Assignment(Box<AssignmentStatement<'a>>),
    Return(Box<ReturnStatement<'a>>),
    Break(Box<BreakStatement>),
    Continue(Box<ContinueStatement>),
}

/// `v.a = 0;`
#[derive(Debug, Clone, PartialEq)]
pub struct AssignmentStatement<'a> {
    pub span: Span,
    pub left: VariableExpression<'a>,
    pub right: Expression<'a>,
}

/// `return` in `v.a = 1; return v.a;`
#[derive(Debug, Clone, PartialEq)]
pub struct ReturnStatement<'a> {
    pub span: Span,
    pub argument: Expression<'a>,
}

/// <https://bedrock.dev/docs/stable/Molang#break>
///
/// `break` in `loop(10, { v.x = v.x + 1; (v.x > 20) ? break; });`
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BreakStatement {
    pub span: Span,
}

/// <https://bedrock.dev/docs/stable/Molang#continue>
///
/// `continue` in `loop(10, { (v.x > 5) ? continue; v.x = v.x + 1; });`
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ContinueStatement {
    pub span: Span,
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
    Loop(Box<LoopExpression<'a>>),
    ForEach(Box<ForEachExpression<'a>>),
    This(Box<ThisExpression>),
}

/// `1.23` in `v.a = 1.23;`
#[derive(Debug, Clone, PartialEq)]
pub struct NumericLiteral<'a> {
    pub span: Span,
    pub value: f32,
    pub raw: &'a str,
}

/// `true` or `false`
#[derive(Debug, Clone, PartialEq)]
pub struct BooleanLiteral {
    pub span: Span,
    pub value: bool,
}

impl BooleanLiteral {
    /// Returns `"true"` or `"false"` depending on this boolean's value.
    pub fn as_str(&self) -> &'static str {
        if self.value {
            "true"
        } else {
            "false"
        }
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

/// <https://bedrock.dev/docs/stable/Molang#Structs>
#[derive(Debug, Clone, PartialEq)]
pub enum VariableMember<'a> {
    /// `foo.bar` in `v.foo.bar`
    Object { span: Span, object: Box<VariableMember<'a>>, property: IdentifierReference<'a> },
    /// `foo` in `v.foo`
    Property { span: Span, property: IdentifierReference<'a> },
}

impl<'a> VariableMember<'a> {
    pub fn span(&self) -> Span {
        match self {
            VariableMember::Object { span, .. } => *span,
            VariableMember::Property { span, .. } => *span,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParenthesizedExpression<'a> {
    /// `(1 + 1)` in `(1 + 1) * 2`
    Single { span: Span, expression: Expression<'a> },
    /// `(v.a = 1;)` in `(v.b = 'B'; v.a = 1;);`
    Complex { span: Span, statements: Vec<Statement<'a>> },
}

impl<'a> ParenthesizedExpression<'a> {
    pub fn span(&self) -> Span {
        match self {
            ParenthesizedExpression::Single { span, .. } => *span,
            ParenthesizedExpression::Complex { span, .. } => *span,
        }
    }
}

/// `{ v.a = 0; }` in `loop(10, { v.a = 0; })`
#[derive(Debug, Clone, PartialEq)]
pub struct BlockExpression<'a> {
    pub span: Span,
    pub statements: Vec<Statement<'a>>,
}

/// `1 + 1` in `v.a = 1 + 1;`
#[derive(Debug, Clone, PartialEq)]
pub struct BinaryExpression<'a> {
    pub span: Span,
    pub left: Expression<'a>,
    pub operator: BinaryOperator,
    pub right: Expression<'a>,
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
    fn from(token: Kind) -> Self {
        match token {
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
            _ => unreachable!("Binary Operator: {token:?}"),
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
    fn from(token: Kind) -> Self {
        match token {
            Kind::Minus => Self::Negate,
            Kind::Bang => Self::Not,
            _ => unreachable!("Unary Operator: {token:?}"),
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

/// <https://bedrock.dev/docs/stable/Molang#Conditionals>
///
/// `q.foo ? 0`
#[derive(Debug, Clone, PartialEq)]
pub struct ConditionalExpression<'a> {
    pub span: Span,
    pub test: Expression<'a>,
    pub consequent: Expression<'a>,
}

/// <https://bedrock.dev/docs/stable/Molang#Resource%20Expression>
#[derive(Debug, Clone, PartialEq)]
pub struct ResourceExpression<'a> {
    pub span: Span,
    pub section: ResourceSection,
    pub name: IdentifierReference<'a>,
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

/// <https://bedrock.dev/docs/stable/Molang#-%3E%20%20Arrow%20Operator>
///
/// `v.foo->q.bar`
#[derive(Debug, Clone, PartialEq)]
pub struct ArrowAccessExpression<'a> {
    pub span: Span,
    pub left: Expression<'a>,
    pub right: Expression<'a>,
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

impl CallKind {
    /// String representation of the call kind ("math" or "query").
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Math => "math",
            Self::Query => "query",
        }
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

impl From<Kind> for CallKind {
    fn from(kind: Kind) -> Self {
        match kind {
            Kind::Math => Self::Math,
            Kind::Query => Self::Query,
            _ => unreachable!("Call Kind: {kind:?}"),
        }
    }
}

/// <https://bedrock.dev/docs/stable/Molang#loop>
///
/// `loop(10, { v.x = v.x + 1; });`
#[derive(Debug, Clone, PartialEq)]
pub struct LoopExpression<'a> {
    pub span: Span,
    pub count: Expression<'a>,
    pub block: BlockExpression<'a>,
}

/// <https://bedrock.dev/docs/stable/Molang#for_each>
///
/// `for_each(t.foo, q.baz, { v.x = v.x + 1; });`
#[derive(Debug, Clone, PartialEq)]
pub struct ForEachExpression<'a> {
    pub span: Span,
    pub variable: VariableExpression<'a>,
    pub array: Expression<'a>,
    pub block: BlockExpression<'a>,
}

/// `this` in `q.foo(this)`
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ThisExpression {
    pub span: Span,
}
