//! Single character long token tests.

use equation_lexer::{Brace, Kind, Token, Tokenizer};

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

generate_test!(plus: "+", token!(Kind![+]; 1));
generate_test!(minus: "-", token!(Kind![-]; 1));
generate_test!(multiply: "*", token!(Kind![*]; 1));
generate_test!(div: "/", token!(Kind![/]; 1));
generate_test!(modulo: "%", token!(Kind![%]; 1));
generate_test!(eq: "=", token!(Kind![=]; 1));
generate_test!(ge: ">", token!(Kind![>]; 1));
generate_test!(le: "<", token!(Kind![<]; 1));
generate_test!(amp: "&", token!(Kind![&]; 1));
generate_test!(caret: "^", token!(Kind![^]; 1));
generate_test!(exclamation: "!", token!(Kind![!]; 1));
generate_test!(comma: ",", token!(Kind![,]; 1));
generate_test!(open_round_brace: "(", token!(Brace![Round, Open]; 1));
generate_test!(close_round_brace: ")", token!(Brace![Round, Close]; 1));
generate_test!(open_square_brace: "[", token!(Brace![Square, Open]; 1));
generate_test!(close_square_brace: "]", token!(Brace![Square, Close]; 1));
generate_test!(open_curly_brace: "{", token!(Brace![Curly, Open]; 1));
generate_test!(close_curly_brace: "}", token!(Brace![Curly, Close]; 1));
