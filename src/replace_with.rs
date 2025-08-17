use replace_with::replace_with_or_abort;

use crate::ast::*;

/// This is a wrapper around the [`replace_with`] crate. It is implemented
/// on all AST nodes.
///
/// # Example
///
/// ```
/// # use nolana::{Codegen, ParseResult, Parser, ast::*, replace_with::ReplaceWith, span::SPAN};
///
/// fn is_less_than_32(expr: &mut Expression<'_>) {
///     if !matches!(expr, Expression::Binary(_)) {
///         return;
///     }
///     expr.replace_with(|expr| {
///         let Expression::Binary(bin_expr) = expr else { unreachable!() };
///         BinaryExpression {
///             span: SPAN,
///             left: NumericLiteral { span: SPAN, value: 32.0, raw: "32" }.into(),
///             operator: BinaryOperator::LessThan,
///             right: (*bin_expr).into(),
///         }
///         .into()
///     });
/// }
///
/// let ParseResult { mut program, .. } = Parser::new("v.a + v.b").parse();
///
/// let ProgramBody::Simple(expr) = &mut program.body else { unreachable!() };
/// is_less_than_32(expr);
///
/// let out = Codegen::default().build(&program);
/// assert_eq!(out, "32<v.a+v.b");
/// ```
pub trait ReplaceWith: Sized {
    /// Uses [`replace_with::replace_with_or_abort()`] to replace the node with
    /// a new one using derived data from the original, which is very useful in
    /// cases where owned values are needed.
    fn replace_with(&mut self, f: impl FnOnce(Self) -> Self) {
        replace_with_or_abort(self, f);
    }
}

macro_rules! impl_replace_with {
    ($( $type:ty ),* $(,)?) => {
        $(
            impl ReplaceWith for $type {}
        )*
    };
}

impl_replace_with!(
    ProgramBody<'_>,
    Expression<'_>,
    Statement<'_>,
    LoopStatement<'_>,
    ForEachStatement<'_>,
    ReturnStatement<'_>,
    BreakStatement,
    ContinueStatement,
    EmptyStatement,
    NumericLiteral<'_>,
    BooleanLiteral,
    StringLiteral<'_>,
    VariableExpression<'_>,
    VariableMember<'_>,
    ParenthesizedExpression<'_>,
    BlockExpression<'_>,
    BinaryExpression<'_>,
    UnaryExpression<'_>,
    UpdateExpression<'_>,
    TernaryExpression<'_>,
    ConditionalExpression<'_>,
    ResourceExpression<'_>,
    ArrayAccessExpression<'_>,
    ArrowAccessExpression<'_>,
    CallExpression<'_>,
    ThisExpression,
);
