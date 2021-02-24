use crate::token::{BufferPosition, KeywordKind, SymbolKind, Token, TokenKind};
use std::collections::HashMap;
use std::vec::Vec;

#[derive(Debug, Clone, PartialEq)]
pub struct Lexer {
    input: Vec<char>,
    cursor: usize,
    current_char: Option<char>,
    current_line: usize,
    current_col: usize,
}

lazy_static! {
    static ref KEYWORDS: HashMap<String, KeywordKind> = {
        let mut m = HashMap::new();
        m.insert("fn".to_owned(), KeywordKind::Fn);
        m.insert("let".to_owned(), KeywordKind::Let);
        m.insert("if".to_owned(), KeywordKind::If);
        m.insert("else".to_owned(), KeywordKind::Else);
        m.insert("for".to_owned(), KeywordKind::For);
        m.insert("while".to_owned(), KeywordKind::While);
        m.insert("enum".to_owned(), KeywordKind::Enum);
        m.insert("struct".to_owned(), KeywordKind::Struct);
        m.insert("break".to_owned(), KeywordKind::Break);
        m.insert("continue".to_owned(), KeywordKind::Continue);
        m.insert("true".to_owned(), KeywordKind::True);
        m.insert("false".to_owned(), KeywordKind::False);
        m.insert("match".to_owned(), KeywordKind::Match);
        m.insert("return".to_owned(), KeywordKind::Return);
        m
    };
}

impl Lexer {
    pub fn new(input: &str) -> Lexer {
        let mut lexer = Lexer {
            input: input.chars().collect(),
            cursor: 0,
            current_char: None,
            current_line: 1,
            current_col: 0,
        };

        lexer.read_char();
        lexer
    }

    fn is_alpha(ch: char) -> bool {
        ch.is_ascii_alphabetic() || ch == '_'
    }

    fn is_digit(ch: char) -> bool {
        ch.is_ascii_digit()
    }

    fn is_alphanumeric(ch: char) -> bool {
        Self::is_alpha(ch) || ch.is_ascii_digit()
    }

    fn read_char(&mut self) -> Option<char> {
        self.current_col += 1;

        let r = if self.cursor >= self.input.len() {
            None
        } else {
            Some(self.input[self.cursor])
        };

        self.current_char = r;

        self.cursor += 1;

        if r == Some('\n') {
            self.current_line += 1;
            self.current_col = 1;
        }

        r
    }

    fn peek_char(&self) -> Option<char> {
        if self.cursor >= self.input.len() {
            None
        } else {
            Some(self.input[self.cursor])
        }
    }

    fn read_identifier(&mut self) -> String {
        let mut buffer = String::new();

        loop {
            buffer.push(self.current_char.unwrap());

            self.read_char();

            if self.current_char == None || !Lexer::is_alphanumeric(self.current_char.unwrap()) {
                break;
            }
        }

        buffer
    }

    // TODO(kosi): Come back and add universal/unicode characters
    fn read_escaped_char_literal(&mut self) -> Option<char> {
        self.read_char();

        match self.current_char {
            Some('\'') | Some('\"') | Some('\\') => self.current_char,
            Some('t') => Some('\t'),
            Some('n') => Some('\n'),
            Some('f') => Some('\x0C'),
            Some('r') => Some('\r'),
            Some('v') => Some('\x0B'),
            _ => None, // TODO(kosi): Add unknown escape character error here
        }
    }

    fn read_char_literal(&mut self) -> Option<char> {
        self.read_char();

        let ret = if self.current_char == Some('\\') {
            self.read_escaped_char_literal()
        } else {
            self.current_char
        };

        self.read_char();

        // TODO(kosi): Generate some error message here
        if self.current_char != Some('\'') {
            None
        } else {
            ret
        }
    }

    fn read_string_literal(&mut self) -> Option<String> {
        let mut buf = String::new();
        
        loop {
            self.read_char();

            match self.current_char {
                None => return None, // TODO(kosi): Add unterminated string error here
                Some('"') => return Some(buf),
                Some('\\') => {
                    if let Some(ch) = self.read_escaped_char_literal() {
                        buf.push(ch);
                    }
                },
                Some(ch) => buf.push(ch)
            }
        }
    }

