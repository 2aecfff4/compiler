use std::iter::Peekable;

#[derive(Debug, Clone, Copy)]
pub struct Token(pub u32);

///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Keyword {
    Fn,
    Bool,
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
    F32,
    F64,
    For,
    While,
    If,
    Else,
    Break,
    Continue,
    Return,
    Struct,
    Let,
    In,
}

impl Keyword {
    fn from_ident(ident: &str) -> Option<Keyword> {
        Some(match ident {
            "fn" => Keyword::Fn,
            "bool" => Keyword::Bool,
            "u8" => Keyword::U8,
            "u16" => Keyword::U16,
            "u32" => Keyword::U32,
            "u64" => Keyword::U64,
            "i8" => Keyword::I8,
            "i16" => Keyword::I16,
            "i32" => Keyword::I32,
            "i64" => Keyword::I64,
            "f32" => Keyword::F32,
            "f64" => Keyword::F64,
            "for" => Keyword::For,
            "while" => Keyword::While,
            "if" => Keyword::If,
            "else" => Keyword::Else,
            "break" => Keyword::Break,
            "continue" => Keyword::Continue,
            "return" => Keyword::Return,
            "struct" => Keyword::Struct,
            "let" => Keyword::Let,
            "in" => Keyword::In,
            _ => return None,
        })
    }

    pub fn to_string(self) -> &'static str {
        match self {
            Keyword::Fn => "fn",
            Keyword::Bool => "bool",
            Keyword::U8 => "u8",
            Keyword::U16 => "u16",
            Keyword::U32 => "u32",
            Keyword::U64 => "u64",
            Keyword::I8 => "i8",
            Keyword::I16 => "i16",
            Keyword::I32 => "i32",
            Keyword::I64 => "i64",
            Keyword::F32 => "f32",
            Keyword::F64 => "f64",
            Keyword::For => "for",
            Keyword::While => "while",
            Keyword::If => "if",
            Keyword::Else => "else",
            Keyword::Break => "break",
            Keyword::Continue => "continue",
            Keyword::Return => "return",
            Keyword::Struct => "struct",
            Keyword::Let => "let",
            Keyword::In => "in",
        }
    }
}

///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    Identifier,
    Keyword(Keyword),
    Integer,

    Amp,
    Caret,
    Colon,
    CurlyBraceClose,
    CurlyBraceOpen,
    Dot,
    Equal,
    Exclaim,
    Greater,
    Less,
    Minus,
    ParenClose,
    ParenOpen,
    Percent,
    Pipe,
    Plus,
    Slash,
    Star,
    SquareClose,
    SquareOpen,
    Tilde,

    Comma,
    Semicolon,

    Arrow,
    AmpAmp,
    AmpEqual,
    CaretEqual,
    EqualEqual,
    ExclaimEqual,
    GreaterEqual,
    GreaterGreater,
    LessEqual,
    LessLess,
    MinusEqual,
    PercentEqual,
    PipeEqual,
    PipePipe,
    PlusEqual,
    SlashEqual,
    StarEqual,
    TildeEqual,
}

impl TokenKind {
    pub fn to_string(&self) -> Option<&str> {
        match self {
            TokenKind::Identifier => None,
            TokenKind::Keyword(..) => None,
            TokenKind::Integer => None,

            TokenKind::Amp => Some("&"),
            TokenKind::Caret => Some("^"),
            TokenKind::Colon => Some(":"),
            TokenKind::CurlyBraceClose => Some("}"),
            TokenKind::CurlyBraceOpen => Some("{"),
            TokenKind::Dot => Some("."),
            TokenKind::Equal => Some("="),
            TokenKind::Exclaim => Some("!"),
            TokenKind::Greater => Some(">"),
            TokenKind::Less => Some("<"),
            TokenKind::Minus => Some("-"),
            TokenKind::ParenClose => Some(")"),
            TokenKind::ParenOpen => Some("("),
            TokenKind::Percent => Some("%"),
            TokenKind::Pipe => Some("|"),
            TokenKind::Plus => Some("+"),
            TokenKind::Slash => Some("/"),
            TokenKind::Star => Some("*"),
            TokenKind::SquareClose => Some("]"),
            TokenKind::SquareOpen => Some("["),
            TokenKind::Tilde => Some("~"),

            TokenKind::Comma => Some(","),
            TokenKind::Semicolon => Some(";"),

            TokenKind::Arrow => Some("->"),
            TokenKind::AmpAmp => Some("&&"),
            TokenKind::AmpEqual => Some("&="),
            TokenKind::CaretEqual => Some("^="),
            TokenKind::EqualEqual => Some("=="),
            TokenKind::ExclaimEqual => Some("!="),
            TokenKind::GreaterEqual => Some(">="),
            TokenKind::GreaterGreater => Some(">>"),
            TokenKind::LessEqual => Some("<="),
            TokenKind::LessLess => Some("<<"),
            TokenKind::MinusEqual => Some("-="),
            TokenKind::PercentEqual => Some("%="),
            TokenKind::PipeEqual => Some("|="),
            TokenKind::PipePipe => Some("||"),
            TokenKind::PlusEqual => Some("+="),
            TokenKind::SlashEqual => Some("/="),
            TokenKind::StarEqual => Some("*="),
            TokenKind::TildeEqual => Some("~="),
        }
    }
}

