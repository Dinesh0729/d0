use crate::token::{Token, TokenKind};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LexError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} at {}:{}", self.message, self.line, self.column)
    }
}

impl std::error::Error for LexError {}

pub fn lex(source: &str) -> Result<Vec<Token>, LexError> {
    Lexer::new(source).scan_tokens()
}

struct Lexer {
    chars: Vec<char>,
    start: usize,
    current: usize,
    line: usize,
    column: usize,
    start_line: usize,
    start_column: usize,
    tokens: Vec<Token>,
}

impl Lexer {
    fn new(source: &str) -> Self {
        Self {
            chars: source.chars().collect(),
            start: 0,
            current: 0,
            line: 1,
            column: 1,
            start_line: 1,
            start_column: 1,
            tokens: Vec::new(),
        }
    }

    fn scan_tokens(mut self) -> Result<Vec<Token>, LexError> {
        while !self.is_at_end() {
            self.start = self.current;
            self.start_line = self.line;
            self.start_column = self.column;
            self.scan_token()?;
        }

        self.tokens.push(Token::new(
            TokenKind::Eof,
            String::new(),
            self.line,
            self.column,
        ));
        Ok(self.tokens)
    }

    fn scan_token(&mut self) -> Result<(), LexError> {
        let ch = self.advance();

        match ch {
            ' ' | '\r' | '\t' => {}
            '\n' => {}
            '(' => self.add(TokenKind::LeftParen),
            ')' => self.add(TokenKind::RightParen),
            '{' => self.add(TokenKind::LeftBrace),
            '}' => self.add(TokenKind::RightBrace),
            '[' => self.add(TokenKind::LeftBracket),
            ']' => self.add(TokenKind::RightBracket),
            ',' => self.add(TokenKind::Comma),
            ':' => self.add(TokenKind::Colon),
            ';' => self.add(TokenKind::Semicolon),
            '+' => self.add(TokenKind::Plus),
            '*' => self.add(TokenKind::Star),
            '%' => self.add(TokenKind::Percent),
            '-' => {
                if self.match_char('>') {
                    self.add(TokenKind::Arrow);
                } else {
                    self.add(TokenKind::Minus);
                }
            }
            '/' => {
                if self.match_char('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else if self.match_char('*') {
                    self.block_comment()?;
                } else {
                    self.add(TokenKind::Slash);
                }
            }
            '=' => {
                let kind = if self.match_char('=') {
                    TokenKind::EqualEqual
                } else {
                    TokenKind::Equal
                };
                self.add(kind);
            }
            '!' => {
                let kind = if self.match_char('=') {
                    TokenKind::BangEqual
                } else {
                    TokenKind::Bang
                };
                self.add(kind);
            }
            '<' => {
                let kind = if self.match_char('=') {
                    TokenKind::LessEqual
                } else {
                    TokenKind::Less
                };
                self.add(kind);
            }
            '>' => {
                let kind = if self.match_char('=') {
                    TokenKind::GreaterEqual
                } else {
                    TokenKind::Greater
                };
                self.add(kind);
            }
            '&' => {
                if self.match_char('&') {
                    self.add(TokenKind::AndAnd);
                } else {
                    return Err(self.error("Expected '&' after '&'"));
                }
            }
            '|' => {
                if self.match_char('|') {
                    self.add(TokenKind::OrOr);
                } else {
                    return Err(self.error("Expected '|' after '|'"));
                }
            }
            '.' => {
                if self.match_char('.') {
                    if self.match_char('=') {
                        self.add(TokenKind::DotDotEqual);
                    } else {
                        self.add(TokenKind::DotDot);
                    }
                } else {
                    self.add(TokenKind::Dot);
                }
            }
            '"' => self.string_literal()?,
            '\'' => self.char_literal()?,
            c if c.is_ascii_digit() => self.number(),
            c if is_identifier_start(c) => self.identifier(),
            _ => return Err(self.error(&format!("Unexpected character '{}'", ch))),
        }

        Ok(())
    }

    fn block_comment(&mut self) -> Result<(), LexError> {
        while !self.is_at_end() {
            if self.peek() == '*' && self.peek_next() == '/' {
                self.advance();
                self.advance();
                return Ok(());
            }
            self.advance();
        }
        Err(self.error("Unterminated block comment"))
    }

    fn string_literal(&mut self) -> Result<(), LexError> {
        while !self.is_at_end() && self.peek() != '"' {
            if self.peek() == '\n' {
                return Err(self.error("String literal cannot contain an unescaped newline"));
            }
            if self.peek() == '\\' {
                self.advance();
                if self.is_at_end() {
                    return Err(self.error("Unterminated escape sequence"));
                }
            }
            self.advance();
        }

        if self.is_at_end() {
            return Err(self.error("Unterminated string literal"));
        }

        self.advance(); // closing quote
        self.add(TokenKind::String);
        Ok(())
    }

