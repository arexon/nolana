use logos::{Lexer, Logos};

use crate::{
    ast::*,
    diagnostic::{errors, Diagnostic, Result},
    span::Span,
    token::{Kind, Token},
};

/// Return value of [`Parser::parse`] which contains the AST and errors.
///
/// ## AST
///
/// [`program`] will always contain structurally valid AST, even if there
/// are syntax errors. However, the AST may be semantically invalid. To ensure it is valid:
///
/// 1. Check that [`errors`] is empty
/// 2. Analyze the AST semantically with [`SemanticChcker`][`crate::semantic::SemanticChecker`]
///
/// ## Errors
///
/// Nolana is able to recover from most syntax errors and continue parsing
/// anyway. When this happens:
/// 1. [`program`] will contain an AST
/// 2. [`errors`] will be non-empty
/// 3. [`panicked`] will be false
///
/// [`program`]: ParserReturn::program
/// [`errors`]: ParserReturn::errors
/// [`panicked`]: ParserReturn::panicked
#[derive(Debug)]
pub struct ParserReturn<'a> {
    pub program: Program<'a>,
    pub errors: Vec<Diagnostic>,
    pub panicked: bool,
}

/// Recursive Descent Parser for [Molang](https://bedrock.dev/docs/stable/Molang).
pub struct Parser<'a> {
    lexer: Lexer<'a, Kind>,
    source_code: &'a str,
    token: Token,
    prev_token_end: u32,
    /// An expression is considered a [`complex expression`] if there is at
    /// least one `=` or `;`.
    ///
    /// [`complex expression`]: https://bedrock.dev/docs/stable/Molang#Simple%20vs%20Complex%20Expressions
    is_complex: bool,
    errors: Vec<Diagnostic>,
}

impl<'a> Parser<'a> {
    /// Creates a new [`Parser`].
    pub fn new(source_code: &'a str) -> Self {
        Self {
            lexer: Logos::lexer(source_code),
            source_code,
            token: Token::default(),
            prev_token_end: 0,
            is_complex: false,
            errors: Vec::new(),
        }
    }

