use std::fmt::Debug;
use std::iter;
use std::iter::from_fn;

#[allow(dead_code)]
#[derive(Debug)]
pub enum TokenKind {
    Identifier,
    StringLiteral,
    Number(i64),
    Float(f64), //TODO: Impl Float kind

    OpenParen,
    CloseParen,
    OpenBrack,
    CloseBrack,
    OpenCurly,
    CloseCurly,

    Colon,
    Semicolon,
    Pipe,
    Pound,
    Bang,
    Dot,
    Comma,
    Tilda,
    Question,
    LessThan,
    GreaterThan,

    Plus,
    Minus,
    Slash,
    Star,
    Equals,

    EOF
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Token {
    value: String,
    kind: TokenKind,
}

impl Token {
    pub fn new(value: &String, kind: TokenKind) -> Self {
        Token {
            value: value.to_string(),
            kind: kind
        }
    }
}

pub fn tokenize(input: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut iter = input.chars().peekable();

    while let Some(ch) = iter.next() {
        match ch {
            '(' => tokens.push(Token::new(&ch.to_string(), TokenKind::OpenParen)),
            ')' => tokens.push(Token::new(&ch.to_string(), TokenKind::CloseParen)),
            '[' => tokens.push(Token::new(&ch.to_string(), TokenKind::OpenBrack)),
            ']' => tokens.push(Token::new(&ch.to_string(), TokenKind::CloseBrack)),
            '{' => tokens.push(Token::new(&ch.to_string(), TokenKind::OpenCurly)),
            '}' => tokens.push(Token::new(&ch.to_string(), TokenKind::CloseCurly)),
            '+' => tokens.push(Token::new(&ch.to_string(), TokenKind::Plus)),
            '-' => tokens.push(Token::new(&ch.to_string(), TokenKind::Minus)),
            '*' => tokens.push(Token::new(&ch.to_string(), TokenKind::Star)),
            '/' => tokens.push(Token::new(&ch.to_string(), TokenKind::Slash)),
            '=' => tokens.push(Token::new(&ch.to_string(), TokenKind::Equals)),
            ':' => tokens.push(Token::new(&ch.to_string(), TokenKind::Colon)),
            ';' => tokens.push(Token::new(&ch.to_string(), TokenKind::Semicolon)),
            '|' => tokens.push(Token::new(&ch.to_string(), TokenKind::Pipe)),
            '#' => tokens.push(Token::new(&ch.to_string(), TokenKind::Pound)),
            '!' => tokens.push(Token::new(&ch.to_string(), TokenKind::Bang)),
            '?' => tokens.push(Token::new(&ch.to_string(), TokenKind::Question)),
            '.' => tokens.push(Token::new(&ch.to_string(), TokenKind::Dot)),
            ',' => tokens.push(Token::new(&ch.to_string(), TokenKind::Comma)),
            '~' => tokens.push(Token::new(&ch.to_string(), TokenKind::Tilda)),
            '<' => tokens.push(Token::new(&ch.to_string(), TokenKind::LessThan)),
            '>' => tokens.push(Token::new(&ch.to_string(), TokenKind::GreaterThan)),
            '1'..='9' => {
                let n: i64 = iter::once(ch)
                    .chain(from_fn(|| iter.by_ref().next_if(|s| s.is_ascii_digit())))
                    .collect::<String>()
                    .parse()
                    .unwrap();

                tokens.push(Token::new(&n.to_string(), TokenKind::Number(n)));
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let c: String = iter::once(ch)
                    .chain(from_fn(|| iter.by_ref().next_if(|s| s.is_ascii_alphanumeric() || s == &'_')))
                    .collect::<String>()
                    .parse()
                    .unwrap();

                tokens.push(Token::new(&c, TokenKind::Identifier))
            }
            '\'' | '"' => {
                let mut c: String = iter::once(ch)
                    .chain(from_fn(|| iter.by_ref().next_if(|s|(    
                        s.is_ascii_alphanumeric() || 
                        s == &'_' ||
                        s.is_whitespace()) && (s != &'\'' || 
                        s != &'"'
                    ))))
                    .collect::<String>()
                    .parse()
                    .unwrap();
                if c.len() != 1 {
                    c.push('"');
                    tokens.push(Token::new(&c, TokenKind::StringLiteral))
                }
            }
            ch if ch.is_whitespace() => continue,
            _ => {
                panic!("unrecognized char");
            }
        }
    }

    tokens.push(Token::new(&"EOF".to_string(), TokenKind::EOF));
    tokens
}
