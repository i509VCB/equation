use equation_lexer::{Kind, NumberKind, Token, TokenKind, Tokenizer};

macro_rules! token {
    ($kind: expr; $len: expr) => {{
        Token {
            kind: $kind,
            len: $len,
        }
    }};
}

#[test]
fn peek_two() {
    let mut tokenizer = Tokenizer::from("-2");
    assert_eq!(
        tokenizer.peek(2),
        Some(token!(TokenKind::Number(NumberKind::Decimal); 1))
    );
    assert_eq!(tokenizer.next(), Some(token!(Kind![-]; 1)));
    assert_eq!(tokenizer.peek(2), None);
    assert_eq!(
        tokenizer.next(),
        Some(token!(TokenKind::Number(NumberKind::Decimal); 1))
    );

    // All subsequent calls should return None.
    assert_eq!(tokenizer.peek(1), None);
    assert_eq!(tokenizer.peek(2), None);
    assert_eq!(tokenizer.peek(usize::MAX), None);
    assert_eq!(tokenizer.next(), None);
}
