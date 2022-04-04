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
fn add_two_numbers() {
    let mut tokenizer = Tokenizer::from("2+2");
    assert_eq!(
        tokenizer.next(),
        Some(token!(TokenKind::Number(NumberKind::Decimal); 1))
    );
    assert_eq!(tokenizer.next(), Some(token!(Kind![+]; 1)));
    assert_eq!(
        tokenizer.next(),
        Some(token!(TokenKind::Number(NumberKind::Decimal); 1))
    );
    assert_eq!(tokenizer.next(), None);
}

#[test]
fn add_two_numbers_with_ws() {
    let mut tokenizer = Tokenizer::from("2 + 2");
    assert_eq!(
        tokenizer.next(),
        Some(token!(TokenKind::Number(NumberKind::Decimal); 1))
    );
    assert_eq!(tokenizer.next(), Some(token!(TokenKind::Ws; 1)));
    assert_eq!(tokenizer.next(), Some(token!(Kind![+]; 1)));
    assert_eq!(tokenizer.next(), Some(token!(TokenKind::Ws; 1)));
    assert_eq!(
        tokenizer.next(),
        Some(token!(TokenKind::Number(NumberKind::Decimal); 1))
    );
    assert_eq!(tokenizer.next(), None);
}

#[test]
fn add_one_positive_one_negative_with_ws() {
    let mut tokenizer = Tokenizer::from("2 + -2");
    assert_eq!(
        tokenizer.next(),
        Some(token!(TokenKind::Number(NumberKind::Decimal); 1))
    );
    assert_eq!(tokenizer.next(), Some(token!(TokenKind::Ws; 1)));
    assert_eq!(tokenizer.next(), Some(token!(Kind![+]; 1)));
    assert_eq!(tokenizer.next(), Some(token!(TokenKind::Ws; 1)));
    assert_eq!(tokenizer.next(), Some(token!(Kind![-]; 1)));
    assert_eq!(
        tokenizer.next(),
        Some(token!(TokenKind::Number(NumberKind::Decimal); 1))
    );
    assert_eq!(tokenizer.next(), None);
}

#[test]
fn mul_two_numbers() {
    let mut tokenizer = Tokenizer::from("34*0");
    assert_eq!(
        tokenizer.next(),
        Some(token!(TokenKind::Number(NumberKind::Decimal); 2))
    );
    assert_eq!(tokenizer.next(), Some(token!(Kind![*]; 1)));
    assert_eq!(
        tokenizer.next(),
        Some(token!(TokenKind::Number(NumberKind::Decimal); 1))
    );
    assert_eq!(tokenizer.next(), None);
}

#[test]
fn div_two_numbers() {
    let mut tokenizer = Tokenizer::from("64/2");
    assert_eq!(
        tokenizer.next(),
        Some(token!(TokenKind::Number(NumberKind::Decimal); 2))
    );
    assert_eq!(tokenizer.next(), Some(token!(Kind![/]; 1)));
    assert_eq!(
        tokenizer.next(),
        Some(token!(TokenKind::Number(NumberKind::Decimal); 1))
    );
    assert_eq!(tokenizer.next(), None);
}

#[test]
fn mod_two_numbers() {
    let mut tokenizer = Tokenizer::from("28%3");
    assert_eq!(
        tokenizer.next(),
        Some(token!(TokenKind::Number(NumberKind::Decimal); 2))
    );
    assert_eq!(tokenizer.next(), Some(token!(Kind![%]; 1)));
    assert_eq!(
        tokenizer.next(),
        Some(token!(TokenKind::Number(NumberKind::Decimal); 1))
    );
    assert_eq!(tokenizer.next(), None);
}

#[test]
fn three_factorial() {
    let mut tokenizer = Tokenizer::from("3!");
    assert_eq!(
        tokenizer.next(),
        Some(token!(TokenKind::Number(NumberKind::Decimal); 1))
    );
    assert_eq!(tokenizer.next(), Some(token!(Kind![!]; 1)));
    assert_eq!(tokenizer.next(), None);
}

#[test]
fn mul_and_add() {
    let mut tokenizer = Tokenizer::from("(4*2) + 1");
    assert_eq!(tokenizer.next(), Some(token!(Brace![Round, Open]; 1)));
    assert_eq!(
        tokenizer.next(),
        Some(token!(TokenKind::Number(NumberKind::Decimal); 1))
    );
    assert_eq!(tokenizer.next(), Some(token!(Kind![*]; 1)));
    assert_eq!(
        tokenizer.next(),
        Some(token!(TokenKind::Number(NumberKind::Decimal); 1))
    );
    assert_eq!(tokenizer.next(), Some(token!(Brace![Round, Close]; 1)));
    assert_eq!(tokenizer.next(), Some(token!(TokenKind::Ws; 1)));
    assert_eq!(tokenizer.next(), Some(token!(Kind![+]; 1)));
    assert_eq!(tokenizer.next(), Some(token!(TokenKind::Ws; 1)));
    assert_eq!(
        tokenizer.next(),
        Some(token!(TokenKind::Number(NumberKind::Decimal); 1))
    );
    assert_eq!(tokenizer.next(), None);
}

#[test]
fn false_equation_eq() {
    let mut tokenizer = Tokenizer::from("2+2 = 5");
    assert_eq!(
        tokenizer.next(),
        Some(token!(TokenKind::Number(NumberKind::Decimal); 1))
    );
    assert_eq!(tokenizer.next(), Some(token!(Kind![+]; 1)));
    assert_eq!(
        tokenizer.next(),
        Some(token!(TokenKind::Number(NumberKind::Decimal); 1))
    );
    assert_eq!(tokenizer.next(), Some(token!(TokenKind::Ws; 1)));
    assert_eq!(tokenizer.next(), Some(token!(Kind![=]; 1)));
    assert_eq!(tokenizer.next(), Some(token!(TokenKind::Ws; 1)));
    assert_eq!(
        tokenizer.next(),
        Some(token!(TokenKind::Number(NumberKind::Decimal); 1))
    );
    assert_eq!(tokenizer.next(), None);
}
