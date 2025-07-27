use logos::Logos;

use crate::span::Span;

#[derive(Debug, Default, Clone, Copy)]
pub struct Token {
    pub kind: Kind,
    pub start: u32,
    pub end: u32,
}

impl Token {
    pub const fn span(&self) -> Span {
        Span::new(self.start, self.end)
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Default, Logos)]
#[logos(skip "[ \t\n\r]+")]
pub enum Kind {
    #[default]
    Eof,

    #[regex("[a-zA-Z_]+[a-zA-Z0-9_]*")]
    Identifier,

    #[regex("'[^']*'")]
    String,

    #[regex("'[^']*")]
    UnterminatedString,

    // NOTE: The optional 'f' suffix must be removed.
    #[regex(r"[0-9]*\.?[0-9]+([eE][+-]?[0-9]+)?f?")]
    Number,

    #[token("(")]
    LeftParen,

    #[token(")")]
    RightParen,

    #[token("{")]
    LeftBrace,

    #[token("}")]
    RightBrace,

    #[token("[")]
    LeftBracket,

    #[token("]")]
    RightBracket,

    #[token("=")]
    Assign,

    #[token("!")]
    Not,

    #[token("==")]
    Eq,

    #[token("!=")]
    NotEq,

    #[token("<")]
    Lt,

    #[token(">")]
    Gt,

    #[token("<=")]
    LtEq,

    #[token(">=")]
    GtEq,

    #[token("||")]
    Or,

    #[token("&&")]
    And,

    #[token("->")]
    Arrow,

    #[token(".")]
    Dot,

    #[token("?")]
    Conditional,

    #[token("??")]
    NullCoal,

    #[token(":")]
    Colon,

    #[token(";")]
    Semi,

    #[token(",")]
    Comma,

    #[token("-")]
    Minus,

    #[token("+")]
    Plus,

    #[token("*")]
    Star,

    #[token("/")]
    Slash,

    #[token("temp")]
    #[token("t", priority = 3)]
    Temporary,

    #[token("variable")]
    #[token("v", priority = 3)]
    Variable,

    #[token("context")]
    #[token("c", priority = 3)]
    Context,

    #[regex(r"[Mm]ath")]
    Math,

    #[regex(r"[Qq]uery")]
    #[token("q", priority = 3)]
    Query,

    #[regex(r"[Gg]eometry")]
    Geometry,

    #[regex(r"[Mm]aterial")]
    Material,

    #[regex(r"[Tt]exture")]
    Texture,

    #[regex(r"[Aa]rray")]
    Array,

    #[token("true")]
    True,

    #[token("false")]
    False,

    #[token("this")]
    This,

    #[token("break")]
    Break,

    #[token("continue")]
    Continue,

    #[token("for_each")]
    ForEach,

    #[token("loop")]
    Loop,

    #[token("return")]
    Return,
}

impl Kind {
    pub fn is_binary_operator(self) -> bool {
        matches!(
            self,
            Kind::Eq
                | Kind::NotEq
                | Kind::Lt
                | Kind::Gt
                | Kind::LtEq
                | Kind::GtEq
                | Kind::Or
                | Kind::And
                | Kind::NullCoal
                | Kind::Minus
                | Kind::Plus
                | Kind::Star
                | Kind::Slash
        )
    }

    /// <https://bedrock.dev/docs/stable/Molang#Operator%20Precedence>
    pub fn binding_power(self) -> Option<(u8, u8)> {
        Some(match self {
            Self::Not => (16, 17),
            Self::Star | Self::Slash => (14, 15),
            Self::Plus | Self::Minus => (12, 13),
            Self::Lt | Self::Gt | Self::LtEq | Self::GtEq => (10, 11),
            Self::Eq | Self::NotEq => (8, 9),
            Self::And => (6, 7),
            Self::Or => (4, 5),
            Self::Conditional => (3, 4),
            Self::NullCoal => (1, 2),
            _ => return None,
        })
    }

    pub fn to_str(self) -> &'static str {
        match self {
            Kind::Eof => "EOF",
            Kind::Identifier => "Identifier",
            Kind::String => "string",
            Kind::UnterminatedString => "Unterminated String",
            Kind::Number => "number",
            Kind::LeftParen => "(",
            Kind::RightParen => ")",
            Kind::LeftBrace => "{",
            Kind::RightBrace => "}",
            Kind::LeftBracket => "[",
            Kind::RightBracket => "]",
            Kind::Assign => "=",
            Kind::Not => "!",
            Kind::Eq => "==",
            Kind::NotEq => "!=",
            Kind::Lt => "<",
            Kind::Gt => ">",
            Kind::LtEq => "<=",
            Kind::GtEq => ">=",
            Kind::Or => "||",
            Kind::And => "&&",
            Kind::Arrow => "->",
            Kind::Dot => ".",
            Kind::Conditional => "?",
            Kind::NullCoal => "??",
            Kind::Colon => ":",
            Kind::Semi => ";",
            Kind::Comma => ",",
            Kind::Minus => "-",
            Kind::Plus => "+",
            Kind::Star => "*",
            Kind::Slash => "/",
            Kind::Temporary => "temp",
            Kind::Variable => "variable",
            Kind::Context => "context",
            Kind::Math => "math",
            Kind::Query => "query",
            Kind::Geometry => "geometry",
            Kind::Material => "material",
            Kind::Texture => "texture",
            Kind::Array => "array",
            Kind::True => "true",
            Kind::False => "false",
            Kind::This => "this",
            Kind::Break => "break",
            Kind::Continue => "continue",
            Kind::ForEach => "for_each",
            Kind::Loop => "loop",
            Kind::Return => "return",
        }
    }
}

