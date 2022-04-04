use equation_lexer::{NumberKind, Token, TokenKind, Tokenizer};

macro_rules! token {
    ($kind: expr; $len: expr) => {{
        Token {
            kind: $kind,
            len: $len,
        }
    }};
}

#[test]
fn too_many_decimal_places() {
    let mut tokenizer = Tokenizer::from("2.......................................................");
    assert_eq!(
        tokenizer.next(),
        Some(token!(TokenKind::Number(NumberKind::Decimal); 56))
    );
    assert_eq!(tokenizer.next(), None);
}

#[test]
fn one_million_dollars() {
    let mut tokenizer = Tokenizer::from("$1000000");
    assert_eq!(tokenizer.next(), Some(token!(TokenKind::Invalid; 1)));
    assert_eq!(
        tokenizer.next(),
        Some(token!(TokenKind::Number(NumberKind::Decimal); 7))
    );
    assert_eq!(tokenizer.next(), None);
}

#[test]
fn sentence() {
    let mut tokenizer = Tokenizer::from("I'd just like to interject for a moment.");

    assert_eq!(tokenizer.next(), Some(token!(TokenKind::Chars; 1)));
    assert_eq!(tokenizer.next(), Some(token!(TokenKind::Invalid; 1)));
    assert_eq!(tokenizer.next(), Some(token!(TokenKind::Chars; 1)));
    assert_eq!(tokenizer.next(), Some(token!(TokenKind::Ws; 1)));
    assert_eq!(tokenizer.next(), Some(token!(TokenKind::Chars; 4)));
    assert_eq!(tokenizer.next(), Some(token!(TokenKind::Ws; 1)));
    assert_eq!(tokenizer.next(), Some(token!(TokenKind::Chars; 4)));
    assert_eq!(tokenizer.next(), Some(token!(TokenKind::Ws; 1)));
    assert_eq!(tokenizer.next(), Some(token!(TokenKind::Chars; 2)));
    assert_eq!(tokenizer.next(), Some(token!(TokenKind::Ws; 1)));
    assert_eq!(tokenizer.next(), Some(token!(TokenKind::Chars; 9)));
    assert_eq!(tokenizer.next(), Some(token!(TokenKind::Ws; 1)));
    assert_eq!(tokenizer.next(), Some(token!(TokenKind::Chars; 3)));
    assert_eq!(tokenizer.next(), Some(token!(TokenKind::Ws; 1)));
    assert_eq!(tokenizer.next(), Some(token!(TokenKind::Chars; 1)));
    assert_eq!(tokenizer.next(), Some(token!(TokenKind::Ws; 1)));
    assert_eq!(tokenizer.next(), Some(token!(TokenKind::Chars; 6)));
    assert_eq!(tokenizer.next(), Some(token!(TokenKind::Invalid; 1)));
    assert_eq!(tokenizer.next(), None);
}
