use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq)]
pub struct IrProgram<'src> {
    pub body: Vec<IrStatement<'src>>,
    pub is_complex: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum IrStatement<'src> {
    Expression(Box<IrExpression<'src>>),
    Assignment(Box<IrAssignment<'src>>),
    Loop(Box<IrLoop<'src>>),
    ForEach(Box<IrForEach<'src>>),
    Return(Box<IrExpression<'src>>),
    Break,
    Continue,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IrAssignment<'src> {
    pub target: IrVariable<'src>,
    pub value: IrExpression<'src>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IrLoop<'src> {
    pub count: IrExpression<'src>,
    pub body: Vec<IrStatement<'src>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IrForEach<'src> {
    pub variable: IrVariable<'src>,
    pub array: IrExpression<'src>,
    pub body: Vec<IrStatement<'src>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum IrExpression<'src> {
    Number(f32),
    Boolean(bool),
    String(&'src str),
    Variable(Box<IrVariable<'src>>),
    Parenthesized(Box<IrExpression<'src>>),
    Block(Vec<IrStatement<'src>>),
    Binary(Box<IrBinary<'src>>),
    Unary(Box<IrUnary<'src>>),
    Ternary(Box<IrTernary<'src>>),
    Conditional(Box<IrConditional<'src>>),
    Query(Box<IrQuery<'src>>),
    Math(Box<IrMath<'src>>),
    ArrayAccess(Box<IrArrayAccess<'src>>),
    Resource(Box<IrResource<'src>>),
    Arrow(Box<IrArrowAccess<'src>>),
    This,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IrVariable<'src> {
    pub scope: IrVariableScope,
    pub name: Vec<Cow<'src, str>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum IrVariableScope {
    Variable,
    Temporary,
    Context,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IrBinary<'src> {
    pub left: IrExpression<'src>,
    pub operator: IrBinaryOperator,
    pub right: IrExpression<'src>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IrBinaryOperator {
    Equality,
    Inequality,
    LessThan,
    LessEqualThan,
    GreaterThan,
    GreaterEqualThan,
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Or,
    And,
    Coalesce,
}

impl IrBinaryOperator {
    pub fn binding_power(self) -> (u8, u8) {
        match self {
            Self::Multiplication | Self::Division => (23, 24),
            Self::Addition | Self::Subtraction => (21, 22),
            Self::LessThan | Self::GreaterThan | Self::LessEqualThan | Self::GreaterEqualThan => {
                (17, 18)
            }
            Self::Equality | Self::Inequality => (15, 16),
            Self::And => (7, 8),
            Self::Or => (5, 6),
            Self::Coalesce => (1, 2),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct IrUnary<'src> {
    pub operator: IrUnaryOperator,
    pub argument: IrExpression<'src>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IrUnaryOperator {
    Negate,
    Not,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IrTernary<'src> {
    pub test: IrExpression<'src>,
    pub consequent: IrExpression<'src>,
    pub alternate: IrExpression<'src>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IrConditional<'src> {
    pub test: IrExpression<'src>,
    pub consequent: IrExpression<'src>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IrQuery<'src> {
    pub name: Cow<'src, str>,
    pub arguments: Vec<IrExpression<'src>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IrMath<'src> {
    pub name: Cow<'src, str>,
    pub arguments: Vec<IrExpression<'src>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IrArrayAccess<'src> {
    pub name: Cow<'src, str>,
    pub index: IrExpression<'src>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IrResource<'src> {
    pub section: IrResourceSection,
    pub name: Cow<'src, str>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IrResourceSection {
    Geometry,
    Material,
    Texture,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IrArrowAccess<'src> {
    pub left: IrExpression<'src>,
    pub right: IrExpression<'src>,
}

impl<'src> From<IrAssignment<'src>> for IrStatement<'src> {
    fn from(value: IrAssignment<'src>) -> Self {
        Self::Assignment(value.into())
    }
}

impl<'src> From<IrLoop<'src>> for IrStatement<'src> {
    fn from(value: IrLoop<'src>) -> Self {
        Self::Loop(value.into())
    }
}

impl<'src> From<IrForEach<'src>> for IrStatement<'src> {
    fn from(value: IrForEach<'src>) -> Self {
        Self::ForEach(value.into())
    }
}

impl<'src> From<IrExpression<'src>> for IrStatement<'src> {
    fn from(value: IrExpression<'src>) -> Self {
        Self::Expression(value.into())
    }
}

impl<'src> From<IrVariable<'src>> for IrExpression<'src> {
    fn from(value: IrVariable<'src>) -> Self {
        Self::Variable(value.into())
    }
}

impl<'src> From<IrBinary<'src>> for IrExpression<'src> {
    fn from(value: IrBinary<'src>) -> Self {
        Self::Binary(value.into())
    }
}

impl<'src> From<IrUnary<'src>> for IrExpression<'src> {
    fn from(value: IrUnary<'src>) -> Self {
        Self::Unary(value.into())
    }
}

impl<'src> From<IrTernary<'src>> for IrExpression<'src> {
    fn from(value: IrTernary<'src>) -> Self {
        Self::Ternary(value.into())
    }
}

impl<'src> From<IrConditional<'src>> for IrExpression<'src> {
    fn from(value: IrConditional<'src>) -> Self {
        Self::Conditional(value.into())
    }
}

impl<'src> From<IrQuery<'src>> for IrExpression<'src> {
    fn from(value: IrQuery<'src>) -> Self {
        Self::Query(value.into())
    }
}

impl<'src> From<IrMath<'src>> for IrExpression<'src> {
    fn from(value: IrMath<'src>) -> Self {
        Self::Math(value.into())
    }
}

impl<'src> From<IrArrayAccess<'src>> for IrExpression<'src> {
    fn from(value: IrArrayAccess<'src>) -> Self {
        Self::ArrayAccess(value.into())
    }
}

impl<'src> From<IrResource<'src>> for IrExpression<'src> {
    fn from(value: IrResource<'src>) -> Self {
        Self::Resource(value.into())
    }
}

impl<'src> From<IrArrowAccess<'src>> for IrExpression<'src> {
    fn from(value: IrArrowAccess<'src>) -> Self {
        Self::Arrow(value.into())
    }
}
