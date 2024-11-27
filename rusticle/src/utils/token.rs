#[derive(Debug, PartialEq, Clone, serde::Deserialize)]
pub enum TokenType {
    // Single-character tokens
    LeftParen, RightParen, LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus, Semicolon, Slash, Star, Mod,
    // One or two character tokens
    Bang, BangEqual, Equal, EqualEqual,
    Greater, GreaterEqual, Less, LessEqual,
    // Literals
    Identifier, String, Number,
    // Keywords
    And, Class, Else, False, For, Fun, If, Nil, Or,
    Print, Return, Super, This, True, Var, While,
    // Special tokens
    Error(String), Eof,
    // Import keyword
    Import, From
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, line: usize) -> Self {
        Token { token_type, lexeme, line }
    }
}