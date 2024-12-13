use crate::ast::*;

#[derive(Default)]
pub struct CodegenOptions {
    pub minify: bool,
}

#[derive(Default)]
pub struct Codegen {
    code: String,
    is_complex: bool,
    options: CodegenOptions,
}

impl Codegen {
    pub fn build(mut self, program: &Program) -> String {
        self.code.reserve(program.source.len());
        self.is_complex = program.is_complex;
        program.gen(&mut self);
        self.code
    }

    pub fn with_options(mut self, options: CodegenOptions) -> Self {
        self.options = options;
        self
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
    fn print_space(&mut self) {
        if self.options.minify {
            return;
        }
        self.code.push(' ');
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
    fn print_list<T: Gen>(&mut self, items: &[T]) {
        for (index, item) in items.iter().enumerate() {
            if index != 0 {
                self.print_comma();
                self.print_space();
            }
            item.gen(self);
        }
    }
}

/// Generate code for an AST.
pub trait Gen {
    fn gen(&self, c: &mut Codegen);
}

impl Gen for Program<'_> {
    fn gen(&self, c: &mut Codegen) {
        for expr in &self.body {
            expr.gen(c);
            if c.is_complex {
                c.print_semi();
            }
        }
    }
}

impl Gen for Expression<'_> {
    fn gen(&self, c: &mut Codegen) {
        match self {
            Self::BooleanLiteral(expr) => expr.gen(c),
            Self::NumericLiteral(expr) => expr.gen(c),
            Self::StringLiteral(expr) => expr.gen(c),
            Self::Variable(expr) => expr.gen(c),
            Self::Parenthesized(expr) => expr.gen(c),
            Self::Block(expr) => expr.gen(c),
            Self::Binary(expr) => expr.gen(c),
            Self::Unary(expr) => expr.gen(c),
            Self::Ternary(expr) => expr.gen(c),
            Self::Conditional(expr) => expr.gen(c),
            Self::Assignment(expr) => expr.gen(c),
            Self::Resource(expr) => expr.gen(c),
            Self::ArrayAccess(expr) => expr.gen(c),
            Self::ArrowAccess(expr) => expr.gen(c),
            Self::Call(expr) => expr.gen(c),
            Self::Loop(expr) => expr.gen(c),
            Self::ForEach(expr) => expr.gen(c),
            Self::Break(expr) => expr.gen(c),
            Self::Continue(expr) => expr.gen(c),
            Self::This(expr) => expr.gen(c),
            Self::Return(expr) => expr.gen(c),
        }
    }
}

impl Gen for IdentifierReference<'_> {
    fn gen(&self, c: &mut Codegen) {
        c.print_str(self.name);
    }
}

impl Gen for BooleanLiteral {
    fn gen(&self, c: &mut Codegen) {
        c.print_str(self.as_str());
    }
}

impl Gen for NumericLiteral<'_> {
    fn gen(&self, c: &mut Codegen) {
        c.print_str(self.raw);
    }
}

impl Gen for StringLiteral<'_> {
    fn gen(&self, c: &mut Codegen) {
        c.print_char('\'');
        c.print_str(self.value);
        c.print_char('\'');
    }
}

impl Gen for VariableExpression<'_> {
    fn gen(&self, c: &mut Codegen) {
        self.lifetime.gen(c);
        c.print_dot();
        self.member.gen(c);
    }
}

impl Gen for VariableLifetime {
    fn gen(&self, c: &mut Codegen) {
        let lifetime = self.as_str();
        if c.options.minify {
            c.print_char(lifetime.chars().nth(0).unwrap());
        } else {
            c.print_str(lifetime);
        }
    }
}

impl Gen for VariableMember<'_> {
    fn gen(&self, c: &mut Codegen) {
        match self {
            Self::Object {
                object, property, ..
            } => {
                object.gen(c);
                c.print_dot();
                property.gen(c);
            }
            Self::Property { property, .. } => {
                property.gen(c);
            }
        }
    }
}

impl Gen for ParenthesizedExpression<'_> {
    fn gen(&self, c: &mut Codegen) {
        c.print_char('(');
        match self {
            Self::Single { expression, .. } => expression.gen(c),
            Self::Complex { expressions, .. } => {
                for expr in expressions {
                    expr.gen(c);
                    c.print_semi();
                }
            }
        }
        c.print_char(')');
    }
}

