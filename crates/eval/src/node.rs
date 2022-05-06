use equation_lexer::Token;

#[derive(Clone, Copy)]
#[cfg_attr(feature = "fmt", derive(Debug))]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

pub trait Spanned {
    fn span(&self) -> Span;
}

#[cfg_attr(feature = "fmt", derive(Debug))]
#[derive(Clone, Copy)]
pub struct Unary<'a> {
    pub kind: UnaryKind<'a>,
    span: Span,
}

impl Spanned for Unary<'_> {
    fn span(&self) -> Span {
        self.span
    }
}

#[cfg_attr(feature = "fmt", derive(Debug))]
#[derive(Clone, Copy)]
pub enum UnaryKind<'a> {
    /// Integer value
    Int(i64),

    /// Decimal value
    Decimal(f64),

    /// Constant symbol value
    ///
    /// This needs to be resolved.
    Constant(&'a str),
}

#[cfg_attr(feature = "fmt", derive(Debug))]
#[derive(Clone, Copy)]
pub struct Function<'a> {
    pub name: &'a str,
    span: Span,
}

impl Spanned for Function<'_> {
    fn span(&self) -> Span {
        self.span
    }
}

#[cfg_attr(feature = "fmt", derive(Debug))]
#[derive(Clone, Copy)]
pub enum OperatorKind {
    Add,

    Sub,

    Mul,

    Div,

    Mod,

    Pow,
}

#[cfg_attr(feature = "fmt", derive(Debug))]
#[derive(Clone, Copy)]
pub struct Operator {
    pub kind: OperatorKind,
    span: Span,
}

impl Spanned for Operator {
    fn span(&self) -> Span {
        self.span
    }
}

#[cfg_attr(feature = "fmt", derive(Debug))]
#[derive(Clone, Copy)]
pub enum EquationKind {
    Eq,

    Neq,

    Ge,

    Le,
}

#[cfg_attr(feature = "fmt", derive(Debug))]
#[derive(Clone, Copy)]
pub struct Equation {
    pub kind: EquationKind,
    span: Span,
}

impl Spanned for Equation {
    fn span(&self) -> Span {
        self.span
    }
}

#[cfg_attr(feature = "fmt", derive(Debug))]
#[derive(Clone, Copy)]
pub enum Node<'a> {
    Unary(Unary<'a>),

    Function(Function<'a>),

    Operator(Operator),

    Equation(Equation),
}

impl Spanned for Node<'_> {
    fn span(&self) -> Span {
        match self {
            Node::Unary(token) => token.span(),
            Node::Function(token) => token.span(),
            Node::Operator(token) => token.span(),
            Node::Equation(token) => token.span(),
        }
    }
}

impl Node<'_> {
    pub fn parse(str: &str) -> Result<impl Iterator<Item = Node> + '_, ()> {
        let tokens = equation_lexer::Tokenizer::from(str);
        Self::from_tokens(tokens)
    }

    pub fn from_tokens<'a>(
        _iter: impl Iterator<Item = Token>,
    ) -> Result<impl Iterator<Item = Node<'a>>, ()> {
        Ok(core::iter::empty())
    }
}
