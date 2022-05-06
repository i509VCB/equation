//! Lexing for math equations
//!
//! This crate provides the [`Tokenizer`] type, used to parse a string and get the components of a math expression.
//! Tokens are representations of the human readable expression in the input and should be converted into
//! another form for evaluation.
//!
//! A [`Tokenizer`] may be constructed using it's [`From`] implementation.

#![no_std]
#![forbid(clippy::expect_used, clippy::panic, clippy::unwrap_used)]
#![warn(missing_docs)]
#![cfg_attr(feature = "fmt", forbid(missing_debug_implementations))]

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
///
/// A number token is simply a way to guess if the value is likely a number. The number itself could be invalid.
#[cfg_attr(feature = "fmt", derive(Debug))]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum NumberKind {
    /// Decimal number
    ///
    /// A decimal number may be an integer, have decimal points or be a complex number.
    Decimal,

    /// Binary number
    Binary,

    /// Hexadecimal number
    Hexadecimal,
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

/// A single token in an expression.
#[cfg_attr(feature = "fmt", derive(Debug))]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Token {
    /// The token kind
    pub kind: TokenKind,

    /// Length of the token in characters
    pub len: usize,
}

/// Tokenizer, an iterator of [`Tokens`](Token).
#[cfg_attr(feature = "fmt", derive(Debug))]
#[derive(Clone)]
pub struct Tokenizer<'a> {
    iter: Chars<'a>,
}

impl Tokenizer<'_> {
    /// Peek by a specified amount for the next [`Token`].
    ///
    /// This does not advance the iterator and repeated calls return the same token.
    ///
    /// If the amount to peek by is zero, then [`None`] is returned.
    pub fn peek(&self, by: usize) -> Option<Token> {
        let mut count = by;
        // Convert the iterator into a str so peeking does not advance the real iterator.
        let mut str = self.iter.as_str();

        loop {
            // If we peek by zero tokens, this will return None
            count = count.checked_sub(1)?;

            let mut chars = str.chars();
            let (kind, mut len) = kind_with_iter(&mut chars)?;

            // Determine the length of an invalid token
            if kind == TokenKind::Invalid {
                loop {
                    // Consume the next character in the iterator until the end or invalid token.
                    match kind_with_iter(&mut chars) {
                        Some((kind, _)) if kind == TokenKind::Invalid => {
                            len += 1;
                        }

                        _ => break,
                    }
                }
            }

            if count == 0 {
                break Some(Token { kind, len });
            }

            // Take remainder of the str borrowed from the true iterator to peek at the next token.
            // TODO: Use advance_by when stabilized.
            str = chars.as_str().get((len - 1)..)?;
        }
    }
}

impl Iterator for Tokenizer<'_> {
    type Item = Token;

    /// Read and consume the next token.
    ///
    /// Subsequent calls will return the next token in the input string.
    ///
    /// Returns [`None`] if no more tokens are available.
    fn next(&mut self) -> Option<Self::Item> {
        let token = self.peek(1)?;
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

impl<'a, T> From<&'a T> for Tokenizer<'a>
where
    T: AsRef<str>,
{
    fn from(str: &'a T) -> Self {
        Self::from(str.as_ref())
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

            c if c.is_ascii_alphabetic() => (TokenKind::Chars, chr(chars) + 1),
            c if c.is_ascii_whitespace() => (TokenKind::Ws, ws(chars) + 1),
            _ => (TokenKind::Invalid, 1),
        },
    };

    Some((kind, len))
}

fn chr(chars: &mut Chars) -> usize {
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
            Some('x') => (NumberKind::Hexadecimal, hexadecimal(chars) + 2),
            // Validity of binary is checked later in parsing process.
            Some('b') => (NumberKind::Binary, decimal_or_complex(chars) + 2),
            Some('.') | Some('0'..='9') | Some('e') | Some('E') => {
                (NumberKind::Decimal, decimal_or_complex(chars) + 2)
            }

            // Just a zero
            _ => (NumberKind::Decimal, 1),
        }
    } else {
        // Otherwise assume a decimal or complex number.
        (NumberKind::Decimal, decimal_or_complex(chars) + 1)
    }
}

fn hexadecimal(chars: &mut Chars) -> usize {
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

fn decimal_or_complex(chars: &mut Chars) -> usize {
    let mut len = 0;

    loop {
        match chars.next() {
            Some('.') | Some('0'..='9') | Some('e') | Some('E') => {
                len += 1;
            }

            _ => break len,
        }
    }
}
