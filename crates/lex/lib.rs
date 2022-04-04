#![no_std]

// TODO: Crate docs

use core::str::Chars;

/// A kind of brace
#[cfg_attr(feature = "fmt", derive(Debug))]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BraceKind {
    /// Round brackets: `(` or `)`
    Round,

    /// Square brackets: `[` or `]`
    Square,

    /// Curly brackets: `{` or `}`
    Curly,
}

/// The kind of number
#[cfg_attr(feature = "fmt", derive(Debug))]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum NumberKind {
    /// Decimal number
    ///
    /// A decimal number may be an integer or have decimal points.
    Decimal,

    /// Hexadecimal number
    Hexadecimal,

    /// Octal number
    Octal,

    /// Binary number
    Binary,

    /// Complex number
    Complex,
}

/// A kind of token
#[cfg_attr(feature = "fmt", derive(Debug))]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    /// `+`
    Plus,

    /// `-`
    Minus,

    /// `*`
    Multiply,

    /// `/`
    Divide,

    /// `%`
    Modulo,

    /// A brace
    Brace {
        /// The kind of brace
        kind: BraceKind,

        /// Whether the brace is opening or closing
        open: bool,
    },

    /// `=`
    Eq,

    /// `>`
    Ge,

    /// `<`
    Le,

    /// `&`
    Amp,

    /// `^`
    Caret,

    /// `!`
    Exclamation,

    /// `,`
    Comma,

    /// A sequence of characters
    ///
    /// The sequence of characters may be one of the following:
    /// - Magic values (Euler's number, Pi, infinity)
    /// - The name of a function (ln (natural log), sin, cos, square root).
    /// - Some invalid sequence of characters
    Chars,

    /// A number
    ///
    /// TODO: Possibly store data about the kind of number?
    Number(NumberKind),

    /// Whitespace character(s)
    ///
    /// See [`char::is_ascii_whitespace`].
    Ws,

    /// An invalid token
    ///
    /// This kind of token is handled for error checking purposes.
    Invalid,
}

/// A type-macro which expands to the [`TokenKind`] representation of a given token.
///
/// The following tokens may be used in this macro:
/// - `+`
/// - `-`
/// - `*`
/// - `/`
/// - `%`
/// - `=`
/// - `>`
/// - `<`
/// - `&`
/// - `^`
/// - `!`
/// - `,`
///
/// Note that [`TokenKind::Brace`] cannot be created using this macro. Use the [`Brace`] macro instead to generate a
/// [`TokenKind`] for a brace.
///
/// ```
/// use equation_lexer::{Kind, TokenKind};
///
/// let plus = Kind![+];
/// assert_eq!(plus, TokenKind::Plus);
/// ```
#[macro_export]
macro_rules! Kind {
    [+] => { $crate::TokenKind::Plus };
    [-] => { $crate::TokenKind::Minus };
    [*] => { $crate::TokenKind::Multiply };
    [/] => { $crate::TokenKind::Divide };
    [%] => { $crate::TokenKind::Modulo };
    [=] => { $crate::TokenKind::Eq };
    [>] => { $crate::TokenKind::Ge };
    [<] => { $crate::TokenKind::Le };
    [&] => { $crate::TokenKind::Amp };
    [^] => { $crate::TokenKind::Caret };
    [!] => { $crate::TokenKind::Exclamation };
    [,] => { $crate::TokenKind::Comma };
}

/// A type-macro which expands to the [`TokenKind`] representation of a type of brace.
///
/// ```
/// use equation_lexer::{Brace, BraceKind, TokenKind};
///
/// let open_brace = TokenKind::Brace {
///     kind: BraceKind::Square,
///     open: true,
/// };
/// assert_eq!(open_brace, Brace![Square, Open]);
/// ```
#[macro_export]
macro_rules! Brace {
    [$kind: ident, Open] => {{
        $crate::TokenKind::Brace {
            kind: $crate::BraceKind::$kind,
            open: true,
        }
    }};

    [$kind: ident, Close] => {{
        $crate::TokenKind::Brace {
            kind: $crate::BraceKind::$kind,
            open: false,
        }
    }};
}

#[cfg_attr(feature = "fmt", derive(Debug))]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Token {
    /// The token kind
    pub kind: TokenKind,

    /// Length of the token in characters
    pub len: usize,
}

#[cfg_attr(feature = "fmt", derive(Debug))]
#[derive(Clone)]
pub struct Tokenizer<'a> {
    iter: Chars<'a>,
}

impl Tokenizer<'_> {
    pub fn peek(&mut self) -> Option<Token> {
        let str = self.iter.as_str();
        let mut chars = str.chars();
        let (kind, mut len) = kind_with_iter(&mut chars)?;

        // Determine the length of an invalid token
        if kind == TokenKind::Invalid {
            loop {
                match kind_with_iter(&mut chars) {
                    Some((kind, _)) if kind == TokenKind::Invalid => {
                        len += 1;
                    }

                    _ => break,
                }
            }
        }

        Some(Token { kind, len })
    }
}

