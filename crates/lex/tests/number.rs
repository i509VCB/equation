//! Number tokens

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

generate_test!(zero_only: "0", token!(TokenKind::Number(NumberKind::Decimal); 1));
generate_test!(two_zeroes_only: "00", token!(TokenKind::Number(NumberKind::Decimal); 2));
generate_test!(zero_point_zero: "0.0", token!(TokenKind::Number(NumberKind::Decimal); 3));

// Non-zero numbers
generate_test!(one_only: "1", token!(TokenKind::Number(NumberKind::Decimal); 1));
generate_test!(five_hundred_nine: "509", token!(TokenKind::Number(NumberKind::Decimal); 3));
generate_test!(four_twenty_point_sixty_nine: "420.69", token!(TokenKind::Number(NumberKind::Decimal); 6));

// Complex numbers

// Euler's number is parsed as some characters.
generate_test!(euler: "e", token!(TokenKind::Chars; 1));
generate_test!(zero_e_one: "0e1", token!(TokenKind::Number(NumberKind::Decimal); 3));
generate_test!(zero_point_zero_e_one: "0.0e1", token!(TokenKind::Number(NumberKind::Decimal); 5));
generate_test!(five_e_five: "5e5", token!(TokenKind::Number(NumberKind::Decimal); 3));
generate_test!(five_point_three_four_e_five: "5.34e5", token!(TokenKind::Number(NumberKind::Decimal); 6));

// Hexadecimal

// TODO
generate_test!(hexadecimal_zero: "0x0000", token!(TokenKind::Number(NumberKind::Hexadecimal); 6));
generate_test!(hexadecimal_alphas: "0xabcdefABCDEF", token!(TokenKind::Number(NumberKind::Hexadecimal); 14));
generate_test!(hexadecimal_no_value: "0x", token!(TokenKind::Number(NumberKind::Hexadecimal); 2));

// Binary
generate_test!(binary_zero: "0b0000", token!(TokenKind::Number(NumberKind::Binary); 6));
generate_test!(binary_no_bits: "0b", token!(TokenKind::Number(NumberKind::Binary); 2));
generate_test!(binary_16: "0b1111", token!(TokenKind::Number(NumberKind::Binary); 6));
generate_test!(invalid_binary: "0b5344", token!(TokenKind::Number(NumberKind::Binary); 6));
