//! https://en.cppreference.com/w/cpp/language/operator_precedence

use crate::{
    ast::{BinaryOp, Expression, UnaryOp},
    lexer::{Lexer, Token, TokenKind, Tokens},
    parser::Parser,
};

impl<'a, I> Parser<'a, I>
where
    I: Iterator<Item = Token>,
{
    ///
    pub(super) fn parse_expression(&mut self) -> Option<Expression> {
        Self::parse_expression_impl(self.lexer, &mut self.tokens)
    }

    ///
    fn parse_expression_impl(
        lexer: &'a Lexer<'a>,
        tokens: &mut Tokens<I>,
    ) -> Option<Expression> {
        Self::parse_logical_or(lexer, tokens)
    }

    ///
    fn parse_binary(
        lexer: &'a Lexer<'a>,
        tokens: &mut Tokens<I>,
        op: BinaryOp,
        needle: TokenKind,
        mut other_parse: impl FnMut(&'a Lexer<'a>, &mut Tokens<I>) -> Option<Expression>,
    ) -> Option<Expression> {
        let mut lhs = other_parse(lexer, tokens)?;

        while let Some(token) = tokens.peek_token() {
            let kind = lexer.get_token_kind(token);
            if kind != needle {
                break;
            }

            lhs = Self::make_binary(lhs, op, other_parse(lexer, tokens).unwrap());
        }

        Some(lhs)
    }

    ///
    fn make_binary(lhs: Expression, op: BinaryOp, rhs: Expression) -> Expression {
        let lhs = Box::new(lhs);
        let rhs = Box::new(rhs);
        Expression::Binary { lhs, op, rhs }
    }

    /// Precedence: 15
    fn parse_logical_or(
        lexer: &'a Lexer<'a>,
        tokens: &mut Tokens<I>,
    ) -> Option<Expression> {
        Self::parse_binary(
            lexer,
            tokens,
            BinaryOp::Or,
            TokenKind::PipePipe,
            |lexer, tokens| Self::parse_logical_and(lexer, tokens),
        )
    }

    /// Precedence: 14
    fn parse_logical_and(
        lexer: &'a Lexer<'a>,
        tokens: &mut Tokens<I>,
    ) -> Option<Expression> {
        Self::parse_binary(
            lexer,
            tokens,
            BinaryOp::And,
            TokenKind::AmpAmp,
            |lexer, tokens| Self::parse_bitwise_or(lexer, tokens),
        )
    }

    /// Precedence: 13
    fn parse_bitwise_or(
        lexer: &'a Lexer<'a>,
        tokens: &mut Tokens<I>,
    ) -> Option<Expression> {
        Self::parse_binary(
            lexer,
            tokens,
            BinaryOp::BitOr,
            TokenKind::Pipe,
            |lexer, tokens| Self::parse_bitwise_xor(lexer, tokens),
        )
    }

    /// Precedence: 12
    fn parse_bitwise_xor(
        lexer: &'a Lexer<'a>,
        tokens: &mut Tokens<I>,
    ) -> Option<Expression> {
        Self::parse_binary(
            lexer,
            tokens,
            BinaryOp::Xor,
            TokenKind::Caret,
            |lexer, tokens| Self::parse_bitwise_and(lexer, tokens),
        )
    }

    /// Precedence: 11
    fn parse_bitwise_and(
        lexer: &'a Lexer<'a>,
        tokens: &mut Tokens<I>,
    ) -> Option<Expression> {
        Self::parse_binary(
            lexer,
            tokens,
            BinaryOp::BitAnd,
            TokenKind::Amp,
            |lexer, tokens| Self::parse_eq(lexer, tokens),
        )
    }

    /// Precedence: 10
    fn parse_eq(lexer: &'a Lexer<'a>, tokens: &mut Tokens<I>) -> Option<Expression> {
        let mut lhs = Self::parse_relational(lexer, tokens)?;

        loop {
            let token = tokens.peek_token().unwrap();
            let kind = lexer.get_token_kind(token);
            let op = match kind {
                TokenKind::EqualEqual => {
                    tokens.eat_token();
                    BinaryOp::Equal
                }
                TokenKind::ExclaimEqual => {
                    tokens.eat_token();
                    BinaryOp::NotEqual
                }
                _ => break,
            };

            lhs = Self::make_binary(
                lhs,
                op,
                Self::parse_relational(lexer, tokens).unwrap(),
            );
        }

        Some(lhs)
    }

    /// Precedence: 9
    fn parse_relational(
        lexer: &'a Lexer<'a>,
        tokens: &mut Tokens<I>,
    ) -> Option<Expression> {
        let mut lhs = Self::parse_shift(lexer, tokens)?;

        loop {
            let token = tokens.peek_token().unwrap();
            let kind = lexer.get_token_kind(token);
            let op = match kind {
                TokenKind::Less => {
                    tokens.eat_token();
                    BinaryOp::Less
                }
                TokenKind::LessEqual => {
                    tokens.eat_token();
                    BinaryOp::LessEqual
                }
                TokenKind::Greater => {
                    tokens.eat_token();
                    BinaryOp::Greater
                }
                TokenKind::GreaterEqual => {
                    tokens.eat_token();
                    BinaryOp::GreaterEqual
                }
                _ => break,
            };

            lhs = Self::make_binary(lhs, op, Self::parse_shift(lexer, tokens).unwrap());
        }

        Some(lhs)
    }

    /// Precedence: 7
    fn parse_shift(lexer: &'a Lexer<'a>, tokens: &mut Tokens<I>) -> Option<Expression> {
        let mut lhs = Self::parse_add(lexer, tokens)?;

        loop {
            let token = tokens.peek_token().unwrap();
            let kind = lexer.get_token_kind(token);
            let op = match kind {
                TokenKind::LessLess => {
                    tokens.eat_token();
                    BinaryOp::Shl
                }
                TokenKind::GreaterGreater => {
                    tokens.eat_token();
                    BinaryOp::Shr
                }
                _ => break,
            };

            lhs = Self::make_binary(lhs, op, Self::parse_add(lexer, tokens).unwrap());
        }

        Some(lhs)
    }

    /// Precedence: 6
    fn parse_add(lexer: &'a Lexer<'a>, tokens: &mut Tokens<I>) -> Option<Expression> {
        let mut lhs = Self::parse_mul(lexer, tokens)?;

        loop {
            let token = tokens.peek_token().unwrap();
            let kind = lexer.get_token_kind(token);
            let op = match kind {
                TokenKind::Plus => {
                    tokens.eat_token();
                    BinaryOp::Add
                }
                TokenKind::Minus => {
                    tokens.eat_token();
                    BinaryOp::Sub
                }
                _ => break,
            };

            lhs = Self::make_binary(lhs, op, Self::parse_mul(lexer, tokens).unwrap());
        }

        Some(lhs)
    }

    /// Precedence: 5
    fn parse_mul(lexer: &'a Lexer<'a>, tokens: &mut Tokens<I>) -> Option<Expression> {
        let mut lhs = Self::parse_unary(lexer, tokens)?;

        loop {
            let token = tokens.peek_token().unwrap();
            let kind = lexer.get_token_kind(token);
            let op = match kind {
                TokenKind::Star => {
                    tokens.eat_token();
                    BinaryOp::Mul
                }
                TokenKind::Slash => {
                    tokens.eat_token();
                    BinaryOp::Div
                }
                TokenKind::Percent => {
                    tokens.eat_token();
                    BinaryOp::Mod
                }
                _ => break,
            };

            lhs = Self::make_binary(lhs, op, Self::parse_unary(lexer, tokens).unwrap());
        }

        Some(lhs)
    }

    ///
    fn make_unary_impl(
        lexer: &'a Lexer<'a>,
        tokens: &mut Tokens<I>,
        op: UnaryOp,
        mut parse: impl FnMut(&'a Lexer<'a>, &mut Tokens<I>) -> Expression,
    ) -> Expression {
        let expr = parse(lexer, tokens);
        let expr = Box::new(expr);
        Expression::Unary { op, expr }
    }

    ///
    fn make_unary(
        lexer: &'a Lexer<'a>,
        tokens: &mut Tokens<I>,
        op: UnaryOp,
    ) -> Expression {
        Self::make_unary_impl(
            lexer, //
            tokens,
            op,
            |lexer, tokens| Self::parse_unary(lexer, tokens).unwrap(),
        )
    }

    /// Precedence: 3
    fn parse_unary(lexer: &'a Lexer<'a>, tokens: &mut Tokens<I>) -> Option<Expression> {
        let token = tokens.peek_token().unwrap();
        let kind = lexer.get_token_kind(token);

        match kind {
            TokenKind::Minus => {
                tokens.eat_token();
                Some(Self::make_unary(lexer, tokens, UnaryOp::Neg))
            }
            // TokenKind::Exclaim => {
            //     tokens.eat_token();
            //     Self::make_unary(lexer, tokens, UnaryOp::Neg)
            // }
            // TokenKind::Tilde => {
            //     tokens.eat_token();
            //     BinaryOp::Mod
            // }
            _ => Self::parse_nested(lexer, tokens),
        }
    }

    /// Precedence: 2
    pub(super) fn parse_nested(
        lexer: &'a Lexer<'a>,
        tokens: &mut Tokens<I>,
    ) -> Option<Expression> {
        let mut lhs = Self::parse_term(lexer, tokens)?;

        loop {
            let token = tokens.peek_token().unwrap();
            let kind = lexer.get_token_kind(token);
            match kind {
                TokenKind::ParenOpen => {
                    tokens.eat_token();
                    assert!(
                        matches!(lhs, Expression::Identifier { .. }),
                        "expected an identifier"
                    );

                    let mut args = Vec::new();
                    loop {
                        args.push(Self::parse_expression_impl(lexer, tokens).unwrap());

                        if let Some(token) = tokens.peek_token() {
                            let kind = lexer.get_token_kind(token);
                            if kind == TokenKind::Comma {
                                continue;
                            }
                        }
                        break;
                    }

                    lhs = Expression::Call {
                        func: Box::new(lhs),
                        args,
                    }
                }
                TokenKind::SquareOpen => {
                    tokens.eat_token();
                    let index = Self::parse_expression_impl(lexer, tokens).unwrap();

                    let token = tokens.eat_token();
                    assert_eq!(lexer.get_token_kind(token), TokenKind::SquareClose);

                    let object = lhs;

                    lhs = Expression::Subscript {
                        object: Box::new(object),
                        index: Box::new(index),
                    };
                }
                TokenKind::Dot => {
                    tokens.eat_token();
                    let token = tokens.peek_token().unwrap();
                    let kind = lexer.get_token_kind(token);

                    match kind {
                        TokenKind::Dot => {
                            tokens.eat_token();
                            let expr =
                                Self::parse_expression_impl(lexer, tokens).unwrap();

                            lhs = Expression::Range {
                                from: Box::new(lhs),
                                to: Box::new(expr),
                            }
                        }
                        TokenKind::Identifier => {
                            todo!("member access")
                        }
                        _ => panic!("unexpected token"),
                    }
                }
                _ => break,
            };
        }

        Some(lhs)
    }

    /// Precedence: 1
    fn parse_term(lexer: &'a Lexer<'a>, tokens: &mut Tokens<I>) -> Option<Expression> {
        let token = tokens.peek_token().unwrap();
        let kind = lexer.get_token_kind(token);
        match kind {
            TokenKind::ParenOpen => {
                tokens.eat_token();
                let expr = Self::parse_expression_impl(lexer, tokens).unwrap();
                assert_eq!(
                    lexer.get_token_kind(tokens.eat_token()),
                    TokenKind::ParenClose
                );

                Some(expr)
            }
            TokenKind::Identifier => {
                tokens.eat_token();
                let identifier = lexer.get_identifier(token).unwrap();
                Some(Expression::Identifier {
                    name: identifier.to_string(),
                })
            }
            TokenKind::Integer => {
                tokens.eat_token();
                let integer = lexer.get_integer(token).unwrap();
                Some(Expression::Literal(integer))
            }
            _ => None,
        }
    }

    ///
    pub(super) fn parse_identifier(&mut self) -> Expression {
        let token = self.eat_expect(TokenKind::Identifier);
        let ident = self.lexer.get_identifier(token).unwrap().to_string();

        Expression::Identifier { name: ident }
    }
}