impl Iterator for Tokenizer<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let token = self.peek()?;
        let _ = self.iter.nth(token.len - 1);
        Some(token)
    }
}

impl<'a> From<&'a str> for Tokenizer<'a> {
    /// Creates a tokenizer from a string.
    ///
    /// ```
    /// use equation_lexer::{Kind, Token, Tokenizer};
    ///
    /// let mut parser = Tokenizer::from("+");
    ///
    /// assert_eq!(parser.next(), Some(Token {
    ///     kind: Kind![+],
    ///     len: 1,
    /// }));
    /// // No remaining tokens
    /// assert_eq!(parser.next(), None);
    /// ```
    fn from(str: &'a str) -> Self {
        Self { iter: str.chars() }
    }
}

/// Helper macro to parse a 1 character long token.
///
/// This macro returns the length of the token `1` and the token expression.
macro_rules! tok1 {
    ($kind: expr) => {{
        ($kind, 1usize)
    }};
}

fn kind_with_iter(chars: &mut Chars) -> Option<(TokenKind, usize)> {
    let (kind, len) = match chars.next()? {
        '+' => tok1!(Kind![+]),
        '-' => tok1!(Kind![-]),
        '*' => tok1!(Kind![*]),
        '/' => tok1!(Kind![/]),
        '%' => tok1!(Kind![%]),
        '=' => tok1!(Kind![=]),
        '>' => tok1!(Kind![>]),
        '<' => tok1!(Kind![<]),
        '&' => tok1!(Kind![&]),
        '^' => tok1!(Kind![^]),
        '!' => tok1!(Kind![!]),
        ',' => tok1!(Kind![,]),
        '(' => tok1!(Brace![Round, Open]),
        ')' => tok1!(Brace![Round, Close]),
        '[' => tok1!(Brace![Square, Open]),
        ']' => tok1!(Brace![Square, Close]),
        '{' => tok1!(Brace![Curly, Open]),
        '}' => tok1!(Brace![Curly, Close]),

        c => match c {
            '0'..='9' => {
                let (kind, len) = number(c, chars);
                (TokenKind::Number(kind), len)
            }

            c if c.is_ascii_alphabetic() => (TokenKind::Chars, chrs(chars) + 1),
            c if c.is_ascii_whitespace() => (TokenKind::Ws, ws(chars) + 1),
            _ => (TokenKind::Invalid, 1),
        },
    };

    Some((kind, len))
}

fn chrs(chars: &mut Chars) -> usize {
    let mut len = 0;

    loop {
        match chars.next() {
            Some(c) if c.is_ascii_alphanumeric() => {
                len += 1;
            }
            _ => break len,
        }
    }
}

fn ws(chars: &mut Chars) -> usize {
    let mut len = 0;

    loop {
        match chars.next() {
            Some(c) if c.is_ascii_whitespace() => {
                len += 1;
            }
            _ => break len,
        }
    }
}

fn number(first_char: char, chars: &mut Chars) -> (NumberKind, usize) {
    // 0 first character may imply multiple types of numbers
    if first_char == '0' {
        match chars.next() {
            Some('x') => (NumberKind::Hexadecimal, hexadecimal_number(chars) + 2),
            Some('b') => (NumberKind::Binary, decimal_number(chars).1 + 2),
            Some('o') => (NumberKind::Octal, decimal_number(chars).1 + 2),
            Some('e') | Some('E') => (NumberKind::Complex, decimal_or_complex_number(chars).1 + 2),
            Some('.') | Some('0'..='9') => {
                let (kind, len) = decimal_or_complex_number(chars);
                (kind, len + 2)
            }
            // Just a zero
            _ => (NumberKind::Decimal, 1),
        }
    } else {
        // Otherwise assume we have a decimal or complex number
        let (kind, len) = decimal_or_complex_number(chars);
        (kind, len + 1)
    }
}

fn hexadecimal_number(chars: &mut Chars) -> usize {
    let mut len = 0;

    loop {
        match chars.next() {
            Some('a'..='f') | Some('A'..='F') | Some('0'..='9') => {
                len += 1;
            }

            _ => break len,
        }
    }
}

fn decimal_or_complex_number(chars: &mut Chars) -> (NumberKind, usize) {
    let mut kind = NumberKind::Decimal;
    let mut len = 0;

    loop {
        match chars.next() {
            Some('.') | Some('0'..='9') => {
                len += 1;
            }
            // Complex number
            Some('e') | Some('E') => {
                kind = NumberKind::Complex;
                len += 1;
            }

            _ => break (kind, len),
        }
    }
}

fn decimal_number(chars: &mut Chars) -> (NumberKind, usize) {
    let mut len = 0;

    loop {
        match chars.next() {
            Some('0'..='9') => {
                len += 1;
            }

            _ => break (NumberKind::Decimal, len),
        }
    }
}