    /// Main entry point.
    ///
    /// See [`ParserReturn`] for more info.
    pub fn parse(mut self) -> ParserReturn<'a> {
        self.bump(); // First token.
        let (program, panicked) = match self.parse_program() {
            Ok(program) => (program, false),
            Err(error) => {
                self.error(error);
                let program = Program::new(Span::default(), self.source_code, false, Vec::new());
                (program, true)
            }
        };
        ParserReturn { program, errors: self.errors, panicked }
    }

    fn parse_program(&mut self) -> Result<Program<'a>> {
        let span = self.start_span();
        let mut exprs = Vec::new();
        while !self.at(Kind::Eof) {
            if let Some(stmt) = self.parse_expression_delimited_by_semi()? {
                exprs.push(stmt);
            }
        }
        Ok(Program::new(self.end_span(span), self.source_code, self.is_complex, exprs))
    }

    fn parse_expression_delimited_by_semi(&mut self) -> Result<Option<Expression<'a>>> {
        let expr = match self.current_kind() {
            Kind::Semi => None, // We skip expressions that start with `;`.
            _ => Some(self.parse_expression(0)?),
        };
        if self.eat(Kind::Semi) {
            if !self.is_complex {
                self.is_complex = true;
            }
        } else if self.is_complex {
            self.error(errors::semi_required(self.current_token().span()));
        }
        Ok(expr)
    }

    fn parse_expression(&mut self, min_bp: u8) -> Result<Expression<'a>> {
        let span = self.start_span();
        let mut lhs = match self.current_kind() {
            Kind::True | Kind::False => self.parse_literal_boolean()?,
            Kind::Number => self.parse_literal_number()?,
            Kind::String => self.parse_literal_string()?,
            Kind::Temporary | Kind::Variable | Kind::Context => {
                self.parse_variable_expression_rest()?
            }
            Kind::LeftParen => self.parse_parenthesized_expression()?,
            Kind::LeftBrace => {
                self.parse_block_expression().map(|expr| Expression::Block(expr.into()))?
            }
            Kind::Minus | Kind::Bang => self.parse_unary_expression()?,
            Kind::Query | Kind::Math => self.parse_call_expression()?,
            Kind::Geometry | Kind::Material | Kind::Texture => self.parse_resource_expression()?,
            Kind::Array => self.parse_array_access_expression()?,
            Kind::Loop => self.parse_loop_expression()?,
            Kind::ForEach => self.parse_for_each_expression()?,
            Kind::Break => self.parse_break_expression()?,
            Kind::Continue => self.parse_continue_expression()?,
            Kind::This => self.parse_this_expression()?,
            Kind::Return => self.parse_return_expression()?,
            Kind::UnterminatedString => {
                return Err(errors::unterminated_string(self.end_span(span)))
            }
            _ => return Err(errors::unexpected_token(self.current_token().span())),
        };

        loop {
            let kind = self.current_kind();

            if kind == Kind::Arrow {
                lhs = self.parse_arrow_access_expression(span, lhs)?;
                break;
            }

            let Some((lbp, rbp)) = kind.binding_power() else {
                break;
            };
            if lbp < min_bp {
                break;
            }

            match self.current_kind() {
                kind if kind.is_binary_operator() => {
                    lhs = self.parse_binary_expression(span, lhs, rbp)?;
                }
                Kind::Conditional => {
                    lhs = self.parse_ternary_or_conditional_expression(span, lhs)?;
                }
                _ => break,
            }
        }

        Ok(lhs)
    }

    fn parse_literal_boolean(&mut self) -> Result<Expression<'a>> {
        let span = self.start_span();
        let value = match self.current_kind() {
            Kind::True => true,
            Kind::False => false,
            kind => unreachable!("Boolean Literal: {kind:?}"),
        };
        self.bump();
        Ok(Expression::new_boolean_literal(self.end_span(span), value))
    }

    fn parse_literal_number(&mut self) -> Result<Expression<'a>> {
        let span = self.start_span();
        let raw = self.current_src();
        self.expect(Kind::Number)?;
        let value = raw.parse::<f32>().map_err(|_| errors::invalid_number(self.end_span(span)))?;
        Ok(Expression::new_numeric_literal(self.end_span(span), value, raw))
    }

    pub fn parse_literal_string(&mut self) -> Result<Expression<'a>> {
        let span = self.start_span();
        let value = self.current_src();
        let value = &value[1..value.len() - 1];
        self.expect(Kind::String)?;
        Ok(Expression::new_string_literal(self.end_span(span), value))
    }

    #[inline(always)] // Hot path
    fn parse_identifier_reference(&mut self) -> Result<IdentifierReference<'a>> {
        let span = self.start_span();
        let name = self.current_src();
        match self.current_kind() {
            Kind::Context | Kind::Variable | Kind::Temporary | Kind::Math | Kind::Query => {
                self.bump()
            }
            _ => self.expect(Kind::Identifier)?,
        }
        Ok(IdentifierReference::new(self.end_span(span), name))
    }

    fn parse_parenthesized_expression(&mut self) -> Result<Expression<'a>> {
        let span = self.start_span();
        self.expect(Kind::LeftParen)?;
        if self.at(Kind::RightParen) {
            let span = self.start_span();
            self.bump();
            return Err(errors::empty_parenthesized_expression(self.end_span(span)));
        }
        let expr = self.parse_expression(0)?;
        if self.eat(Kind::Semi) {
            let mut exprs = Vec::new();
            exprs.push(expr);
            loop {
                if let Some(stmt) = self.parse_expression_delimited_by_semi()? {
                    exprs.push(stmt);
                }
                if self.at(Kind::RightParen) {
                    break;
                }
            }
            self.expect(Kind::RightParen)?;
            Ok(Expression::new_parenthesized_complex(self.end_span(span), exprs))
        } else {
            self.expect(Kind::RightParen)?;
            Ok(Expression::new_parenthesized_single(self.end_span(span), expr))
        }
    }

    fn parse_block_expression(&mut self) -> Result<BlockExpression<'a>> {
        // This deviates from Molang a little bit. However, because every
        // expression inside `{}` must be delimited with a `;`, it is grammatically
        // correct to do this early.
        if !self.is_complex {
            self.is_complex = true;
        }
        let span = self.start_span();
        self.expect(Kind::LeftBrace)?;
        let mut exprs = Vec::new();
        while !self.at(Kind::RightBrace) {
            exprs.push(self.parse_expression(0)?);
            if !self.eat(Kind::Semi) {
                self.error(errors::semi_required_in_block_expression(self.current_token().span()))
            }
        }
        self.expect(Kind::RightBrace)?;
        Ok(BlockExpression::new(self.end_span(span), exprs))
    }

    fn parse_binary_expression(
        &mut self,
        left_span: Span,
        left: Expression<'a>,
        rbp: u8,
    ) -> Result<Expression<'a>> {
        let operator = self.current_kind().into();
        self.bump();
        let right = self.parse_expression(rbp)?;
        Ok(Expression::new_binary(self.end_span(left_span), left, operator, right))
    }

    fn parse_unary_expression(&mut self) -> Result<Expression<'a>> {
        let span = self.start_span();
        let operator = self.current_kind().into();
        self.bump();
        let argument = self.parse_expression(0)?;
        Ok(Expression::new_unary(self.end_span(span), operator, argument))
    }

    fn parse_ternary_or_conditional_expression(
        &mut self,
        test_span: Span,
        test: Expression<'a>,
    ) -> Result<Expression<'a>> {
        self.expect(Kind::Conditional)?;
        let consequent = self.parse_expression(0)?;
        if self.eat(Kind::Colon) {
            let alternate = self.parse_expression(0)?;
            Ok(Expression::new_ternary(self.end_span(test_span), test, consequent, alternate))
        } else {
            Ok(Expression::new_conditional(self.end_span(test_span), test, consequent))
        }
    }

    fn parse_variable_expression(&mut self) -> Result<VariableExpression<'a>> {
        let span = self.start_span();
        let lifetime: VariableLifetime = self.current_kind().into();
        self.bump();
        self.expect(Kind::Dot)?;
        let property = self.parse_identifier_reference()?;
        let mut member = VariableMember::new_property(self.end_span(span), property);
        while self.eat(Kind::Dot) {
            let property = self.parse_identifier_reference()?;
            member = VariableMember::new_object(self.end_span(span), member.into(), property);
        }
        Ok(VariableExpression::new(self.end_span(span), lifetime, member))
    }

    fn parse_variable_expression_rest(&mut self) -> Result<Expression<'a>> {
        let span = self.start_span();
        let left = self.parse_variable_expression()?;
        if self.eat(Kind::Eq) {
            if !self.is_complex {
                self.is_complex = true;
            }
            let right = self.parse_expression(0)?;
            Ok(Expression::new_assignment(self.end_span(span), left, right))
        } else {
            Ok(Expression::Variable(left.into()))
        }
    }

    fn parse_resource_expression(&mut self) -> Result<Expression<'a>> {
        let span = self.start_span();
        let section: ResourceSection = self.current_kind().into();
        self.bump();
        self.expect(Kind::Dot)?;
        let name = self.parse_identifier_reference()?;
        Ok(Expression::new_resource(self.end_span(span), section, name))
    }

    fn parse_array_access_expression(&mut self) -> Result<Expression<'a>> {
        let span = self.start_span();
        self.expect(Kind::Array)?;
        self.expect(Kind::Dot)?;
        let name = self.parse_identifier_reference()?;
        self.expect(Kind::LeftBracket)?;
        let index = self.parse_expression(0)?;
        self.expect(Kind::RightBracket)?;
        Ok(Expression::new_array_access(self.end_span(span), name, index))
    }

    fn parse_arrow_access_expression(
        &mut self,
        left_span: Span,
        left: Expression<'a>,
    ) -> Result<Expression<'a>> {
        self.expect(Kind::Arrow)?;
        let right = self.parse_expression(0)?;
        Ok(Expression::new_arrow_access(self.end_span(left_span), left, right))
    }

    fn parse_call_expression(&mut self) -> Result<Expression<'a>> {
        let span = self.start_span();
        let kind: CallKind = self.current_kind().into();
        self.bump();
        self.expect(Kind::Dot)?;
        let callee = self.parse_identifier_reference()?;
        let arguments = if self.eat(Kind::LeftParen) {
            let mut arguments = Vec::new();
            let mut first = true;
            loop {
                if self.at(Kind::RightParen) || self.at(Kind::Eof) {
                    break;
                }
                if first {
                    first = false;
                } else {
                    self.expect(Kind::Comma)?;
                    if self.at(Kind::RightParen) {
                        break;
                    }
                }
                arguments.push(self.parse_expression(0)?);
            }
            self.expect(Kind::RightParen)?;
            Some(arguments)
        } else {
            None
        };
        Ok(Expression::new_call(self.end_span(span), kind, callee, arguments))
    }

    fn parse_loop_expression(&mut self) -> Result<Expression<'a>> {
        let span = self.start_span();
        self.expect(Kind::Loop)?;
        self.expect(Kind::LeftParen)?;
        let count = self.parse_expression(0)?;
        self.expect(Kind::Comma)?;
        let expr = self.parse_block_expression()?;
        self.expect(Kind::RightParen)?;
        Ok(Expression::new_loop(self.end_span(span), count, expr))
    }

    fn parse_for_each_expression(&mut self) -> Result<Expression<'a>> {
        let span = self.start_span();
        self.expect(Kind::ForEach)?;
        self.expect(Kind::LeftParen)?;
        if !(self.at(Kind::Variable) || self.at(Kind::Temporary)) {
            return Err(errors::for_each_wrong_first_arg(self.current_token().span()));
        }
        let variable = self.parse_variable_expression()?;
        self.expect(Kind::Comma)?;
        let array = self.parse_expression(0)?;
        self.expect(Kind::Comma)?;
        let expr = self.parse_block_expression()?;
        self.expect(Kind::RightParen)?;
        Ok(Expression::new_for_each(self.end_span(span), variable, array, expr))
    }

    fn parse_break_expression(&mut self) -> Result<Expression<'a>> {
        let span = self.start_span();
        self.expect(Kind::Break)?;
        Ok(Expression::new_break(self.end_span(span)))
    }

    fn parse_continue_expression(&mut self) -> Result<Expression<'a>> {
        let span = self.start_span();
        self.expect(Kind::Continue)?;
        Ok(Expression::new_continue(self.end_span(span)))
    }

    fn parse_this_expression(&mut self) -> Result<Expression<'a>> {
        let span = self.start_span();
        self.expect(Kind::This)?;
        Ok(Expression::new_this(self.end_span(span)))
    }

    fn parse_return_expression(&mut self) -> Result<Expression<'a>> {
        let span = self.start_span();
        self.expect(Kind::Return)?;
        let argument = self.parse_expression(0)?;
        Ok(Expression::new_return(self.end_span(span), argument))
    }

    #[inline]
    fn current_token(&self) -> Token {
        self.token
    }

    #[inline]
    fn current_kind(&self) -> Kind {
        self.token.kind
    }

    #[inline]
    fn current_src(&self) -> &'a str {
        self.lexer.slice()
    }

    #[inline]
    fn start_span(&self) -> Span {
        Span::new(self.token.start, 0)
    }

    #[inline]
    fn end_span(&self, mut span: Span) -> Span {
        span.end = self.prev_token_end;
        debug_assert!(span.end >= span.start);
        span
    }

    #[inline]
    fn at(&self, kind: Kind) -> bool {
        self.current_kind() == kind
    }

    #[inline(always)] // Hot path
    fn bump(&mut self) {
        self.prev_token_end = self.token.end;
        let kind = self.lexer.next().unwrap_or(Ok(Kind::Eof)).unwrap_or(Kind::UnterminatedString);
        let span = self.lexer.span();
        self.token = Token { kind, start: span.start as u32, end: span.end as u32 };
    }

    #[inline]
    fn eat(&mut self, kind: Kind) -> bool {
        if self.at(kind) {
            self.bump();
            return true;
        }
        false
    }

    #[inline(always)] // Hot path
    fn expect(&mut self, kind: Kind) -> Result<()> {
        if !self.eat(kind) {
            return Err(self.expected_token(kind));
        }
        Ok(())
    }

    fn expected_token(&self, kind: Kind) -> Diagnostic {
        let curr_token = self.current_token();
        errors::expected_token(kind.to_str(), curr_token.kind.to_str(), curr_token.span())
    }

    fn error(&mut self, error: Diagnostic) {
        self.errors.push(error);
    }
}
