use crate::{
    ast::Statement,
    lexer::{Keyword, Token, TokenKind},
    parser::Parser,
};

impl<'a, I> Parser<'a, I>
where
    I: Iterator<Item = Token>,
{
    ///
    pub(super) fn parse_statement(&mut self) -> Statement {
        let token = self.peek_token().unwrap();
        let kind = self.lexer.get_token_kind(token);

        match kind {
            TokenKind::Keyword(Keyword::For) => self.parse_for(),
            TokenKind::Keyword(Keyword::While) => self.parse_while(),
            TokenKind::Keyword(Keyword::If) => self.parse_if(),
            TokenKind::Keyword(Keyword::Break) => self.parse_break(),
            TokenKind::Keyword(Keyword::Continue) => self.parse_continue(),
            TokenKind::Keyword(Keyword::Return) => self.parse_return(),
            TokenKind::CurlyBraceOpen => self.parse_block(),
            TokenKind::Identifier => self.parse_asignment_semi(),
            _ => self.parse_declaration(),
        }
    }

    ///
    fn parse_for(&mut self) -> Statement {
        self.eat_expect(TokenKind::Keyword(Keyword::For));
        let ident = self.parse_identifier();
        self.eat_expect(TokenKind::Keyword(Keyword::In));
        let range = self.parse_expression().unwrap();
        let body = self.parse_block();

        Statement::For {
            value: Box::new(ident),
            range: Box::new(range),
            body: Box::new(body),
        }
    }

    ///
    fn parse_while(&mut self) -> Statement {
        self.eat_expect(TokenKind::Keyword(Keyword::While));
        let condition = self.parse_expression().unwrap();
        let body = self.parse_block();

        Statement::While {
            condition: Box::new(condition),
            body: Box::new(body),
        }
    }

    ///
    fn parse_if(&mut self) -> Statement {
        self.eat_expect(TokenKind::Keyword(Keyword::If));
        let condition = self.parse_expression().unwrap();
        let on_true = self.parse_block();
        let on_false = {
            if let Some(token) = self.peek_token() {
                let kind = self.lexer.get_token_kind(token);
                match kind {
                    TokenKind::Keyword(Keyword::Else) => {
                        self.eat_token();
                        Some(self.parse_block())
                    }
                    _ => None,
                }
            } else {
                None
            }
        };

        Statement::If {
            condition: Box::new(condition),
            on_true: Box::new(on_true),
            on_false: on_false.map(Box::new),
        }
    }

    ///
    fn parse_break(&mut self) -> Statement {
        self.eat_expect(TokenKind::Keyword(Keyword::Break));
        self.eat_expect(TokenKind::Semicolon);

        Statement::Break
    }

    ///
    fn parse_continue(&mut self) -> Statement {
        self.eat_expect(TokenKind::Keyword(Keyword::Continue));
        self.eat_expect(TokenKind::Semicolon);

        Statement::Continue
    }

    ///
    fn parse_return(&mut self) -> Statement {
        self.eat_expect(TokenKind::Keyword(Keyword::Return));
        let token = self.peek_token().unwrap();
        match self.lexer.get_token_kind(token) {
            TokenKind::Semicolon => {
                self.eat_token();
                Statement::Return { expr: None }
            }
            _ => {
                let expr = self.parse_expression().unwrap();
                let stmt = Statement::Return {
                    expr: Some(Box::new(expr)),
                };

                self.eat_expect(TokenKind::Semicolon);

                stmt
            }
        }
    }

    ///
    fn parse_block(&mut self) -> Statement {
        self.eat_expect(TokenKind::CurlyBraceOpen);
        let mut nodes = Vec::new();
        while let Some(token) = self.tokens.peek_token() {
            let kind = self.lexer.get_token_kind(token);
            if kind == TokenKind::CurlyBraceClose {
                break;
            }

            let stmt = self.parse_statement();
            nodes.push(stmt);
        }

        self.eat_expect(TokenKind::CurlyBraceClose);

        Statement::Block { nodes }
    }
}