    fn char_literal(&mut self) -> Result<(), LexError> {
        if self.is_at_end() || self.peek() == '\n' {
            return Err(self.error("Unterminated character literal"));
        }

        if self.peek() == '\\' {
            self.advance();
            if self.is_at_end() {
                return Err(self.error("Unterminated character escape"));
            }
            self.advance();
        } else {
            self.advance();
        }

        if self.peek() != '\'' {
            return Err(self.error("Character literal must contain exactly one character"));
        }

        self.advance(); // closing quote
        self.add(TokenKind::Char);
        Ok(())
    }

    fn number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        let is_float = self.peek() == '.'
            && self.peek_next().is_ascii_digit()
            && self.peek_next() != '.';

        if is_float {
            self.advance(); // decimal point
            while self.peek().is_ascii_digit() {
                self.advance();
            }
            self.add(TokenKind::Float);
        } else {
            self.add(TokenKind::Integer);
        }
    }

    fn identifier(&mut self) {
        while is_identifier_continue(self.peek()) {
            self.advance();
        }

        let text: String = self.chars[self.start..self.current].iter().collect();
        let kind = match text.as_str() {
            "fn" => TokenKind::Fn,
            "module" => TokenKind::Module,
            "let" => TokenKind::Let,
            "mut" => TokenKind::Mut,
            "if" => TokenKind::If,
            "else" => TokenKind::Else,
            "while" => TokenKind::While,
            "for" => TokenKind::For,
            "in" => TokenKind::In,
            "return" => TokenKind::Return,
            "true" => TokenKind::True,
            "false" => TokenKind::False,
            "struct" => TokenKind::Struct,
            "trait" => TokenKind::Trait,
            "import" => TokenKind::Import,
            "pub" => TokenKind::Pub,
            _ => TokenKind::Identifier,
        };

        self.add(kind);
    }

    fn add(&mut self, kind: TokenKind) {
        let lexeme = self.chars[self.start..self.current].iter().collect();
        self.tokens.push(Token::new(
            kind,
            lexeme,
            self.start_line,
            self.start_column,
        ));
    }

    fn advance(&mut self) -> char {
        let ch = self.chars[self.current];
        self.current += 1;
        if ch == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
        ch
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.peek() != expected {
            return false;
        }
        self.advance();
        true
    }

    fn peek(&self) -> char {
        self.chars.get(self.current).copied().unwrap_or('\0')
    }

    fn peek_next(&self) -> char {
        self.chars.get(self.current + 1).copied().unwrap_or('\0')
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.chars.len()
    }

    fn error(&self, message: &str) -> LexError {
        LexError {
            message: message.to_string(),
            line: self.start_line,
            column: self.start_column,
        }
    }
}

fn is_identifier_start(ch: char) -> bool {
    ch == '_' || ch.is_ascii_alphabetic()
}

fn is_identifier_continue(ch: char) -> bool {
    ch == '_' || ch.is_ascii_alphanumeric()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scans_keywords_and_identifiers() {
        let tokens = lex("fn main() -> Void {}").unwrap();
        assert_eq!(tokens[0].kind, TokenKind::Fn);
        assert_eq!(tokens[1].kind, TokenKind::Identifier);
        assert_eq!(tokens[1].lexeme, "main");
        assert_eq!(tokens[5].lexeme, "Void");
        assert_eq!(tokens.last().unwrap().kind, TokenKind::Eof);
    }

    #[test]
    fn scans_numbers_and_ranges() {
        let tokens = lex("10 2.5 1..=5").unwrap();
        assert_eq!(tokens[0].kind, TokenKind::Integer);
        assert_eq!(tokens[1].kind, TokenKind::Float);
        assert_eq!(tokens[2].kind, TokenKind::Integer);
        assert_eq!(tokens[3].kind, TokenKind::DotDotEqual);
        assert_eq!(tokens[4].kind, TokenKind::Integer);
    }

    #[test]
    fn skips_comments() {
        let tokens = lex("// line comment\nfn /* block */ main()").unwrap();
        assert_eq!(tokens[0].kind, TokenKind::Fn);
        assert_eq!(tokens[1].lexeme, "main");
    }

    #[test]
    fn reports_unterminated_string() {
        let error = lex("\"unfinished").unwrap_err();
        assert!(error.message.contains("Unterminated string"));
    }

    #[test]
    fn tracks_line_and_column() {
        let tokens = lex("fn main() {\n  let x = 1;\n}").unwrap();
        let let_token = tokens.iter().find(|t| t.kind == TokenKind::Let).unwrap();
        assert_eq!(let_token.line, 2);
        assert_eq!(let_token.column, 3);
    }
}
