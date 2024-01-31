use crate::{
    ast::{Builtin, Ty},
    lexer::{Keyword, Token, TokenKind},
    parser::Parser,
};

impl<'a, I> Parser<'a, I>
where
    I: Iterator<Item = Token>,
{
    ///
    pub(super) fn parse_type(&mut self) -> Ty {
        if let Some(ty) = self.parse_builtin_type() {
            return ty;
        }

        if let Some(ty) = self.parse_named_type() {
            return ty;
        }

        if let Some(ty) = self.parse_struct() {
            return ty;
        }

        panic!("unexpected token")
    }

    ///
    fn parse_builtin_type(&mut self) -> Option<Ty> {
        let token = self.peek_token().unwrap();
        let kind = self.lexer.get_token_kind(token);
        let ty = match kind {
            TokenKind::Keyword(keyword) => match keyword {
                Keyword::Bool => Some(Ty::Builtin(Builtin::Bool)),
                Keyword::U8 => Some(Ty::Builtin(Builtin::U8)),
                Keyword::U16 => Some(Ty::Builtin(Builtin::U16)),
                Keyword::U32 => Some(Ty::Builtin(Builtin::U32)),
                Keyword::U64 => Some(Ty::Builtin(Builtin::U64)),
                Keyword::I8 => Some(Ty::Builtin(Builtin::I8)),
                Keyword::I16 => Some(Ty::Builtin(Builtin::I16)),
                Keyword::I32 => Some(Ty::Builtin(Builtin::I32)),
                Keyword::I64 => Some(Ty::Builtin(Builtin::I64)),
                Keyword::F32 => Some(Ty::Builtin(Builtin::F32)),
                Keyword::F64 => Some(Ty::Builtin(Builtin::F64)),
                _ => None,
            },
            _ => None,
        };

        if let Some(ty) = ty {
            self.eat_token();
            Some(ty)
        } else {
            None
        }
    }

    ///
    fn parse_named_type(&mut self) -> Option<Ty> {
        let token = self.eat_token();
        let kind = self.lexer.get_token_kind(token);
        match kind {
            TokenKind::Identifier => {
                let ident = self.lexer.get_identifier(token).unwrap();
                Some(Ty::NamedType {
                    name: ident.to_string(),
                })
            }
            _ => None,
        }
    }

    ///
    fn parse_struct(&mut self) -> Option<Ty> {
        todo!()
    }
}