impl Gen for BlockExpression<'_> {
    fn gen(&self, c: &mut Codegen) {
        c.print_char('{');
        for expr in &self.expressions {
            expr.gen(c);
            c.print_semi();
        }
        c.print_char('}');
    }
}

impl Gen for BinaryExpression<'_> {
    fn gen(&self, c: &mut Codegen) {
        self.left.gen(c);
        c.print_space();
        self.operator.gen(c);
        c.print_space();
        self.right.gen(c);
    }
}

impl Gen for BinaryOperator {
    fn gen(&self, c: &mut Codegen) {
        c.print_str(self.as_str());
    }
}

impl Gen for UnaryExpression<'_> {
    fn gen(&self, c: &mut Codegen) {
        self.operator.gen(c);
        self.argument.gen(c);
    }
}

impl Gen for UnaryOperator {
    fn gen(&self, c: &mut Codegen) {
        c.print_str(self.as_str());
    }
}

impl Gen for TernaryExpression<'_> {
    fn gen(&self, c: &mut Codegen) {
        self.test.gen(c);
        c.print_space();
        c.print_char('?');
        c.print_space();
        self.consequent.gen(c);
        c.print_space();
        c.print_colon();
        c.print_space();
        self.alternate.gen(c);
    }
}

impl Gen for ConditionalExpression<'_> {
    fn gen(&self, c: &mut Codegen) {
        self.test.gen(c);
        c.print_space();
        c.print_char('?');
        c.print_space();
        self.consequent.gen(c);
    }
}

impl Gen for AssignmentExpression<'_> {
    fn gen(&self, c: &mut Codegen) {
        self.left.gen(c);
        c.print_space();
        c.print_char('=');
        c.print_space();
        self.right.gen(c);
    }
}

impl Gen for ResourceExpression<'_> {
    fn gen(&self, c: &mut Codegen) {
        c.print_str(self.section.as_str());
        c.print_dot();
        self.name.gen(c);
    }
}

impl Gen for ArrayAccessExpression<'_> {
    fn gen(&self, c: &mut Codegen) {
        c.print_str("array.");
        self.name.gen(c);
        c.print_char('[');
        self.index.gen(c);
        c.print_char(']');
    }
}

impl Gen for ArrowAccessExpression<'_> {
    fn gen(&self, c: &mut Codegen) {
        self.left.gen(c);
        c.print_str("->");
        self.right.gen(c);
    }
}

impl Gen for CallExpression<'_> {
    fn gen(&self, c: &mut Codegen) {
        self.kind.gen(c);
        c.print_dot();
        self.callee.gen(c);
        if let Some(args) = &self.arguments {
            c.print_char('(');
            c.print_list(args);
            c.print_char(')');
        }
    }
}

impl Gen for CallKind {
    fn gen(&self, c: &mut Codegen) {
        if let Self::Query = self {
            let kind = self.as_str();
            if c.options.minify {
                c.print_char(kind.chars().nth(0).unwrap());
            } else {
                c.print_str(kind);
            }
        } else {
            c.print_str(self.as_str());
        }
    }
}

impl Gen for LoopExpression<'_> {
    fn gen(&self, c: &mut Codegen) {
        c.print_str("loop");
        c.print_char('(');
        self.count.gen(c);
        c.print_comma();
        c.print_space();
        self.expression.gen(c);
        c.print_char(')');
    }
}

impl Gen for ForEachExpression<'_> {
    fn gen(&self, c: &mut Codegen) {
        c.print_str("for_each");
        c.print_char('(');
        self.variable.gen(c);
        c.print_comma();
        c.print_space();
        self.array.gen(c);
        c.print_comma();
        c.print_space();
        self.expression.gen(c);
        c.print_char(')');
    }
}

impl Gen for Break {
    fn gen(&self, c: &mut Codegen) {
        c.print_str("break");
    }
}

impl Gen for Continue {
    fn gen(&self, c: &mut Codegen) {
        c.print_str("continue");
    }
}

impl Gen for Return<'_> {
    fn gen(&self, c: &mut Codegen) {
        c.print_str("return ");
        self.argument.gen(c);
    }
}

impl Gen for This {
    fn gen(&self, c: &mut Codegen) {
        c.print_str("this");
    }
}