///
#[derive(Debug, Clone, Copy)]
pub struct TokenData {
    range: (u32, u32),
    kind: TokenKind,
}

///
#[derive(Debug, Clone)]
pub struct Tokens<I>
where
    I: Iterator<Item = Token>,
{
    tokens: Peekable<I>,
}

impl<I> Tokens<I>
where
    I: Iterator<Item = Token>,
{
    ///
    pub fn eat_token(&mut self) -> Token {
        self.tokens.next().unwrap()
    }

    ///
    pub fn peek_token(&mut self) -> Option<Token> {
        self.tokens.peek().cloned()
    }
}

///
#[derive(Debug, Clone)]
pub struct Lexer<'a> {
    source: &'a str,
    tokens: Vec<TokenData>,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            tokens: Vec::new(),
        }
    }
    ///
    pub fn lex(&mut self) -> Tokens<impl Iterator<Item = Token>> {
        // #NOTE: This is a very simple lexer without any optimizations

        let mut indices = self.source.char_indices().peekable();
        while let Some((i, c)) = indices.next() {
            let start = i;
            let mut end = i;
            if unicode_ident::is_xid_start(c) {
                while let Some((i, c)) = indices.peek() {
                    if unicode_ident::is_xid_continue(*c) {
                        indices.next();
                    } else {
                        end = *i;
                        break;
                    }
                }

                let identifier = &self.source[start..end];
                if let Some(keyword) = Keyword::from_ident(identifier) {
                    self.tokens.push(TokenData {
                        range: (start as u32, end as u32),
                        kind: TokenKind::Keyword(keyword),
                    });
                } else {
                    self.tokens.push(TokenData {
                        range: (start as u32, end as u32),
                        kind: TokenKind::Identifier,
                    });
                }
            } else {
                let start = i as u32;

                macro_rules! impl_match {
                    ($on_fail: expr, $(($to_match: literal, $on_success: expr)),*) => {
                        let token = match indices.peek() {
                            $(
                                Some((_, $to_match)) => {
                                    indices.next();
                                    TokenData {
                                        range: (start, start + 2),
                                        kind: $on_success,
                                    }
                                }
                            )*
                            _ => {
                                TokenData {
                                    range: (start, start + 1),
                                    kind: $on_fail,
                                }
                            }
                        };
                        self.tokens.push(token);
                    };
                }

                match c {
                    '&' => {
                        impl_match!(
                            TokenKind::Amp,
                            ('=', TokenKind::AmpEqual),
                            ('&', TokenKind::AmpAmp)
                        );
                    }
                    '^' => {
                        impl_match!(TokenKind::Caret, ('=', TokenKind::CaretEqual));
                    }
                    ':' => {
                        self.tokens.push(TokenData {
                            range: (start, start + 1),
                            kind: TokenKind::Colon,
                        });
                    }
                    '}' => {
                        self.tokens.push(TokenData {
                            range: (start, start + 1),
                            kind: TokenKind::CurlyBraceClose,
                        });
                    }
                    '{' => {
                        self.tokens.push(TokenData {
                            range: (start, start + 1),
                            kind: TokenKind::CurlyBraceOpen,
                        });
                    }
                    '.' => {
                        self.tokens.push(TokenData {
                            range: (start, start + 1),
                            kind: TokenKind::Dot,
                        });
                    }
                    '=' => {
                        impl_match!(TokenKind::Equal, ('=', TokenKind::EqualEqual));
                    }
                    '!' => {
                        impl_match!(TokenKind::Exclaim, ('=', TokenKind::ExclaimEqual));
                    }
                    '>' => {
                        impl_match!(
                            TokenKind::Greater,
                            ('=', TokenKind::GreaterEqual),
                            ('>', TokenKind::GreaterGreater)
                        );
                    }
                    '<' => {
                        impl_match!(
                            TokenKind::Less,
                            ('=', TokenKind::LessEqual),
                            ('<', TokenKind::LessLess)
                        );
                    }
                    '-' => {
                        impl_match!(
                            TokenKind::Minus,
                            ('=', TokenKind::MinusEqual),
                            ('>', TokenKind::Arrow)
                        );
                    }
                    ')' => {
                        self.tokens.push(TokenData {
                            range: (start, start + 1),
                            kind: TokenKind::ParenClose,
                        });
                    }
                    '(' => {
                        self.tokens.push(TokenData {
                            range: (start, start + 1),
                            kind: TokenKind::ParenOpen,
                        });
                    }
                    '%' => {
                        impl_match!(TokenKind::Percent, ('=', TokenKind::PercentEqual));
                    }
                    '|' => {
                        impl_match!(
                            TokenKind::Pipe,
                            ('=', TokenKind::PipeEqual),
                            ('|', TokenKind::PipePipe)
                        );
                    }
                    '+' => {
                        impl_match!(TokenKind::Plus, ('=', TokenKind::PlusEqual));
                    }
                    '/' => {
                        impl_match!(TokenKind::Slash, ('=', TokenKind::SlashEqual));
                        // #TODO: Comments?
                    }
                    '*' => {
                        impl_match!(TokenKind::Star, ('=', TokenKind::StarEqual));
                    }
                    ']' => {
                        self.tokens.push(TokenData {
                            range: (start, start + 1),
                            kind: TokenKind::SquareClose,
                        });
                    }
                    '[' => {
                        self.tokens.push(TokenData {
                            range: (start, start + 1),
                            kind: TokenKind::SquareOpen,
                        });
                    }
                    '~' => {
                        impl_match!(TokenKind::Tilde, ('=', TokenKind::TildeEqual));
                    }
                    ',' => {
                        self.tokens.push(TokenData {
                            range: (start, start + 1),
                            kind: TokenKind::Comma,
                        });
                    }
                    ';' => {
                        self.tokens.push(TokenData {
                            range: (start, start + 1),
                            kind: TokenKind::Semicolon,
                        });
                    }
                    '\n' | ' ' => continue,
                    other => {
                        if other.is_numeric() {
                            let mut end = start;
                            while let Some((i, c)) = indices.peek() {
                                if c.is_numeric() {
                                    indices.next();
                                } else {
                                    end = *i as u32;
                                    break;
                                }
                            }

                            self.tokens.push(TokenData {
                                range: (start, end),
                                kind: TokenKind::Integer,
                            });
                        } else {
                            todo!()
                        }
                    }
                }
            }
        }

        let len = self.tokens.len() as u32;
        Tokens {
            tokens: (0..len).map(Token).peekable(),
        }
    }

    ///
    fn get_token_data(&self, token: Token) -> &TokenData {
        self.tokens.get(token.0 as usize).unwrap()
    }

    ///
    pub fn get_identifier(&self, token: Token) -> Option<&str> {
        let token_data = self.get_token_data(token);
        if token_data.kind == TokenKind::Identifier {
            let start = token_data.range.0 as usize;
            let end = token_data.range.1 as usize;

            Some(&self.source[start..end])
        } else {
            None
        }
    }

    ///
    pub fn get_integer(&self, token: Token) -> Option<u64> {
        let token_data = self.get_token_data(token);
        if token_data.kind == TokenKind::Integer {
            let start = token_data.range.0 as usize;
            let end = token_data.range.1 as usize;
            let str = &self.source[start..end];
            Some(str.parse().unwrap())
        } else {
            None
        }
    }

    ///
    pub fn get_keyword(&self, token: Token) -> Option<Keyword> {
        let token_data: &TokenData = self.get_token_data(token);
        None
    }

    ///
    pub fn get_token_kind(&self, token: Token) -> TokenKind {
        let token_data = self.get_token_data(token);
        token_data.kind
    }
}
