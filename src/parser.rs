use crate::ast::*;
use crate::lexer::*;
use crate::token::*;

pub struct Parser {
    lexer: Box<Lexer>,
    cursor_token: Token,
    peek_token: Token,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Parser {
        let mut parser = Parser {
            lexer: Box::new(lexer),
            cursor_token: Token::EOF,
            peek_token: Token::EOF,
        };

        // Read two tokens so cursor_token points to the first token
        parser.next_token();
        parser.next_token();

        parser
    }

    pub fn parse(&mut self) -> Program {
        let mut program = Program::new();

        while self.cursor_token.kind != TokenKind::EOF {
            let stmt = self.parse_stmt();

            if let Some(ok) = stmt {
                program.0.push(ok);
            }

            self.next_token();
        }

        program
    }

    fn parse_stmt(&mut self) -> Option<Stmt> {
        match &self.cursor_token.kind {
            TokenKind::Keyword(KeywordKind::Let) => self.parse_let_stmt(),
            _ => None,
        }
    }

    fn parse_let_stmt(&mut self) -> Option<Stmt> {
        // If the next token is an identifier, increment
        match &self.peek_token.kind {
            TokenKind::Identifier(_) => self.next_token(),
            _ => return None,
        };

        let name = match self.parse_ident() {
            Some(name) => name,
            None => return None,
        };

        if !self.expect_next_token(&TokenKind::Symbol(SymbolKind::Assign)) {
            return None;
        }

        self.next_token();

        // TODO(kosi): Skip expressions until we see end of statement,
        // change to parsing expression later.
        while !self.cursor_is_end_stmt() {
            self.next_token();
        }

        Some(Stmt::Let(name, Expr::Empty))
    }

    fn cursor_is_end_stmt(&self) -> bool {
        match &self.cursor_token.kind {
            TokenKind::Newline => true,
            TokenKind::Symbol(SymbolKind::Semicolon) => true,
            _ => false
        }
    }

    fn parse_ident(&mut self) -> Option<Identifier> {
        match &self.cursor_token.kind {
            TokenKind::Identifier(ref ident) => Some(Identifier::new(ident.clone())),
            _ => None,
        }
    }

    fn peek_token_is(&self, _kind: &TokenKind) -> bool {
        matches!(&self.peek_token.kind, _kind)
    }

    fn expect_next_token(&mut self, kind: &TokenKind) -> bool {
        if self.peek_token_is(kind) {
            self.next_token();
            true
        } else {
            // TODO(kosi): Add error handling here
            false
        }
    }

    fn next_token(&mut self) {
        self.cursor_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_let_statements() {
        let input = "
        let x = 5;
        let y = 10
        let foobar = 838383;
        ";

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse();

        println!("{:?}", program);

        assert_eq!(
            program.0.len(),
            3,
            "Program does not contain 3 statements. Got {}",
            program.0.len()
        );

        let tests = vec!["x", "y", "foobar"];

        for (i, test) in tests.iter().enumerate() {
            let stmt = &program.0[i];

            test_let_statement(stmt, test);
        }
    }

    fn test_let_statement(stmt: &Stmt, identifier: &str) {
        if let Stmt::Let(name, _value) = stmt {
            assert_eq!(
                name.0, identifier,
                "Name is not {}. Got {}",
                identifier, name.0
            );
        } else {
            assert!(false, "stmt is not Stmt::Let. Got Stmt::{:?}", *stmt);
        }
    }
}
