use alloc::vec::Vec;
use rust_decimal::Decimal;

use crate::{node::Node, Error, Evaluated};

pub enum Constant {
    Integer(i64),

    Decimal(Decimal),
}

/// A trait used to resolve unknown constants and functions.
pub trait Resolver {
    /// Resolve a constant represented by a symbol
    ///
    /// If the symbol is not able to be resolved, then return [`None`].
    fn resolve_const_symbol(&self, name: &str) -> Option<Constant>;

    /// Resolve a function
    ///
    /// The return value of this function indicates how many unary operands need to be on the stack
    fn resolve_function(&self, name: &str, stack: &mut Vec<Node>) -> Result<Evaluated, Error>;
}
