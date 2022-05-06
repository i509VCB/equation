#![no_std]

extern crate alloc;

use alloc::vec::Vec;
use equation_lexer::Token;
use node::{Function, Node, Span};
use resolve::Resolver;
use rust_decimal::Decimal;

use crate::node::{Equation, Operator, OperatorKind};

pub mod node;
pub mod resolve;

/// Returned value from evaluation.
#[cfg_attr(feature = "fmt", derive(Debug))]
pub enum Evaluated {
    /// Integer
    Int(i64),

    /// Decimal number
    Decimal(Decimal),

    /// Equability
    Eq(bool),

    /// Comparison
    ///
    /// If true, then the value was greater than, otherwise less than.
    Cmp(bool),

    /// The value is undefined.
    Undefined,
}

#[cfg_attr(feature = "fmt", derive(Debug))]
pub enum ErrorKind {}

#[non_exhaustive]
#[cfg_attr(feature = "fmt", derive(Debug))]
pub struct Error {
    pub kind: ErrorKind,
    pub span: Span,
}

pub fn eval(resolver: &dyn Resolver, str: &str) -> Result<Evaluated, Error> {
    eval_tokens(resolver, equation_lexer::Tokenizer::from(str))
}

pub fn eval_tokens(
    resolver: &dyn Resolver,
    tokens: impl Iterator<Item = Token>,
) -> Result<Evaluated, Error> {
    eval_nodes(resolver, Node::from_tokens(tokens).expect("TODO: Err"))
}

pub fn eval_nodes<'a>(
    resolver: &dyn Resolver,
    nodes: impl Iterator<Item = Node<'a>>,
) -> Result<Evaluated, Error> {
    // Most of the time operations do not go deeper than 4, let's preallocate for 4 entries.
    let mut stack = Vec::<Node>::with_capacity(4);

    for node in nodes {
        match node {
            Node::Unary(_) => stack.push(node),

            Node::Function(Function { name, .. }) => {
                resolver.resolve_function(name, &mut stack)?;
            }

            Node::Operator(Operator { kind, .. }) => match kind {
                OperatorKind::Add => todo!(),
                OperatorKind::Sub => todo!(),
                OperatorKind::Mul => todo!(),
                OperatorKind::Div => todo!(),
                OperatorKind::Mod => todo!(),
                OperatorKind::Pow => todo!(),
            },

            Node::Equation(Equation { kind: _, .. }) => todo!(),
        }
    }

    todo!()
}
