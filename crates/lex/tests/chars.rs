//! Character tokens

use equation_lexer::{Token, TokenKind, Tokenizer};

macro_rules! token {
    ($kind: expr; $len: expr) => {{
        Token {
            kind: $kind,
            len: $len,
        }
    }};
}

#[test]
fn pi() {
    let mut reader = Tokenizer::from("pi");

    assert_eq!(reader.next(), Some(token!(TokenKind::Chars; 2)));
    assert_eq!(reader.next(), None);
}

#[test]
fn log2() {
    let mut reader = Tokenizer::from("log2");

    assert_eq!(reader.next(), Some(token!(TokenKind::Chars; 4)));
    assert_eq!(reader.next(), None);
}

#[test]
fn space_separated_words() {
    let mut reader = Tokenizer::from("foo bar");

    assert_eq!(reader.next(), Some(token!(TokenKind::Chars; 3)));
    assert_eq!(reader.next(), Some(token!(TokenKind::Ws; 1)));
    assert_eq!(reader.next(), Some(token!(TokenKind::Chars; 3)));
}
