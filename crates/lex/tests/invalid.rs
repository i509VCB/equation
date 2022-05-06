//! Invalid tokens

use equation_lexer::{NumberKind, Token, TokenKind, Tokenizer};

macro_rules! token {
    ($kind: expr; $len: expr) => {{
        Token {
            kind: $kind,
            len: $len,
        }
    }};
}

macro_rules! generate_test {
    ($name: ident: $input: expr, $expected: expr) => {
        #[test]
        fn $name() {
            const INPUT: &str = $input;

            let mut parser = Tokenizer::from(INPUT);
            assert_eq!(parser.next(), Some($expected));
            // All tests use a single token, so the next fetch should return None.
            assert_eq!(parser.next(), None);
        }
    };
}

#[test]
fn peek_zero() {
    let tokenizer = Tokenizer::from("3.0");
    assert_eq!(None, tokenizer.peek(0));
}

#[test]
fn intersperse_invalid() {
    let mut tokenizer = Tokenizer::from("3`44`55`6");
    assert_eq!(
        tokenizer.next(),
        Some(token!(TokenKind::Number(NumberKind::Decimal); 1))
    );
    assert_eq!(tokenizer.next(), Some(token!(TokenKind::Invalid; 1)));
    assert_eq!(
        tokenizer.next(),
        Some(token!(TokenKind::Number(NumberKind::Decimal); 2))
    );
    assert_eq!(tokenizer.next(), Some(token!(TokenKind::Invalid; 1)));
    assert_eq!(
        tokenizer.next(),
        Some(token!(TokenKind::Number(NumberKind::Decimal); 2))
    );
    assert_eq!(tokenizer.next(), Some(token!(TokenKind::Invalid; 1)));
    assert_eq!(
        tokenizer.next(),
        Some(token!(TokenKind::Number(NumberKind::Decimal); 1))
    );
    assert_eq!(tokenizer.next(), None);
}

generate_test!(hash: "#", token!(TokenKind::Invalid; 1));
generate_test!(many_hash: "######################################", token!(TokenKind::Invalid; 38));
generate_test!(quotes: "'''''''", token!(TokenKind::Invalid; 7));
generate_test!(tilde: "~", token!(TokenKind::Invalid; 1));
generate_test!(dollar: "$", token!(TokenKind::Invalid; 1));
// カニ is crab in Japanese
generate_test!(kani: "カニ", token!(TokenKind::Invalid; 2));
generate_test!(zero_width_joiner: "\u{200D}", token!(TokenKind::Invalid; 1));
generate_test!(upside_down_face: "🙃", token!(TokenKind::Invalid; 1));
