use std::fs::File;
use std::io::prelude::*;
use std::iter::Iterator;
use std::path::Path;
use std::string::String;

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

#[derive(Debug, PartialEq, Clone)]
pub enum KeywordKind {
    Fn,
    If,
    Else,
    For,
    While,
    EnumKeyowrd,
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
    Div,
    Mod,
    Asterisk,
    Semicolon,
    Equals,
    LeftParen,
    RightParent,
    LeftBrace,
    RightBrace,
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
    Whitespace,
    Newline,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub position: BufferPosition,
}

#[derive(Debug, Clone)]
pub struct Lexer {
    position: BufferPosition,
    current_lexeme: String,
    cursor: usize,
    pub filename: Option<String>,
    buffer: Vec<char>,
}

impl Token {
    pub fn new(kind: TokenKind, position: BufferPosition) -> Token {
        Token { kind, position }
    }
}

// TODO(kosi): Add buffered file I/O and tokenization (possible thru iterators?)
// TODO(kosi): Add multithreading + asynchronous processing
//

impl Lexer {
    pub fn new(text: &str) -> Lexer {
        Lexer {
            position: BufferPosition::new(0, 0),
            current_lexeme: String::new(),
            filename: None,
            cursor: 0,
            buffer: text.chars().collect(),
        }
    }

    // TODO(kosi): Replace with better error messaging
    pub fn from_file<P>(path: P) -> std::io::Result<Lexer>
    where
        P: AsRef<Path>,
    {
        let mut file = File::open(path.as_ref())?;
        let mut contents = String::new();

        file.read_to_string(&mut contents)?;

        let filename = String::from(path.as_ref().to_str().unwrap());

        Ok(Lexer {
            filename: Some(filename),
            cursor: 0,
            position: BufferPosition::new(0, 0),
            current_lexeme: String::new(),
            buffer: contents.chars().collect(),
        })
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        while self.cursor < self.buffer.len() {
            let lexeme = String::from(self.buffer[self.cursor]);

            // Increment cursor position and column
            {
                self.cursor += 1;
                self.position.column += 1;
            }

            return Some(Token::new(TokenKind::Identifier(lexeme), self.position));
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::Lexer;

    #[test]
    fn binary_addition_works() {
        let lexer = Lexer::new("3 + 4");

        let tokens: Vec<_> = lexer.collect();

        println!("{:?}", tokens);
    }

    #[test]
    fn load_file_works() {
        let filename = "samples/arithmetic.k";
        let lexer = Lexer::from_file(filename).unwrap();

        match &lexer.filename {
            Some(name) => assert_eq!(name, filename),
            None => assert!(false, format!("Expected filename: {}", filename)),
        }
    }
}
