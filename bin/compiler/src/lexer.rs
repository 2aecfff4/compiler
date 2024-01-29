#[derive(Debug, Clone, Copy)]
pub struct Token(pub u32);

///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Keyword {
    Fn,
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
}

impl Keyword {
    fn from_ident(ident: &str) -> Option<Keyword> {
        Some(match ident {
            "fn" => Keyword::Fn,
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
            _ => return None,
        })
    }

    pub fn to_string(self) -> &'static str {
        match self {
            Keyword::Fn => "fn",
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
        }
    }
}

macro_rules! tokens_impl {
    {$({ $name: ident, $str: expr }),*} => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum TokenKind {
            $(
                $name,
            )*
        }

        impl TokenKind {
            pub fn to_string(&self) -> Option<&str> {
                match self {
                    $(
                        TokenKind::$name => $str,
                    )*
                }
            }
        }
    };
}

tokens_impl! {
    { Identifier, None },
    { Keyword, None },

    { Amp,                 Some("&") },
    { Caret,               Some("^") },
    { Colon,               Some(":") },
    { CurlyBraceClose,     Some("}") },
    { CurlyBraceOpen,      Some("{") },
    { Equal,               Some("=") },
    { Exclaim,             Some("!") },
    { Greater,             Some(">") },
    { Less,                Some("<") },
    { Minus,               Some("-") },
    { ParenClose,          Some(")") },
    { ParenOpen,           Some("(") },
    { Percent,             Some("%") },
    { Period,              Some(".") },
    { Pipe,                Some("|") },
    { Plus,                Some("+") },
    { Slash,               Some("/") },
    { Star,                Some("*") },
    { Tilde,               Some("~") },

    { Comma,      Some(",") },
    { Semicolon,  Some(";") },

    { Arrow,               Some("->") },
    { AmpEqual,            Some("&=") },
    { CaretEqual,          Some("^=") },
    { ColonEqual,          Some(":=") },
    { EqualEqual,          Some("==") },
    { ExclaimEqual,        Some("!=") },
    { GreaterEqual,        Some(">=") },
    { LessEqual,           Some("<=") },
    { MinusEqual,          Some("-=") },
    { PercentEqual,        Some("%=") },
    { PipeEqual,           Some("|=") },
    { PlusEqual,           Some("+=") },
    { SlashEqual,          Some("/=") },
    { StarEqual,           Some("*=") },
    { TildeEqual,          Some("~=") }
}

///
#[derive(Debug, Clone, Copy)]
pub struct TokenData {
    range: (u32, u32),
    kind: TokenKind,
}

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
    pub fn lex(&mut self) -> impl Iterator<Item = Token> {
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
                if let Some(ident) = Keyword::from_ident(identifier) {
                    self.tokens.push(TokenData {
                        range: (start as u32, end as u32),
                        kind: TokenKind::Keyword,
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
                        impl_match!(TokenKind::Amp, ('=', TokenKind::AmpEqual));
                    }
                    '^' => {
                        impl_match!(TokenKind::Caret, ('=', TokenKind::CaretEqual));
                    }
                    ':' => {
                        impl_match!(TokenKind::Colon, ('=', TokenKind::ColonEqual));
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
                    '=' => {
                        impl_match!(TokenKind::Equal, ('=', TokenKind::EqualEqual));
                    }
                    '!' => {
                        impl_match!(TokenKind::Exclaim, ('=', TokenKind::ExclaimEqual));
                    }
                    '>' => {
                        impl_match!(TokenKind::Greater, ('=', TokenKind::GreaterEqual));
                    }
                    '<' => {
                        impl_match!(TokenKind::Less, ('=', TokenKind::LessEqual));
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
                    '.' => {
                        self.tokens.push(TokenData {
                            range: (start, start + 1),
                            kind: TokenKind::Period,
                        });
                    }
                    '|' => {
                        impl_match!(TokenKind::Pipe, ('=', TokenKind::PipeEqual));
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
                    other => {
                        if other.is_numeric() {
                            // #TODO
                            todo!()
                        }
                    }
                }
            }
        }

        let len = self.tokens.len() as u32;
        (0..len).map(Token).peekable()
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
