#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Illegal,
    Eof,

    // Identifiers + literals
    Ident(String),
    Int(i64),
    Bool(bool),

    // Statements
    Assign,
    If,
    Else,

    // Operators
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,

    Equal,
    NotEqual,
    LessThan,
    GreaterThan,

    // Delimiters
    Comma,
    Semicolon,
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,

    // Reseved keywords
    Function,
    Let,
    Return,
}