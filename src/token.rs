#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    // Keywords
    Fn,
    Module,
    Let,
    Mut,
    If,
    Else,
    While,
    For,
    In,
    Return,
    True,
    False,
    Struct,
    Trait,
    Import,
    Pub,

    // Identifiers and literals
    Identifier,
    Integer,
    Float,
    String,
    Char,

    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Equal,
    EqualEqual,
    Bang,
    BangEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    AndAnd,
    OrOr,
    Arrow,
    Dot,
    DotDot,
    DotDotEqual,

    // Punctuation
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Colon,
    Semicolon,

    Eof,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub line: usize,
    pub column: usize,
}

impl Token {
    pub fn new(kind: TokenKind, lexeme: String, line: usize, column: usize) -> Self {
        Self { kind, lexeme, line, column }
    }
}