#[cfg(all(test, target_pointer_width = "64"))]
mod size_asserts {
    const _: () = assert!(size_of::<super::Kind>() == 1);
}

#[cfg(test)]
mod tests {
    use logos::Logos;

    use super::*;

    fn assert_lexer(source: &str, expected: &[(Result<Kind, ()>, &str)]) {
        let tokens: Vec<_> =
            Kind::lexer(source).spanned().map(|(token, span)| (token, &source[span])).collect();
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_identifiers() {
        assert_lexer(
            "foo_bar.a23",
            &[
                (Ok(Kind::Identifier), "foo_bar"),
                (Ok(Kind::Dot), "."),
                (Ok(Kind::Identifier), "a23"),
            ],
        );
    }

    #[test]
    fn test_strings() {
        assert_lexer(
            "'abc123-_!' '' ' '",
            &[
                (Ok(Kind::String), "'abc123-_!'"),
                (Ok(Kind::String), "''"),
                (Ok(Kind::String), "' '"),
            ],
        );
        assert_lexer("'unterminated {}", &[(Ok(Kind::UnterminatedString), "'unterminated {}")]);
    }

    #[test]
    fn test_numbers() {
        assert_lexer(
            "0 123 123.456 .456 123.456f 0.1 1.23e10 1.23E+10 1.23e-10f 1e5 1.5f 0.0 1.23f",
            &[
                (Ok(Kind::Number), "0"),
                (Ok(Kind::Number), "123"),
                (Ok(Kind::Number), "123.456"),
                (Ok(Kind::Number), ".456"),
                (Ok(Kind::Number), "123.456f"),
                (Ok(Kind::Number), "0.1"),
                (Ok(Kind::Number), "1.23e10"),
                (Ok(Kind::Number), "1.23E+10"),
                (Ok(Kind::Number), "1.23e-10f"),
                (Ok(Kind::Number), "1e5"),
                (Ok(Kind::Number), "1.5f"),
                (Ok(Kind::Number), "0.0"),
                (Ok(Kind::Number), "1.23f"),
            ],
        );
    }

    #[test]
    fn test_members() {
        assert_lexer(
            "temp t variable v context c Math math Query query q Geometry geometry Texture texture Material material Array array",
            &[
                (Ok(Kind::Temporary), "temp"),
                (Ok(Kind::Temporary), "t"),
                (Ok(Kind::Variable), "variable"),
                (Ok(Kind::Variable), "v"),
                (Ok(Kind::Context), "context"),
                (Ok(Kind::Context), "c"),
                (Ok(Kind::Math), "Math"),
                (Ok(Kind::Math), "math"),
                (Ok(Kind::Query), "Query"),
                (Ok(Kind::Query), "query"),
                (Ok(Kind::Query), "q"),
                (Ok(Kind::Geometry), "Geometry"),
                (Ok(Kind::Geometry), "geometry"),
                (Ok(Kind::Texture), "Texture"),
                (Ok(Kind::Texture), "texture"),
                (Ok(Kind::Material), "Material"),
                (Ok(Kind::Material), "material"),
                (Ok(Kind::Array), "Array"),
                (Ok(Kind::Array), "array"),
            ],
        );
    }

    #[test]
    fn test_keywords() {
        assert_lexer(
            "true false break continue for_each loop return",
            &[
                (Ok(Kind::True), "true"),
                (Ok(Kind::False), "false"),
                (Ok(Kind::Break), "break"),
                (Ok(Kind::Continue), "continue"),
                (Ok(Kind::ForEach), "for_each"),
                (Ok(Kind::Loop), "loop"),
                (Ok(Kind::Return), "return"),
            ],
        );
    }

    #[test]
    fn test_symbols() {
        assert_lexer(
            "() {} [] = ! == != <> <= >= || && -> ? ?? : ; , - + * /",
            &[
                (Ok(Kind::LeftParen), "("),
                (Ok(Kind::RightParen), ")"),
                (Ok(Kind::LeftBrace), "{"),
                (Ok(Kind::RightBrace), "}"),
                (Ok(Kind::LeftBracket), "["),
                (Ok(Kind::RightBracket), "]"),
                (Ok(Kind::Assign), "="),
                (Ok(Kind::Not), "!"),
                (Ok(Kind::Eq), "=="),
                (Ok(Kind::NotEq), "!="),
                (Ok(Kind::Lt), "<"),
                (Ok(Kind::Gt), ">"),
                (Ok(Kind::LtEq), "<="),
                (Ok(Kind::GtEq), ">="),
                (Ok(Kind::Or), "||"),
                (Ok(Kind::And), "&&"),
                (Ok(Kind::Arrow), "->"),
                (Ok(Kind::Conditional), "?"),
                (Ok(Kind::NullCoal), "??"),
                (Ok(Kind::Colon), ":"),
                (Ok(Kind::Semi), ";"),
                (Ok(Kind::Comma), ","),
                (Ok(Kind::Minus), "-"),
                (Ok(Kind::Plus), "+"),
                (Ok(Kind::Star), "*"),
                (Ok(Kind::Slash), "/"),
            ],
        );
    }

    #[test]
    fn test_whitespace() {
        assert_lexer("\t\r\n", &[]);
    }
}
