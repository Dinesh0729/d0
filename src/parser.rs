use crate::ast::*;
use crate::token::{Token, TokenKind};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} at {}:{}", self.message, self.line, self.column)
    }
}

impl std::error::Error for ParseError {}

pub fn parse(tokens: &[Token]) -> Result<Program, ParseError> {
    Parser::new(tokens).parse_program()
}

pub struct Parser<'a> {
    tokens: &'a [Token],
    current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse_program(mut self) -> Result<Program, ParseError> {
        let mut declarations = Vec::new();

        while !self.is_at_end() {
            declarations.push(self.declaration()?);
        }

        Ok(Program { declarations })
    }

    fn declaration(&mut self) -> Result<Declaration, ParseError> {
        if self.matches(&TokenKind::Module) {
            return self.module_declaration();
        }
        if self.matches(&TokenKind::Fn) {
            return self.function_declaration().map(Declaration::Function);
        }
        Err(self.error("Expected a module or function declaration"))
    }

    fn module_declaration(&mut self) -> Result<Declaration, ParseError> {
        let name = self.consume_identifier("Expected module name")?;
        self.consume(&TokenKind::LeftBrace, "Expected '{' after module name")?;

        let mut declarations = Vec::new();
        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
            declarations.push(self.declaration()?);
        }

        self.consume(&TokenKind::RightBrace, "Expected '}' after module body")?;
        Ok(Declaration::Module(ModuleDecl { name, declarations }))
    }

    fn function_declaration(&mut self) -> Result<FunctionDecl, ParseError> {
        let name = self.consume_identifier("Expected function name")?;
        self.consume(&TokenKind::LeftParen, "Expected '(' after function name")?;

        let mut parameters = Vec::new();
        if !self.check(&TokenKind::RightParen) {
            loop {
                let param_name = self.consume_identifier("Expected parameter name")?;
                self.consume(&TokenKind::Colon, "Expected ':' after parameter name")?;
                let type_name = self.consume_identifier("Expected parameter type")?;
                parameters.push(Parameter {
                    name: param_name,
                    type_name,
                });

                if !self.matches(&TokenKind::Comma) {
                    break;
                }
            }
        }

        self.consume(&TokenKind::RightParen, "Expected ')' after parameters")?;
        self.consume(&TokenKind::Arrow, "Expected '->' before return type")?;
        let return_type = self.consume_identifier("Expected return type")?;
        let body = self.block()?;

        Ok(FunctionDecl {
            name,
            parameters,
            return_type,
            body,
        })
    }

    fn block(&mut self) -> Result<Block, ParseError> {
        self.consume(&TokenKind::LeftBrace, "Expected '{' to start block")?;
        let mut statements = Vec::new();

        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
            statements.push(self.statement()?);
        }

        self.consume(&TokenKind::RightBrace, "Expected '}' after block")?;
        Ok(Block { statements })
    }

    fn statement(&mut self) -> Result<Statement, ParseError> {
        if self.matches(&TokenKind::Let) {
            return self.variable_declaration(false);
        }
        if self.matches(&TokenKind::Mut) {
            return self.variable_declaration(true);
        }
        if self.matches(&TokenKind::If) {
            return self.if_statement();
        }
        if self.matches(&TokenKind::While) {
            return self.while_statement();
        }
        if self.matches(&TokenKind::For) {
            return self.for_statement();
        }
        if self.matches(&TokenKind::Return) {
            return self.return_statement();
        }
        if self.check(&TokenKind::LeftBrace) {
            return self.block().map(Statement::Block);
        }

        if self.check(&TokenKind::Identifier) && self.check_next(&TokenKind::Equal) {
            let name = self.advance().lexeme.clone();
            self.advance(); // '='
            let value = self.expression()?;
            self.consume(&TokenKind::Semicolon, "Expected ';' after assignment")?;
            return Ok(Statement::Assignment { name, value });
        }

        let expression = self.expression()?;
        self.consume(&TokenKind::Semicolon, "Expected ';' after expression")?;
        Ok(Statement::Expression(expression))
    }

    fn variable_declaration(&mut self, mutable: bool) -> Result<Statement, ParseError> {
        let name = self.consume_identifier("Expected variable name")?;
        let type_name = if self.matches(&TokenKind::Colon) {
            Some(self.consume_identifier("Expected type name after ':'")?)
        } else {
            None
        };
        self.consume(&TokenKind::Equal, "Expected '=' in variable declaration")?;
        let initializer = self.expression()?;
        self.consume(&TokenKind::Semicolon, "Expected ';' after variable declaration")?;

        Ok(Statement::VariableDeclaration {
            mutable,
            name,
            type_name,
            initializer,
        })
    }

    fn if_statement(&mut self) -> Result<Statement, ParseError> {
        let condition = self.expression()?;
        let then_branch = self.block()?;

        let else_branch = if self.matches(&TokenKind::Else) {
            if self.matches(&TokenKind::If) {
                Some(Box::new(self.if_statement()?))
            } else {
                Some(Box::new(Statement::Block(self.block()?)))
            }
        } else {
            None
        };

        Ok(Statement::If {
            condition,
            then_branch,
            else_branch,
        })
    }

    fn while_statement(&mut self) -> Result<Statement, ParseError> {
        let condition = self.expression()?;
        let body = self.block()?;
        Ok(Statement::While { condition, body })
    }

    fn for_statement(&mut self) -> Result<Statement, ParseError> {
        let variable = self.consume_identifier("Expected loop variable after 'for'")?;
        self.consume(&TokenKind::In, "Expected 'in' after loop variable")?;
        let iterable = self.expression()?;
        let body = self.block()?;
        Ok(Statement::For {
            variable,
            iterable,
            body,
        })
    }

    fn return_statement(&mut self) -> Result<Statement, ParseError> {
        if self.matches(&TokenKind::Semicolon) {
            return Ok(Statement::Return(None));
        }
        let value = self.expression()?;
        self.consume(&TokenKind::Semicolon, "Expected ';' after return value")?;
        Ok(Statement::Return(Some(value)))
    }

    fn expression(&mut self) -> Result<Expression, ParseError> {
        self.parse_precedence(1)
    }

    // Pratt parser. Precedence: ||, &&, equality, comparison, range,
    // addition, multiplication. Unary and call binding are handled separately.
    fn parse_precedence(&mut self, min_precedence: u8) -> Result<Expression, ParseError> {
        let mut left = self.unary()?;

        loop {
            let Some((precedence, operator, is_range, inclusive)) = self.current_operator() else {
                break;
            };
            if precedence < min_precedence {
                break;
            }

            self.advance();
            let right = self.parse_precedence(precedence + 1)?;

            left = if is_range {
                Expression::Range {
                    start: Box::new(left),
                    inclusive,
                    end: Box::new(right),
                }
            } else {
                Expression::Binary {
                    left: Box::new(left),
                    operator,
                    right: Box::new(right),
                }
            };
        }

        Ok(left)
    }

    fn current_operator(&self) -> Option<(u8, String, bool, bool)> {
        let (precedence, text, range, inclusive) = match self.peek().kind {
            TokenKind::OrOr => (1, "||", false, false),
            TokenKind::AndAnd => (2, "&&", false, false),
            TokenKind::EqualEqual => (3, "==", false, false),
            TokenKind::BangEqual => (3, "!=", false, false),
            TokenKind::Less => (4, "<", false, false),
            TokenKind::LessEqual => (4, "<=", false, false),
            TokenKind::Greater => (4, ">", false, false),
            TokenKind::GreaterEqual => (4, ">=", false, false),
            TokenKind::DotDot => (5, "..", true, false),
            TokenKind::DotDotEqual => (5, "..=", true, true),
            TokenKind::Plus => (6, "+", false, false),
            TokenKind::Minus => (6, "-", false, false),
            TokenKind::Star => (7, "*", false, false),
            TokenKind::Slash => (7, "/", false, false),
            TokenKind::Percent => (7, "%", false, false),
            _ => return None,
        };
        Some((precedence, text.to_string(), range, inclusive))
    }

    fn unary(&mut self) -> Result<Expression, ParseError> {
        if self.matches(&TokenKind::Bang) || self.matches(&TokenKind::Minus) {
            let operator = self.previous().lexeme.clone();
            let right = self.unary()?;
            return Ok(Expression::Unary {
                operator,
                right: Box::new(right),
            });
        }
        self.postfix()
    }

    fn postfix(&mut self) -> Result<Expression, ParseError> {
        let mut expression = self.primary()?;

        while self.matches(&TokenKind::LeftParen) {
            let mut arguments = Vec::new();
            if !self.check(&TokenKind::RightParen) {
                loop {
                    arguments.push(self.expression()?);
                    if !self.matches(&TokenKind::Comma) {
                        break;
                    }
                }
            }
            self.consume(&TokenKind::RightParen, "Expected ')' after arguments")?;
            expression = Expression::Call {
                callee: Box::new(expression),
                arguments,
            };
        }

        Ok(expression)
    }

    fn primary(&mut self) -> Result<Expression, ParseError> {
        let token = self.advance().clone();
        match token.kind {
            TokenKind::Integer => Ok(Expression::Integer(token.lexeme)),
            TokenKind::Float => Ok(Expression::Float(token.lexeme)),
            TokenKind::String => Ok(Expression::String(token.lexeme)),
            TokenKind::Char => Ok(Expression::Char(token.lexeme)),
            TokenKind::True => Ok(Expression::Bool(true)),
            TokenKind::False => Ok(Expression::Bool(false)),
            TokenKind::Identifier => Ok(Expression::Identifier(token.lexeme)),
            TokenKind::LeftParen => {
                let expression = self.expression()?;
                self.consume(&TokenKind::RightParen, "Expected ')' after expression")?;
                Ok(Expression::Grouping(Box::new(expression)))
            }
            _ => Err(ParseError {
                message: format!("Expected expression, found '{}'", token.lexeme),
                line: token.line,
                column: token.column,
            }),
        }
    }

    fn consume_identifier(&mut self, message: &str) -> Result<String, ParseError> {
        if self.check(&TokenKind::Identifier) {
            return Ok(self.advance().lexeme.clone());
        }
        Err(self.error(message))
    }

    fn consume(&mut self, kind: &TokenKind, message: &str) -> Result<Token, ParseError> {
        if self.check(kind) {
            return Ok(self.advance().clone());
        }
        Err(self.error(message))
    }

    fn matches(&mut self, kind: &TokenKind) -> bool {
        if self.check(kind) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn check(&self, kind: &TokenKind) -> bool {
        self.peek().kind == *kind
    }

    fn check_next(&self, kind: &TokenKind) -> bool {
        self.tokens
            .get(self.current + 1)
            .map(|token| token.kind == *kind)
            .unwrap_or(false)
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().kind == TokenKind::Eof
    }

    fn peek(&self) -> &Token {
        self.tokens
            .get(self.current)
            .or_else(|| self.tokens.last())
            .expect("Parser requires an EOF token")
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current.saturating_sub(1)]
    }

    fn error(&self, message: &str) -> ParseError {
        let token = self.peek();
        ParseError {
            message: message.to_string(),
            line: token.line,
            column: token.column,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::lex;

    fn parse_source(source: &str) -> Result<Program, ParseError> {
        let tokens = lex(source).expect("lexer should accept test source");
        parse(&tokens)
    }

    #[test]
    fn parses_simple_main_function() {
        let program = parse_source(
            r#"fn main() -> Void { print("Hello"); }"#,
        ).unwrap();

        assert_eq!(program.declarations.len(), 1);
        match &program.declarations[0] {
            Declaration::Function(function) => {
                assert_eq!(function.name, "main");
                assert_eq!(function.return_type, "Void");
                assert_eq!(function.body.statements.len(), 1);
            }
            _ => panic!("expected function"),
        }
    }

    #[test]
    fn parses_module_and_function() {
        let program = parse_source(
            "module Demo { fn main() -> Void { return; } }",
        ).unwrap();

        match &program.declarations[0] {
            Declaration::Module(module) => {
                assert_eq!(module.name, "Demo");
                assert_eq!(module.declarations.len(), 1);
            }
            _ => panic!("expected module"),
        }
    }

    #[test]
    fn parses_variable_declaration() {
        let program = parse_source(
            r#"fn main() -> Void { let name: String = "Dinesh"; }"#,
        ).unwrap();

        let Declaration::Function(function) = &program.declarations[0] else {
            panic!("expected function");
        };
        assert!(matches!(
            &function.body.statements[0],
            Statement::VariableDeclaration {
                mutable: false,
                name,
                type_name: Some(type_name),
                ..
            } if name == "name" && type_name == "String"
        ));
    }

    #[test]
    fn parses_mutable_variable_and_assignment() {
        let program = parse_source(
            "fn main() -> Void { mut count: Int = 1; count = count + 1; }",
        ).unwrap();

        let Declaration::Function(function) = &program.declarations[0] else {
            panic!("expected function");
        };
        assert_eq!(function.body.statements.len(), 2);
        assert!(matches!(
            &function.body.statements[0],
            Statement::VariableDeclaration { mutable: true, .. }
        ));
        assert!(matches!(
            &function.body.statements[1],
            Statement::Assignment { .. }
        ));
    }

    #[test]
    fn parses_if_else() {
        let program = parse_source(
            r#"fn main() -> Void { if score >= 70 { print("pass"); } else { print("fail"); } }"#,
        ).unwrap();

        let Declaration::Function(function) = &program.declarations[0] else {
            panic!("expected function");
        };
        assert!(matches!(&function.body.statements[0], Statement::If { .. }));
    }

    #[test]
    fn parses_for_range() {
        let program = parse_source(
            "fn main() -> Void { for i in 1..=5 { print(i); } }",
        ).unwrap();

        let Declaration::Function(function) = &program.declarations[0] else {
            panic!("expected function");
        };
        match &function.body.statements[0] {
            Statement::For { iterable, .. } => {
                assert!(matches!(iterable, Expression::Range { inclusive: true, .. }));
            }
            _ => panic!("expected for statement"),
        }
    }

    #[test]
    fn respects_operator_precedence() {
        let program = parse_source(
            "fn main() -> Void { let x = 1 + 2 * 3; }",
        ).unwrap();

        let Declaration::Function(function) = &program.declarations[0] else {
            panic!("expected function");
        };
        let Statement::VariableDeclaration { initializer, .. } = &function.body.statements[0] else {
            panic!("expected variable declaration");
        };
        let Expression::Binary { operator, right, .. } = initializer else {
            panic!("expected addition at root");
        };
        assert_eq!(operator, "+");
        assert!(matches!(right.as_ref(), Expression::Binary { operator, .. } if operator == "*"));
    }

    #[test]
    fn reports_missing_semicolon() {
        let error = parse_source(
            r#"fn main() -> Void { print("missing semicolon") }"#,
        ).unwrap_err();

        assert!(error.message.contains("Expected ';'"));
    }

    #[test]
    fn parses_function_parameters_and_return() {
        let program = parse_source(
            "fn add(a: Int, b: Int) -> Int { return a + b; }",
        ).unwrap();

        let Declaration::Function(function) = &program.declarations[0] else {
            panic!("expected function");
        };
        assert_eq!(function.parameters.len(), 2);
        assert_eq!(function.return_type, "Int");
    }
}
