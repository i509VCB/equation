//! Whitespace tokens

use equation_lexer::{Token, TokenKind, Tokenizer};

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

generate_test!(one_space: " ", token!(TokenKind::Ws; 1));
generate_test!(two_spaces: "  ", token!(TokenKind::Ws; 2));
generate_test!(three_spaces: "   ", token!(TokenKind::Ws; 3));
generate_test!(one_tab: "\t", token!(TokenKind::Ws; 1));
generate_test!(two_tabs: "\t\t", token!(TokenKind::Ws; 2));
generate_test!(one_tab_one_space: "\t ", token!(TokenKind::Ws; 2));
generate_test!(one_tab_one_space_twice: "\t \t ", token!(TokenKind::Ws; 4));
