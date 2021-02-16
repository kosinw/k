#[derive(Debug, PartialEq, Clone, Copy)]
pub struct BufferPosition {
    pub line: usize,
    pub column: usize,
}

impl BufferPosition {
    pub fn new(line: usize, column: usize) -> BufferPosition {
        BufferPosition { line, column }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum KeywordKind {
    Fn,
    If,
    Else,
    For,
    While,
    Enum,
    Struct,
    Break,
    Continue,
    Let,
    True,
    False,
    Match,
    Return,
}

#[derive(Debug, PartialEq, Clone)]
pub enum SymbolKind {
    Plus,
    Minus,
    Slash,
    Mod,
    Asterisk,
    Semicolon,
    Colon,
    Assign,
    Comma,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Bang,
    Eq,
    NotEq,
    Lt,
    Gt,
    LtEq,
    GtEq
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    Identifier(String),
    Keyword(KeywordKind),
    IntegerLiteral(i64),
    FloatLiteral(f64),
    StringLiteral(String),
    CharLiteral(char),
    Symbol(SymbolKind),
    Newline,
    Illegal,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub position: BufferPosition,
}

impl Token {
    pub fn new(kind: TokenKind, position: BufferPosition) -> Token {
        Token {
            kind,
            position
        }
    }
}