use crate::{
    ast::{AssignOp, Statement},
    lexer::{Token, TokenKind},
    parser::Parser,
};

impl<'a, I> Parser<'a, I>
where
    I: Iterator<Item = Token>,
{
    ///
    pub(super) fn parse_asignment_semi(&mut self) -> Statement {
        let stmt = self.parse_asignment();
        self.eat_expect(TokenKind::Semicolon);

        stmt
    }

    ///
    pub(super) fn parse_asignment(&mut self) -> Statement {
        let dst = Self::parse_nested(self.lexer, &mut self.tokens).unwrap();

        let token = self.peek_token().unwrap();
        let kind = self.lexer.get_token_kind(token);

        let mut make_compound_assign = |dst, op| {
            self.eat_token();
            let src = self.parse_expression().unwrap();

            Statement::CompoundAssign {
                dst: Box::new(dst),
                op,
                src: Box::new(src),
            }
        };

        match kind {
            TokenKind::Equal => {
                self.eat_token();
                let src = self.parse_expression().unwrap();
                Statement::Assign {
                    dst: Box::new(dst),
                    src: Box::new(src),
                }
            }
            TokenKind::AmpEqual => make_compound_assign(dst, AssignOp::And),
            TokenKind::CaretEqual => make_compound_assign(dst, AssignOp::Xor),
            TokenKind::MinusEqual => make_compound_assign(dst, AssignOp::Sub),
            TokenKind::PercentEqual => make_compound_assign(dst, AssignOp::Mod),
            TokenKind::PipeEqual => make_compound_assign(dst, AssignOp::Or),
            TokenKind::PlusEqual => make_compound_assign(dst, AssignOp::Add),
            TokenKind::SlashEqual => make_compound_assign(dst, AssignOp::Div),
            TokenKind::StarEqual => make_compound_assign(dst, AssignOp::Mul),
            TokenKind::TildeEqual => make_compound_assign(dst, AssignOp::Not),
            _ => panic!("unexpected token"),
        }
    }
}
