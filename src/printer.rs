use std::iter;

use crate::ir::*;

pub struct PrinterOptions {
    pub minify: bool,
}

impl Default for PrinterOptions {
    fn default() -> Self {
        Self { minify: true }
    }
}

#[derive(Default)]
pub struct Printer {
    options: PrinterOptions,
    code: String,
    is_complex: bool,
    indent: usize,
}

impl Printer {
    pub fn print(mut self, program: &IrProgram) -> String {
        self.print_program(program);
        self.code
    }

    pub fn with_options(mut self, options: PrinterOptions) -> Self {
        self.options = options;
        self
    }

    fn print_program(&mut self, program: &IrProgram) {
        self.is_complex = program.is_complex;
        for stmt in &program.body {
            self.print_statement(stmt);
        }
    }

    fn print_statement(&mut self, stmt: &IrStatement) {
        self.print_indent();
        match stmt {
            IrStatement::Expression(expr) => self.print_expression(expr),
            IrStatement::Assignment(stmt) => self.print_assignment(stmt),
            IrStatement::Loop(stmt) => self.print_loop(stmt),
            IrStatement::ForEach(stmt) => self.print_for_each(stmt),
            IrStatement::Return(expr) => {
                self.print_str("return ");
                self.print_expression(expr);
            }
            IrStatement::Break => self.print_str("break"),
            IrStatement::Continue => self.print_str("continue"),
        }
        if self.is_complex {
            self.print_semi();
            self.print_newline();
        }
    }

    fn print_assignment(&mut self, stmt: &IrAssignment) {
        self.print_variable(&stmt.target);
        self.print_space();
        self.print_char('=');
        self.print_space();
        self.print_expression(&stmt.value);
    }

    fn print_loop(&mut self, stmt: &IrLoop) {
        self.print_str("loop");
        self.print_wrapped('(', ')', |c| {
            c.print_expression(&stmt.count);
            c.print_comma();
            c.print_space();
            c.print_block(&stmt.body);
        });
    }

    fn print_for_each(&mut self, stmt: &IrForEach) {
        self.print_str("for_each");
        self.print_scope('(', ')', |c| {
            c.print_variable(&stmt.variable);
            c.print_comma();
            c.print_space();
            c.print_expression(&stmt.array);
            c.print_comma();
            c.print_space();
            c.print_block(&stmt.body);
        });
    }

    fn print_expression(&mut self, expr: &IrExpression) {
        match expr {
            IrExpression::Number(n) => self.print_str(&n.to_string()),
            IrExpression::Boolean(b) => self.print_str(if *b { "true" } else { "false" }),
            IrExpression::String(s) => self.print_wrapped('\'', '\'', |c| c.print_str(s)),
            IrExpression::Variable(expr) => self.print_variable(expr),
            IrExpression::Parenthesized(expr) => {
                self.print_wrapped('(', ')', |c| c.print_expression(expr))
            }
            IrExpression::Block(stmts) => self.print_block(&**stmts),
            IrExpression::Binary(expr) => self.print_binary(expr),
            IrExpression::Unary(expr) => self.print_unary(expr),
            IrExpression::Ternary(expr) => self.print_ternary(expr),
            IrExpression::Conditional(expr) => self.print_conditional(expr),
            IrExpression::Query(expr) => self.print_query(expr),
            IrExpression::Math(expr) => self.print_math(expr),
            IrExpression::ArrayAccess(expr) => self.print_array_access(expr),
            IrExpression::Resource(expr) => self.print_resource(expr),
            IrExpression::Arrow(expr) => self.print_arrow_access(expr),
            IrExpression::This => self.print_str("this"),
        }
    }

    fn print_variable(&mut self, expr: &IrVariable) {
        self.print_str(match expr.scope {
            IrVariableScope::Variable if self.options.minify => "v",
            IrVariableScope::Variable => "variable",
            IrVariableScope::Temporary if self.options.minify => "t",
            IrVariableScope::Temporary => "temp",
            IrVariableScope::Context if self.options.minify => "c",
            IrVariableScope::Context => "context",
        });
        self.print_dot();
        let mut parts = expr.name.iter();
        if let Some(first) = parts.next() {
            self.print_str(first);
            for part in parts {
                self.print_dot();
                self.print_str(part);
            }
        }
    }

    fn print_binary(&mut self, expr: &IrBinary) {
        self.print_expression(&expr.left);
        self.print_space();
        self.print_str(match expr.operator {
            IrBinaryOperator::Equality => "==",
            IrBinaryOperator::Inequality => "!=",
            IrBinaryOperator::LessThan => "<",
            IrBinaryOperator::LessEqualThan => "<=",
            IrBinaryOperator::GreaterThan => ">",
            IrBinaryOperator::GreaterEqualThan => ">=",
            IrBinaryOperator::Addition => "+",
            IrBinaryOperator::Subtraction => "-",
            IrBinaryOperator::Multiplication => "*",
            IrBinaryOperator::Division => "/",
            IrBinaryOperator::Or => "||",
            IrBinaryOperator::And => "&&",
            IrBinaryOperator::Coalesce => "??",
        });
        self.print_space();
        self.print_expression(&expr.right);
    }

