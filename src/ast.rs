use crate::span::Span;

/// Untyped AST Node kind.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AstKind {
    Program,
    BooleanLiteral,
    NumericLiteral,
    StringLiteral,
    IdentifierReference,
    VariableExpression,
    VariableMember,
    ParenthesizedExpression,
    BlockExpression,
    BinaryExpression,
    UnaryExpression,
    TernaryExpression,
    ConditionalExpression,
    AssignmentExpression,
    ResourceExpression,
    ArrayAccessExpression,
    ArrowAccessExpression,
    CallExpression,
    LoopExpression,
    ForEachExpression,
    Break,
    Continue,
    This,
    Return,
}

/// Represents the root of a Molang expression AST, containing all the top-level
/// information.
#[derive(Debug, Clone, PartialEq)]
pub struct Program<'a> {
    pub span: Span,
    pub source: &'a str,
    /// Determines whether the expression is complex or simple. If it contains
    /// at least one `;` or `=`, it is considered a complex expression.
    pub is_complex: bool,
    pub body: Vec<Expression<'a>>,
}

/// <https://bedrock.dev/docs/stable/Molang#Lexical%20Structure>
#[derive(Debug, Clone, PartialEq)]
pub enum Expression<'a> {
    BooleanLiteral(Box<BooleanLiteral>),
    NumericLiteral(Box<NumericLiteral<'a>>),
    StringLiteral(Box<StringLiteral<'a>>),
    Variable(Box<VariableExpression<'a>>),
    Parenthesized(Box<ParenthesizedExpression<'a>>),
    Block(Box<BlockExpression<'a>>),
    Binary(Box<BinaryExpression<'a>>),
    Unary(Box<UnaryExpression<'a>>),
    Ternary(Box<TernaryExpression<'a>>),
    Conditional(Box<ConditionalExpression<'a>>),
    Assignment(Box<AssignmentExpression<'a>>),
    Resource(Box<ResourceExpression<'a>>),
    ArrayAccess(Box<ArrayAccessExpression<'a>>),
    ArrowAccess(Box<ArrowAccessExpression<'a>>),
    Call(Box<CallExpression<'a>>),
    Loop(Box<LoopExpression<'a>>),
    ForEach(Box<ForEachExpression<'a>>),
    Break(Box<Break>),
    Continue(Box<Continue>),
    This(Box<This>),
    Return(Box<Return<'a>>),
}

/// `true` or `false`
#[derive(Debug, Clone, PartialEq)]
pub struct BooleanLiteral {
    pub span: Span,
    pub value: bool,
}

/// `1.23` in `v.a = 1.23;`
#[derive(Debug, Clone, PartialEq)]
pub struct NumericLiteral<'a> {
    pub span: Span,
    pub value: f32,
    pub raw: &'a str,
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

/// <https://bedrock.dev/docs/stable/Molang#Structs>
#[derive(Debug, Clone, PartialEq)]
pub enum VariableMember<'a> {
    /// `foo.bar` in `v.foo.bar`
    Object { span: Span, object: Box<VariableMember<'a>>, property: IdentifierReference<'a> },
    /// `foo` in `v.foo`
    Property { span: Span, property: IdentifierReference<'a> },
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParenthesizedExpression<'a> {
    /// `(1 + 1)` in `(1 + 1) * 2`
    Single { span: Span, expression: Expression<'a> },
    /// `(v.a = 1;)` in `(v.b = 'B'; v.a = 1;);`
    Complex { span: Span, expressions: Vec<Expression<'a>> },
}

/// `{ v.a = 0; }` in `loop(10, { v.a = 0; })`
#[derive(Debug, Clone, PartialEq)]
pub struct BlockExpression<'a> {
    pub span: Span,
    pub expressions: Vec<Expression<'a>>,
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

/// `v.a = 0;`
#[derive(Debug, Clone, PartialEq)]
pub struct AssignmentExpression<'a> {
    pub span: Span,
    pub left: VariableExpression<'a>,
    pub right: Expression<'a>,
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

/// The call kind for [`CallExpression`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CallKind {
    /// `math` in `math.foo`
    Math,
    /// `query` in `query.foo`
    Query,
}

/// <https://bedrock.dev/docs/stable/Molang#loop>
///
/// `loop(10, { v.x = v.x + 1; });`
#[derive(Debug, Clone, PartialEq)]
pub struct LoopExpression<'a> {
    pub span: Span,
    pub count: Expression<'a>,
    pub expression: BlockExpression<'a>,
}

/// <https://bedrock.dev/docs/stable/Molang#for_each>
///
/// `for_each(t.foo, q.baz, { v.x = v.x + 1; });`
#[derive(Debug, Clone, PartialEq)]
pub struct ForEachExpression<'a> {
    pub span: Span,
    pub variable: VariableExpression<'a>,
    pub array: Expression<'a>,
    pub expression: BlockExpression<'a>,
}

/// <https://bedrock.dev/docs/stable/Molang#break>
///
/// `break` in `loop(10, { v.x = v.x + 1; (v.x > 20) ? break; });`
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Break {
    pub span: Span,
}

/// <https://bedrock.dev/docs/stable/Molang#continue>
///
/// `continue` in `loop(10, { (v.x > 5) ? continue; v.x = v.x + 1; });`
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Continue {
    pub span: Span,
}

/// `this` in `q.foo(this)`
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct This {
    pub span: Span,
}

/// `return` in `v.a = 1; return v.a;`
#[derive(Debug, Clone, PartialEq)]
pub struct Return<'a> {
    pub span: Span,
    pub argument: Expression<'a>,
}
