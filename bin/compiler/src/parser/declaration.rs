use crate::{
    ast::{FunctionParam, ParamRef, Statement},
    lexer::{Keyword, Token, TokenKind},
    parser::Parser,
};

impl<'a, I> Parser<'a, I>
where
    I: Iterator<Item = Token>,
{
    ///
    pub(super) fn parse_declaration(&mut self) -> Statement {
        let token = self.peek_token().unwrap();
        let kind = self.lexer.get_token_kind(token);

        match kind {
            TokenKind::Keyword(Keyword::Fn) => self.parse_fn(),
            TokenKind::Keyword(Keyword::Let) => self.parse_let(),
            _ => panic!("unexpected token: {kind:?}"),
        }
    }

    ///
    fn parse_fn(&mut self) -> Statement {
        self.eat_expect(TokenKind::Keyword(Keyword::Fn));
        let name = {
            self.lexer
                .get_identifier(self.eat_expect(TokenKind::Identifier))
                .unwrap()
                .to_string()
        };

        let params = self.parse_function_parameters();
        self.eat_expect(TokenKind::Arrow);
        let ret = self.parse_type();
        let body = self.parse_statement();

        Statement::Function {
            name,
            params,
            ret: Box::new(ret),
            body: Box::new(body),
        }
    }

    ///
    fn parse_function_parameters(&mut self) -> Vec<FunctionParam> {
        self.eat_expect(TokenKind::ParenOpen);
        let mut params = Vec::new();

        loop {
            let name = {
                let token = self.eat_token();
                self.lexer.get_identifier(token).unwrap().to_string()
            };

            self.eat_expect(TokenKind::Colon);
            let param_ref = self.parse_ref();
            let ty = self.parse_type();

            params.push(FunctionParam {
                name,
                param_ref,
                ty: Box::new(ty),
            });

            let token = self.eat_token();
            let kind = self.lexer.get_token_kind(token);
            match kind {
                TokenKind::Comma => continue,
                TokenKind::ParenClose => break,
                _ => panic!("unexpected token {token:?}"),
            }
        }

        params
    }

    ///
    fn parse_ref(&mut self) -> ParamRef {
        if let Some(token) = self.peek_token() {
            let kind = self.lexer.get_token_kind(token);
            match kind {
                TokenKind::Amp => {
                    self.eat_token();
                    ParamRef::Ref
                }
                _ => ParamRef::Val,
            }
        } else {
            ParamRef::Val
        }
    }

    ///
    fn parse_let(&mut self) -> Statement {
        self.eat_expect(TokenKind::Keyword(Keyword::Let));
        let name = {
            let token = self.eat_token();
            self.lexer.get_identifier(token).unwrap().to_string()
        };
        self.eat_expect(TokenKind::Colon);
        let ty = self.parse_type();
        self.eat_expect(TokenKind::Equal);
        let expr = self.parse_expression().unwrap();
        self.eat_expect(TokenKind::Semicolon);

        Statement::Let {
            name,
            ty,
            expr: Box::new(expr),
        }
    }
}