    fn print_unary(&mut self, expr: &IrUnary) {
        self.print_str(match expr.operator {
            IrUnaryOperator::Negate => "-",
            IrUnaryOperator::Not => "!",
        });
        self.print_expression(&expr.argument);
    }

    fn print_ternary(&mut self, expr: &IrTernary) {
        self.print_expression(&expr.test);
        self.print_space();
        self.print_question();
        self.print_space();
        self.print_expression(&expr.consequent);
        self.print_space();
        self.print_colon();
        self.print_space();
        self.print_expression(&expr.alternate);
    }

    fn print_conditional(&mut self, expr: &IrConditional) {
        self.print_expression(&expr.test);
        self.print_space();
        self.print_question();
        self.print_space();
        self.print_expression(&expr.consequent);
    }

    fn print_query(&mut self, expr: &IrQuery) {
        self.print_str(if self.options.minify { "q" } else { "query" });
        self.print_dot();
        self.print_str(&expr.name);
        if !expr.arguments.is_empty() {
            self.print_argument_list(&expr.arguments);
        }
    }

    fn print_math(&mut self, expr: &IrMath) {
        self.print_str("math");
        self.print_dot();
        self.print_str(&expr.name);
        self.print_argument_list(&expr.arguments);
    }

    fn print_array_access(&mut self, expr: &IrArrayAccess) {
        self.print_str("array");
        self.print_dot();
        self.print_str(&expr.name);
        self.print_wrapped('[', ']', |c| c.print_expression(&expr.index));
    }

    fn print_arrow_access(&mut self, expr: &IrArrowAccess) {
        self.print_expression(&expr.left);
        self.print_str("->");
        self.print_expression(&expr.right);
    }

    fn print_resource(&mut self, expr: &IrResource) {
        self.print_str(match expr.section {
            IrResourceSection::Geometry => "geometry",
            IrResourceSection::Material => "material",
            IrResourceSection::Texture => "texture",
        });
        self.print_dot();
        self.print_str(&expr.name);
    }

    #[inline]
    fn indent(&mut self) {
        self.indent += 1;
    }

    #[inline]
    fn dedent(&mut self) {
        self.indent -= 1;
    }

    #[inline]
    fn print_indent(&mut self) {
        if !self.options.minify && self.is_complex {
            self.code.extend(iter::repeat_n("    ", self.indent))
        }
    }

    #[inline]
    fn print_str(&mut self, s: &str) {
        self.code.push_str(s);
    }

    #[inline]
    fn print_char(&mut self, ch: char) {
        self.code.push(ch);
    }

    #[inline]
    fn print_newline(&mut self) {
        if !self.options.minify {
            self.code.push('\n');
        }
    }

    #[inline]
    fn print_space(&mut self) {
        if !self.options.minify {
            self.code.push(' ');
        }
    }

    #[inline]
    fn print_dot(&mut self) {
        self.code.push('.');
    }

    #[inline]
    fn print_comma(&mut self) {
        self.code.push(',');
    }

    #[inline]
    fn print_colon(&mut self) {
        self.code.push(':');
    }

    #[inline]
    fn print_semi(&mut self) {
        self.code.push(';');
    }

    #[inline]
    fn print_question(&mut self) {
        self.code.push('?');
    }

    #[inline]
    fn print_wrapped(&mut self, open: char, close: char, f: impl FnOnce(&mut Self)) {
        self.print_char(open);
        f(self);
        self.print_char(close);
    }

    fn print_scope(&mut self, open: char, close: char, f: impl FnOnce(&mut Self)) {
        self.print_wrapped(open, close, |c| {
            c.print_newline();
            c.indent();
            f(c);
            c.dedent();
            c.print_indent();
        });
    }

    fn print_argument_list<'a, I: IntoIterator<Item = &'a IrExpression<'a>>>(&mut self, iter: I) {
        self.print_wrapped('(', ')', |c| {
            for (index, expr) in iter.into_iter().enumerate() {
                if index != 0 {
                    c.print_comma();
                    c.print_space();
                }
                c.print_expression(expr);
            }
        });
    }

    fn print_block<'a, I: IntoIterator<Item = &'a IrStatement<'a>>>(&mut self, iter: I) {
        self.print_scope('{', '}', |c| {
            for elem in iter {
                c.print_statement(elem);
            }
        });
    }
}
