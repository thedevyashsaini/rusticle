use crate::parser::ast::{Expr, Stmt, Object};
use crate::utils::token::{Token, TokenType};
use std::vec::Vec;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}


impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut statements: Vec<Stmt> = Vec::new();
        while !self.is_at_end() {
            if let Some(stmt) = self.declaration() {
                statements.push(stmt);
            }
        }
        statements
    }

    fn declaration(&mut self) -> Option<Stmt> {
        if self.match_token(&[TokenType::Var]) {
            return self.var_declaration();
        }
        if self.match_token(&[TokenType::Fun]) {
            return self.function("function");
        }
        if self.match_token(&[TokenType::Class]) {
            return self.class_declaration();
        }
        if self.match_token(&[TokenType::Import]) {
            return self.import_statement();
        }
        self.statement()
    }

    fn import_statement(&mut self) -> Option<Stmt> {
        self.consume(TokenType::String, "Expect function name.")?;
        let function_name = self.previous().lexeme.clone();
        self.consume(TokenType::From, "Expect 'from'.")?;
        self.consume(TokenType::String, "Expect package name.")?;
        let package_name = self.previous().lexeme.clone();
        self.consume(TokenType::Semicolon, "Expect ';' after import statement.")?;
        Some(Stmt::Import { function_name, package_name })
    }

    fn var_declaration(&mut self) -> Option<Stmt> {
        let name = self.consume(TokenType::Identifier, "Expect variable name.")?;
        let initializer = if self.match_token(&[TokenType::Equal]) {
            Some(self.expression()?)
        } else {
            None
        };
        self.consume(TokenType::Semicolon, "Expect ';' after variable declaration.")?;
        Some(Stmt::Var { name, initializer })
    }

    fn class_declaration(&mut self) -> Option<Stmt> {
        let name = self.consume(TokenType::Identifier, "Expect class name.")?;
        let superclass = if self.match_token(&[TokenType::Less]) {
            self.consume(TokenType::Identifier, "Expect superclass name.")?;
            Some(Expr::Variable { name: self.previous().clone() })
        } else {
            None
        };
        self.consume(TokenType::LeftBrace, "Expect '{' before class body.")?;
        let mut methods = Vec::new();
        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            methods.push(self.function("method")?);
        }
        self.consume(TokenType::RightBrace, "Expect '}' after class body.")?;
        Some(Stmt::Class { name, superclass, methods })
    }

    fn function(&mut self, kind: &str) -> Option<Stmt> {
        let name = self.consume(TokenType::Identifier, &format!("Expect {} name.", kind))?;
        self.consume(TokenType::LeftParen, &format!("Expect '(' after {} name.", kind))?;
        let mut params = Vec::new();
        if !self.check(TokenType::RightParen) {
            loop {
                if params.len() >= 255 {
                    self.error(self.peek(), "Can't have more than 255 parameters.");
                }
                params.push(self.consume(TokenType::Identifier, "Expect parameter name.")?);
                if !self.match_token(&[TokenType::Comma]) {
                    break;
                }
            }
        }
        self.consume(TokenType::RightParen, "Expect ')' after parameters.")?;
        self.consume(TokenType::LeftBrace, "Expect '{' before function body.")?;
        let body = self.block()?;
        Some(Stmt::Function { name, params, body })
    }

    fn statement(&mut self) -> Option<Stmt> {
        if self.match_token(&[TokenType::Print]) {
            return self.print_statement();
        }
        if self.match_token(&[TokenType::Return]) {
            return self.return_statement();
        }
        if self.match_token(&[TokenType::LeftBrace]) {
            return Some(Stmt::Block { statements: self.block()? });
        }
        if self.match_token(&[TokenType::If]) {
            return self.if_statement();
        }
        if self.match_token(&[TokenType::While]) {
            return self.while_statement();
        }
        if self.match_token(&[TokenType::For]) {
            return self.for_statement();
        }
        self.expression_statement()
    }

    fn print_statement(&mut self) -> Option<Stmt> {
        let value = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after value.")?;
        Some(Stmt::Print { expression: value })
    }

    fn return_statement(&mut self) -> Option<Stmt> {
        let keyword = self.previous().clone();
        let value = if !self.check(TokenType::Semicolon) {
            Some(self.expression()?)
        } else {
            None
        };
        self.consume(TokenType::Semicolon, "Expect ';' after return value.")?;
        Some(Stmt::Return { keyword, value })
    }

    fn if_statement(&mut self) -> Option<Stmt> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'if'.")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after if condition.")?;
        let then_branch = Box::new(self.statement()?);
        let else_branch = if self.match_token(&[TokenType::Else]) {
            Some(Box::new(self.statement()?))
        } else {
            None
        };
        Some(Stmt::If { condition, then_branch, else_branch })
    }

    fn while_statement(&mut self) -> Option<Stmt> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'while'.")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after condition.")?;
        let body = Box::new(self.statement()?);
        Some(Stmt::While { condition, body })
    }

    fn for_statement(&mut self) -> Option<Stmt> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'for'.")?;
        let initializer = if self.match_token(&[TokenType::Semicolon]) {
            None
        } else if self.match_token(&[TokenType::Var]) {
            Some(self.var_declaration()?)
        } else {
            Some(self.expression_statement()?)
        };
        let condition = if !self.check(TokenType::Semicolon) {
            self.expression()?
        } else {
            Expr::Literal { value: Object::Bool(true) }
        };
        self.consume(TokenType::Semicolon, "Expect ';' after loop condition.")?;
        let increment = if !self.check(TokenType::RightParen) {
            Some(self.expression()?)
        } else {
            None
        };
        self.consume(TokenType::RightParen, "Expect ')' after for clauses.")?;
        let mut body = Box::new(self.statement()?);
        if let Some(increment) = increment {
            body = Box::new(Stmt::Block {
                statements: vec![*body, Stmt::Expression { expression: increment }],
            });
        }
        body = Box::new(Stmt::While { condition, body });
        if let Some(initializer) = initializer {
            body = Box::new(Stmt::Block {
                statements: vec![initializer, *body],
            });
        }
        Some(*body)
    }

    fn expression_statement(&mut self) -> Option<Stmt> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after expression.")?;
        Some(Stmt::Expression { expression: expr })
    }

    fn block(&mut self) -> Option<Vec<Stmt>> {
        let mut statements = Vec::new();
        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            if let Some(stmt) = self.declaration() {
                statements.push(stmt);
            }
        }
        self.consume(TokenType::RightBrace, "Expect '}' after block.")?;
        Some(statements)
    }

    fn expression(&mut self) -> Option<Expr> {
        self.assignment()
    }

    fn assignment(&mut self) -> Option<Expr> {
        let expr = self.or()?;

        if self.match_token(&[TokenType::Equal]) {
            let equals = self.previous().clone();
            let value = self.assignment()?;

            if let Expr::Variable { name } = expr {
                return Some(Expr::Assign {
                    name,
                    value: Box::new(value),
                });
            }

            self.error(&equals, "Invalid assignment target.");
        }

        Some(expr)
    }

    fn or(&mut self) -> Option<Expr> {
        let mut expr = self.and()?;
        while self.match_token(&[TokenType::Or]) {
            let operator = self.previous().clone();
            let right = self.and()?;
            expr = Expr::Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        Some(expr)
    }

    fn and(&mut self) -> Option<Expr> {
        let mut expr = self.equality()?;
        while self.match_token(&[TokenType::And]) {
            let operator = self.previous().clone();
            let right = self.equality()?;
            expr = Expr::Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        Some(expr)
    }

    fn equality(&mut self) -> Option<Expr> {
        let mut expr = self.comparison()?;
        while self.match_token(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        Some(expr)
    }

    fn comparison(&mut self) -> Option<Expr> {
        let mut expr = self.term()?;
        while self.match_token(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous().clone();
            let right = self.term()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        Some(expr)
    }

    fn term(&mut self) -> Option<Expr> {
        let mut expr = self.factor()?;
        while self.match_token(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        Some(expr)
    }

    fn factor(&mut self) -> Option<Expr> {
        let mut expr = self.unary()?;
        while self.match_token(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        Some(expr)
    }

    fn unary(&mut self) -> Option<Expr> {
        if self.match_token(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            return Some(Expr::Unary {
                operator,
                right: Box::new(right),
            });
        }
        self.call()
    }

    fn call(&mut self) -> Option<Expr> {
        let mut expr = self.primary()?;
        while self.match_token(&[TokenType::LeftParen]) {
            expr = self.finish_call(expr)?;
        }
        Some(expr)
    }

    fn finish_call(&mut self, callee: Expr) -> Option<Expr> {
        let mut arguments = Vec::new();
        if !self.check(TokenType::RightParen) {
            loop {
                if arguments.len() >= 255 {
                    self.error(self.peek(), "Can't have more than 255 arguments.");
                }
                arguments.push(self.expression()?);
                if !self.match_token(&[TokenType::Comma]) {
                    break;
                }
            }
        }
        let paren: Token = self.consume(TokenType::RightParen, "Expect ')' after arguments.")?;
        Some(Expr::Call {
            callee: Box::new(callee),
            paren,
            arguments,
        })
    }

    fn primary(&mut self) -> Option<Expr> {
        if self.match_token(&[TokenType::False]) {
            return Some(Expr::Literal {
                value: Object::Bool(false),
            });
        }
        if self.match_token(&[TokenType::True]) {
            return Some(Expr::Literal {
                value: Object::Bool(true),
            });
        }
        if self.match_token(&[TokenType::Nil]) {
            return Some(Expr::Literal { value: Object::Nil });
        }
        if self.match_token(&[TokenType::Number]) {
            return Some(Expr::Literal {
                value: Object::Number(self.previous().lexeme.parse().ok()?),
            });
        }
        if self.match_token(&[TokenType::String]) {
            return Some(Expr::Literal {
                value: Object::String(self.previous().lexeme.clone()),
            });
        }
        if self.match_token(&[TokenType::Identifier]) {
            return Some(Expr::Variable {
                name: self.previous().clone(),
            });
        }
        if self.match_token(&[TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen, "Expect ')' after expression.")?;
            return Some(Expr::Grouping {
                expression: Box::new(expr),
            });
        }
        None
    }

    fn match_token(&mut self, types: &[TokenType]) -> bool {
        for token_type in types {
            if self.check(token_type.clone()) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Option<Token> {
        if self.check(token_type) {
            return Some(self.advance().clone());
        }
        self.error(self.peek(), message);
        None
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().token_type == token_type
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn error(&self, token: &Token, message: &str) {
        eprintln!("[line {}] Error at '{}': {}", token.line, token.lexeme, message);
    }
}