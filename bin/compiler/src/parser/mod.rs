pub mod assignment;
mod declaration;
mod expression;
mod statement;
mod ty;

use crate::{
    ast::Statement,
    lexer::{Lexer, Token, TokenKind, Tokens},
};

///
#[derive(Debug, Default)]
pub struct Module {
    pub declarations: Vec<Statement>,
}

/// #NOTE: This is a very simple parser without any optimizations
pub struct Parser<'a, I>
where
    I: Iterator<Item = Token>,
{
    lexer: &'a Lexer<'a>,
    tokens: Tokens<I>,
    module: Module,
}

impl<'a, I> Parser<'a, I>
where
    I: Iterator<Item = Token>,
{
    ///
    pub fn new(lexer: &'a Lexer<'a>, tokens: Tokens<I>) -> Self {
        Self {
            lexer,
            tokens,
            module: Module::default(),
        }
    }

    ///
    pub fn parse(&mut self) -> &Module {
        while self.tokens.peek_token().is_some() {
            let expr = self.parse_declaration();
            self.module.declarations.push(expr);
        }

        &self.module
    }

    ///
    fn eat_expect(&mut self, kind: TokenKind) -> Token {
        let token = self.tokens.eat_token();
        assert_eq!(self.lexer.get_token_kind(token), kind);

        token
    }

    ///
    fn eat_token(&mut self) -> Token {
        self.tokens.eat_token()
    }

    ///
    fn peek_token(&mut self) -> Option<Token> {
        self.tokens.peek_token()
    }
}
