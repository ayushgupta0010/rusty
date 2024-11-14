use std::{borrow::Cow, fmt};

use miette::{Error, LabeledSpan};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Token<'de> {
    origin: &'de str,
    kind: TokenKind,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenKind {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Star,
    Bang,
    BangEqual,
    EqualEqual,
    LessEqual,
    GreaterEqual,
    Less,
    Greater,
    Slash,
    Equal,
    String,
    Ident,
    Number(f64),
    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Return,
    Super,
    This,
    True,
    Var,
    While,
}

impl fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let origin = self.origin;
        match self.kind {
            TokenKind::LeftParen => write!(f, "LEFT_PAREN"),
            TokenKind::RightParen => write!(f, "RIGHT_PAREN"),
            TokenKind::LeftBrace => write!(f, "LEFT_BRACE"),
            TokenKind::RightBrace => write!(f, "RIGHT_BRACE"),
            TokenKind::Comma => write!(f, "COMMA"),
            TokenKind::Dot => write!(f, "DOT"),
            TokenKind::Minus => write!(f, "MINUS"),
            TokenKind::Plus => write!(f, "PLUS"),
            TokenKind::Semicolon => write!(f, "SEMICOLON"),
            TokenKind::Star => write!(f, "STAR"),
            TokenKind::BangEqual => write!(f, "BANG_EQUAL"),
            TokenKind::EqualEqual => write!(f, "EQUAL_EQUAL"),
            TokenKind::LessEqual => write!(f, "LESS_EQUAL"),
            TokenKind::GreaterEqual => write!(f, "GREATER_EQUAL"),
            TokenKind::Less => write!(f, "LESS"),
            TokenKind::Greater => write!(f, "GREATER"),
            TokenKind::Slash => write!(f, "SLASH"),
            TokenKind::Bang => write!(f, "BANG"),
            TokenKind::Equal => write!(f, "EQUAL"),
            TokenKind::String => write!(f, "STRING {origin} {}", Token::unescape(origin)),
            TokenKind::Ident => write!(f, "IDENTFIER {origin}"),
            TokenKind::Number(n) => write!(f, "NUMBER {n}"),
            TokenKind::And => write!(f, "AND"),
            TokenKind::Class => write!(f, "CLASS"),
            TokenKind::Else => write!(f, "ELSE"),
            TokenKind::False => write!(f, "FALSE"),
            TokenKind::For => write!(f, "FOR"),
            TokenKind::Fun => write!(f, "FUN"),
            TokenKind::If => write!(f, "IF"),
            TokenKind::Nil => write!(f, "NIL"),
            TokenKind::Or => write!(f, "OR"),
            TokenKind::Return => write!(f, "RETURN"),
            TokenKind::Super => write!(f, "SUPER"),
            TokenKind::This => write!(f, "THIS"),
            TokenKind::True => write!(f, "TRUE"),
            TokenKind::Var => write!(f, "VAR"),
            TokenKind::While => write!(f, "WHILE"),
        }
    }
}

impl Token<'_> {
    pub fn unescape<'de>(s: &'de str) -> Cow<'de, str> {
        todo!()
    }
}

pub struct Lexer<'de> {
    whole: &'de str,
    rest: &'de str,
    byte: usize,
}

impl<'de> Lexer<'de> {
    pub fn new(input: &'de str) -> Self {
        Self {
            whole: input,
            rest: input,
            byte: 0,
        }
    }
}

impl<'de> Iterator for Lexer<'de> {
    type Item = Result<Token<'de>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let mut chars = self.rest.chars();
            let c = chars.next()?;
            let c_str = &self.rest[..c.len_utf8()];
            let c_onwards = self.rest;
            self.rest = chars.as_str();
            self.byte += c.len_utf8();

            enum Started {
                String,
                Number,
                Ident,
                IfEqualElse(TokenKind, TokenKind),
            }

            let just = move |kind: TokenKind| {
                Some(Ok(Token {
                    kind,
                    origin: c_str,
                }))
            };

            let started = match c {
                '(' => return just(TokenKind::LeftParen),
                ')' => return just(TokenKind::RightParen),
                '{' => return just(TokenKind::LeftBrace),
                '}' => return just(TokenKind::RightBrace),
                ',' => return just(TokenKind::Comma),
                '.' => return just(TokenKind::Dot),
                '-' => return just(TokenKind::Minus),
                '+' => return just(TokenKind::Plus),
                ';' => return just(TokenKind::Semicolon),
                '*' => return just(TokenKind::Star),
                '/' => return just(TokenKind::Slash),
                '<' => Started::IfEqualElse(TokenKind::LessEqual, TokenKind::Less),
                '>' => Started::IfEqualElse(TokenKind::GreaterEqual, TokenKind::Greater),
                '!' => Started::IfEqualElse(TokenKind::BangEqual, TokenKind::Bang),
                '=' => Started::IfEqualElse(TokenKind::EqualEqual, TokenKind::Equal),
                '"' => Started::String,
                c if c.is_whitespace() => continue,
                '0'..='9' => Started::Number,
                'a'..='z' | '_' => Started::Ident,
                c => {
                    return Some(Err(miette::miette!(
                        labels = vec![LabeledSpan::at(
                            self.byte - c.len_utf8()..self.byte,
                            "this character"
                        )],
                        "Unexpected token '{c}' in input"
                    )
                    .with_source_code(self.whole.to_string())))
                }
            };

            break match started {
                Started::String => todo!(),
                Started::Number => todo!(),
                Started::Ident => todo!(),
                Started::IfEqualElse(yes, no) => {
                    self.rest = self.rest.trim_start();
                    let trimmed = c_onwards.len() - self.rest.len() - 1;
                    self.byte += trimmed;
                    if self.rest.starts_with('=') {
                        let span = &c_onwards[..c.len_utf8() + trimmed + 1];
                        self.rest = &self.rest[1..];
                        self.byte += 1;
                        Some(Ok(Token {
                            origin: span,
                            kind: yes,
                        }))
                    } else {
                        Some(Ok(Token {
                            origin: c_str,
                            kind: no,
                        }))
                    }
                }
            };
        }
    }
}