    fn read_decimal_literal(&mut self) -> i64 {
        let mut buffer = String::new();

        loop {
            buffer.push(self.current_char.unwrap());

            self.read_char();

            if self.current_char == None || !Lexer::is_digit(self.current_char.unwrap()) {
                break;
            }
        }

        buffer.parse::<i64>().unwrap()
    }

    fn lookup_identifier(identifier: String) -> TokenKind {
        if let Some(keyword) = KEYWORDS.get(&identifier) {
            TokenKind::Keyword(*keyword)
        } else {
            TokenKind::Identifier(identifier)
        }
    }

    fn skip_line(&mut self) {
        loop {
            let next_ch = self.read_char();

            if next_ch == None || next_ch == Some('\n') {
                break;
            }
        }
    }

    // TODO(kosi): Fix this function so it doesn't look like this mess
    fn skip_whitespace(&mut self) {
        loop {
            if let Some(current_char) = self.current_char {
                if current_char != '\n' && current_char.is_whitespace() {
                    self.read_char();
                    continue;
                } else if current_char == '/' {
                    if let Some(next_ch) = self.peek_char() {
                        if next_ch == '/' {
                            self.read_char();
                            self.skip_line();
                            break;
                        }
                    }
                }

                break;
            }

            break;
        }
    }

    pub fn next_token(&mut self) -> Token {
        let token = self.next();

        match token {
            None => Token::EOF,
            Some(tok) => tok
        }
    }
}

impl Iterator for Lexer {
    type Item = Token;

    // TODO(kosi): Rewrite this to take errors into account
    // maybe call an error method that accumulates errors in an
    // internal buffer in Lexer struct?
    //
    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();

        let current_line = self.current_line;
        let current_col = self.current_col;

        let token_kind = match self.current_char {
            Some('=') => match self.peek_char() {
                Some('=') => {
                    self.read_char();
                    TokenKind::Symbol(SymbolKind::Eq)
                }
                _ => TokenKind::Symbol(SymbolKind::Assign),
            },
            Some('+') => TokenKind::Symbol(SymbolKind::Plus),
            Some('(') => TokenKind::Symbol(SymbolKind::LeftParen),
            Some(')') => TokenKind::Symbol(SymbolKind::RightParen),
            Some('{') => TokenKind::Symbol(SymbolKind::LeftBrace),
            Some('}') => TokenKind::Symbol(SymbolKind::RightBrace),
            Some(',') => TokenKind::Symbol(SymbolKind::Comma),
            Some(';') => TokenKind::Symbol(SymbolKind::Semicolon),
            Some(':') => TokenKind::Symbol(SymbolKind::Colon),
            Some('!') => match self.peek_char() {
                Some('=') => {
                    self.read_char();
                    TokenKind::Symbol(SymbolKind::NotEq)
                }
                _ => TokenKind::Symbol(SymbolKind::Bang),
            },
            Some('-') => TokenKind::Symbol(SymbolKind::Minus),
            Some('*') => TokenKind::Symbol(SymbolKind::Asterisk),
            Some('<') => TokenKind::Symbol(SymbolKind::Lt),
            Some('>') => TokenKind::Symbol(SymbolKind::Gt),
            Some('/') => TokenKind::Symbol(SymbolKind::Slash),
            Some('%') => TokenKind::Symbol(SymbolKind::Mod),
            Some('\n') => TokenKind::Newline,
            Some('\'') => {
                let lit = self.read_char_literal();
                if let Some(l) = lit {
                    TokenKind::CharLiteral(l)
                } else {
                    TokenKind::Illegal
                }
            }
            Some('"') => {
                let lit = self.read_string_literal();

                if let Some(l) = lit {
                    TokenKind::StringLiteral(l)
                } else {
                    TokenKind::Illegal
                }
            }
            Some(ch) => {
                if Lexer::is_alpha(ch) {
                    let ident = self.read_identifier();
                    let token_kind = Lexer::lookup_identifier(ident);
                    return Some(Token::new(
                        token_kind,
                        BufferPosition::new(current_line, current_col),
                    ));
                } else if Lexer::is_digit(ch) {
                    // TODO(kosi): Comeback and add hex, binary, and octal literals
                    // TODO(kosi): Comeback and add floating point literals
                    let literal = self.read_decimal_literal();

                    return Some(Token::new(
                        TokenKind::IntegerLiteral(literal),
                        BufferPosition::new(current_line, current_col),
                    ));
                } else {
                    TokenKind::Illegal
                }
            }
            None => return None,
        };

