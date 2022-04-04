use equation_lexer::{Brace, Kind, NumberKind, Token, TokenKind, Tokenizer};

macro_rules! token {
    ($kind: expr; $len: expr) => {{
        Token {
            kind: $kind,
            len: $len,
        }
    }};
}

#[test]
fn log2_64() {
    let mut tokenizer = Tokenizer::from("log2(64)");
    assert_eq!(tokenizer.next(), Some(token!(TokenKind::Chars; 4)));
    assert_eq!(tokenizer.next(), Some(token!(Brace![Round, Open]; 1)));
    assert_eq!(
        tokenizer.next(),
        Some(token!(TokenKind::Number(NumberKind::Decimal); 2))
    );
    assert_eq!(tokenizer.next(), Some(token!(Brace![Round, Close]; 1)));
    assert_eq!(tokenizer.next(), None);
}

#[test]
fn div_arcsin_pi() {
    let mut tokenizer = Tokenizer::from("22/arcsin(pi)");
    assert_eq!(
        tokenizer.next(),
        Some(token!(TokenKind::Number(NumberKind::Decimal); 2))
    );
    assert_eq!(tokenizer.next(), Some(token!(Kind![/]; 1)));
    assert_eq!(tokenizer.next(), Some(token!(TokenKind::Chars; 6)));
    assert_eq!(tokenizer.next(), Some(token!(Brace![Round, Open]; 1)));
    assert_eq!(tokenizer.next(), Some(token!(TokenKind::Chars; 2)));
    assert_eq!(tokenizer.next(), Some(token!(Brace![Round, Close]; 1)));
    assert_eq!(tokenizer.next(), None);
}