        self.read_char();

        Some(Token::new(
            token_kind,
            BufferPosition::new(current_line, current_col),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::*;

    #[test]
    fn test_string_literals() {
        let input = " \"test\"
        \"newline\n\"";

        let tests = vec![
            TokenKind::StringLiteral("test".to_owned()),
            TokenKind::Newline,
            TokenKind::StringLiteral("newline\n".to_owned())
        ];

        let mut lexer = Lexer::new(input);

        for test in tests {
            let token = lexer.next();

            assert_eq!(test, token.unwrap().kind);
        }
    }

    #[test]
    fn test_comments() {
        let input = "// no tokens
        let token;";

        let tests = vec![
            TokenKind::Newline,
            TokenKind::Keyword(KeywordKind::Let),
            TokenKind::Identifier("token".to_owned()),
            TokenKind::Symbol(SymbolKind::Semicolon)
        ];

        let mut lexer = Lexer::new(input);

       for test in tests {
           let token = lexer.next();

           assert_eq!(test, token.unwrap().kind);
       }
    }

    #[test]
    fn test_char_literals() {
        let input = r#"'a'
        'c'
        '\n'
        '\\'
        '\f'
        'ew'"#;

        let tests = vec![
            TokenKind::CharLiteral('a'),
            TokenKind::Newline,
            TokenKind::CharLiteral('c'),
            TokenKind::Newline,
            TokenKind::CharLiteral('\n'),
            TokenKind::Newline,
            TokenKind::CharLiteral('\\'),
            TokenKind::Newline,
            TokenKind::CharLiteral('\x0C'),
            TokenKind::Newline,
            TokenKind::Illegal,
        ];

        let mut lexer = Lexer::new(input);

        for test in tests {
            let token = lexer.next();

            assert_eq!(test, token.unwrap().kind);
        }
    }

    #[test]
    fn test_tokens_3() {
        let input = "10 == 10; 10 != 9;";

        let tests = vec![
            TokenKind::IntegerLiteral(10),
            TokenKind::Symbol(SymbolKind::Eq),
            TokenKind::IntegerLiteral(10),
            TokenKind::Symbol(SymbolKind::Semicolon),
            TokenKind::IntegerLiteral(10),
            TokenKind::Symbol(SymbolKind::NotEq),
            TokenKind::IntegerLiteral(9),
            TokenKind::Symbol(SymbolKind::Semicolon),
        ];

        let mut lexer = Lexer::new(input);

        for test in tests {
            let token = lexer.next();

            assert_eq!(test, token.unwrap().kind);
        }
    }

    #[test]
    fn test_tokens_2() {
        let input = "fn main(): void {
            if (5 < 10) {
                return true;
            } else {
                return false;
            }
        }";

        let tests = vec![
            TokenKind::Keyword(KeywordKind::Fn),
            TokenKind::Identifier("main".to_owned()),
            TokenKind::Symbol(SymbolKind::LeftParen),
            TokenKind::Symbol(SymbolKind::RightParen),
            TokenKind::Symbol(SymbolKind::Colon),
            TokenKind::Identifier("void".to_owned()),
            TokenKind::Symbol(SymbolKind::LeftBrace),
            TokenKind::Newline,
            TokenKind::Keyword(KeywordKind::If),
            TokenKind::Symbol(SymbolKind::LeftParen),
            TokenKind::IntegerLiteral(5),
            TokenKind::Symbol(SymbolKind::Lt),
            TokenKind::IntegerLiteral(10),
            TokenKind::Symbol(SymbolKind::RightParen),
            TokenKind::Symbol(SymbolKind::LeftBrace),
            TokenKind::Newline,
            TokenKind::Keyword(KeywordKind::Return),
            TokenKind::Keyword(KeywordKind::True),
            TokenKind::Symbol(SymbolKind::Semicolon),
            TokenKind::Newline,
            TokenKind::Symbol(SymbolKind::RightBrace),
            TokenKind::Keyword(KeywordKind::Else),
            TokenKind::Symbol(SymbolKind::LeftBrace),
            TokenKind::Newline,
            TokenKind::Keyword(KeywordKind::Return),
            TokenKind::Keyword(KeywordKind::False),
            TokenKind::Symbol(SymbolKind::Semicolon),
            TokenKind::Newline,
            TokenKind::Symbol(SymbolKind::RightBrace),
            TokenKind::Newline,
            TokenKind::Symbol(SymbolKind::RightBrace),
        ];

        let mut lexer = Lexer::new(input);

        for test in tests {
            let tok = lexer.next();

            assert_eq!(test, tok.unwrap().kind);
        }
    }

    #[test]
    fn test_tokens_1() {
        let input = "fn main(): void {
                        let five: i32 = 5
                        let ten = 10; let fifteen = five + ten     
                    }
                    !-/*%
                    5 < 10 > 5;";
        let tests = vec![
            TokenKind::Keyword(KeywordKind::Fn),
            TokenKind::Identifier("main".to_owned()),
            TokenKind::Symbol(SymbolKind::LeftParen),
            TokenKind::Symbol(SymbolKind::RightParen),
            TokenKind::Symbol(SymbolKind::Colon),
            TokenKind::Identifier("void".to_owned()),
            TokenKind::Symbol(SymbolKind::LeftBrace),
            TokenKind::Newline,
            TokenKind::Keyword(KeywordKind::Let),
            TokenKind::Identifier("five".to_owned()),
            TokenKind::Symbol(SymbolKind::Colon),
            TokenKind::Identifier("i32".to_owned()),
            TokenKind::Symbol(SymbolKind::Assign),
            TokenKind::IntegerLiteral(5),
            TokenKind::Newline,
            TokenKind::Keyword(KeywordKind::Let),
            TokenKind::Identifier("ten".to_owned()),
            TokenKind::Symbol(SymbolKind::Assign),
            TokenKind::IntegerLiteral(10),
            TokenKind::Symbol(SymbolKind::Semicolon),
            TokenKind::Keyword(KeywordKind::Let),
            TokenKind::Identifier("fifteen".to_owned()),
            TokenKind::Symbol(SymbolKind::Assign),
            TokenKind::Identifier("five".to_owned()),
            TokenKind::Symbol(SymbolKind::Plus),
            TokenKind::Identifier("ten".to_owned()),
            TokenKind::Newline,
            TokenKind::Symbol(SymbolKind::RightBrace),
            TokenKind::Newline,
            TokenKind::Symbol(SymbolKind::Bang),
            TokenKind::Symbol(SymbolKind::Minus),
            TokenKind::Symbol(SymbolKind::Slash),
            TokenKind::Symbol(SymbolKind::Asterisk),
            TokenKind::Symbol(SymbolKind::Mod),
            TokenKind::Newline,
            TokenKind::IntegerLiteral(5),
            TokenKind::Symbol(SymbolKind::Lt),
            TokenKind::IntegerLiteral(10),
            TokenKind::Symbol(SymbolKind::Gt),
            TokenKind::IntegerLiteral(5),
            TokenKind::Symbol(SymbolKind::Semicolon),
        ];

        let mut lexer = Lexer::new(input);

        for test in tests {
            let tok = lexer.next();

            assert_eq!(test, tok.unwrap().kind);
        }
    }

    #[test]
    fn test_next_token() {
        let input = "=+(){},;";

        let tests = vec![
            Token::new(
                TokenKind::Symbol(SymbolKind::Assign),
                BufferPosition::new(1, 1),
            ),
            Token::new(
                TokenKind::Symbol(SymbolKind::Plus),
                BufferPosition::new(1, 2),
            ),
            Token::new(
                TokenKind::Symbol(SymbolKind::LeftParen),
                BufferPosition::new(1, 3),
            ),
            Token::new(
                TokenKind::Symbol(SymbolKind::RightParen),
                BufferPosition::new(1, 4),
            ),
            Token::new(
                TokenKind::Symbol(SymbolKind::LeftBrace),
                BufferPosition::new(1, 5),
            ),
            Token::new(
                TokenKind::Symbol(SymbolKind::RightBrace),
                BufferPosition::new(1, 6),
            ),
            Token::new(
                TokenKind::Symbol(SymbolKind::Comma),
                BufferPosition::new(1, 7),
            ),
            Token::new(
                TokenKind::Symbol(SymbolKind::Semicolon),
                BufferPosition::new(1, 8),
            ),
        ];
        let mut lexer = Lexer::new(input);

        for test in tests {
            let tok = lexer.next();

            assert_eq!(Some(test), tok);
        }
    }
}
