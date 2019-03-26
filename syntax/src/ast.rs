use codespan::ByteSpan;
use serde_derive::{Deserialize, Serialize};
use std::any::Any;

pub trait AstNode: Any {
    fn span(&self) -> ByteSpan;
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Statement {
    Declaration(Declaration),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Identifier {
    pub value: String,
    pub span: ByteSpan,
}

impl Identifier {
    pub fn new<S: Into<String>>(value: S, span: ByteSpan) -> Identifier {
        Identifier {
            value: value.into(),
            span,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Declaration {
    pub ident: Identifier,
    pub ty: Identifier,
    pub span: ByteSpan,
}

impl Declaration {
    pub fn new(
        ident: Identifier,
        ty: Identifier,
        span: ByteSpan,
    ) -> Declaration {
        Declaration { ident, ty, span }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Assignment {
    pub variable: Identifier,
    pub value: Expression,
    pub span: ByteSpan,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Expression {
    Literal(Literal),
    Variable(Identifier),
    Binary(BinaryExpression),
    Unary(UnaryExpression),
    Call(FunctionCall),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BinaryExpression {
    pub left: Box<Expression>,
    pub right: Box<Expression>,
    pub op: BinOp,
    pub span: ByteSpan,
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum BinOp {
    Add,
    Subtract,
    Or,
    Xor,
    And,
    Equals,
    NotEquals,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
    Multiply,
    Divide,
    Modulo,
    Not,
    Exponent,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnaryExpression {
    pub value: Box<Expression>,
    pub op: UnaryOp,
    pub span: ByteSpan,
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum UnaryOp {
    Not,
    Negate,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Literal {
    pub kind: LiteralKind,
    pub span: ByteSpan,
}

impl Literal {
    pub fn new<K: Into<LiteralKind>>(kind: K, span: ByteSpan) -> Literal {
        Literal {
            kind: kind.into(),
            span,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LiteralKind {
    Boolean(bool),
    Integer(i64),
    Float(f64),
    String(String),
}

impl From<bool> for LiteralKind {
    fn from(other: bool) -> LiteralKind {
        LiteralKind::Boolean(other)
    }
}

impl From<i64> for LiteralKind {
    fn from(other: i64) -> LiteralKind {
        LiteralKind::Integer(other)
    }
}

impl From<f64> for LiteralKind {
    fn from(other: f64) -> LiteralKind {
        LiteralKind::Float(other)
    }
}

impl From<String> for LiteralKind {
    fn from(other: String) -> LiteralKind {
        LiteralKind::String(other)
    }
}

impl<'a> From<&'a str> for LiteralKind {
    fn from(other: &'a str) -> LiteralKind {
        LiteralKind::String(other.to_string())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FunctionCall {
    pub name: Identifier,
    pub args: Vec<FunctionArg>,
    pub span: ByteSpan,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FunctionArg {
    Bare(Expression),
    Named(Assignment),
}

macro_rules! impl_ast_node {
    ($name:tt => $($variant:tt)|*) => {
        impl AstNode for $name {
            fn span(&self) -> ByteSpan {
                match self {
                    $(
                        $name::$variant(ref inner) => inner.span(),
                    )*
                }
            }
        }
    };
    ($($name:ty),*) => {
        $(
            impl AstNode for $name {
                fn span(&self) -> ByteSpan {
                    self.span
                }
            }
        )*
    };
}

impl_ast_node!(
    Literal,
    Assignment,
    Declaration,
    Identifier,
    BinaryExpression,
    UnaryExpression,
    FunctionCall
);
impl_ast_node!(Expression => Literal | Binary | Unary | Variable | Call);
impl_ast_node!(Statement => Declaration);
impl_ast_node!(FunctionArg => Bare | Named);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::*;
    use pretty_assertions::assert_eq;

    macro_rules! parse_test {
        ($name:ident, $parser:tt, $input:expr => $expected:expr) => {
            #[test]
            fn $name() {
                #[allow(unused_imports)]
                use $crate::grammar::*;
                let got = $parser::new().parse($input).unwrap();
                assert_eq!(got, $expected);
            }
        };
    }

    parse_test!(simple_ident, IdentParser, "hello" => Identifier::new("hello", s(0, 5)));
    parse_test!(ident_with_numbers, IdentParser, "hello_45" => Identifier::new("hello_45", s(0, 8)));

    parse_test!(example_decl, DeclParser, "x: Bool" => Declaration {
        ident: Identifier::new("x", s(0, 1)),
        ty: Identifier::new("Bool", s(3, 7)),
        span: s(0, 7),
    });

    parse_test!(assign_literal, AssignmentParser, "meaning_of_life := 42" => Assignment {
        variable: Identifier::new("meaning_of_life", s(0, 15)),
        value: Expression::Literal(Literal {
            kind: LiteralKind::Integer(42),
            span: s(19, 21),
        }),
        span: s(0, 21),
    });

    parse_test!(function_call, ExprParser, "foo()" => Expression::Call(FunctionCall {
        name: Identifier::new("foo", s(0, 3)),
        args: Vec::new(),
        span: s(0, 5),
    }));

    parse_test!(function_call_with_args, ExprParser, "foo(1, second := 2)" => Expression::Call(FunctionCall {
        name: Identifier::new("foo", s(0, 3)),
        args: vec![
            FunctionArg::Bare(Expression::Literal(Literal::new(1, s(4, 5)))),
            FunctionArg::Named(Assignment {
                variable: Identifier::new("second", s(7, 13)),
                value: Expression::Literal(Literal::new(2, s(17, 18))),
                span: s(7, 18),
            }),
        ],
        span: s(0, 19),
    }));

    parse_test!(super_complex_expression, ExprParser, "5*5 + add(-(9**2), -34/pi)" =>
        Expression::Binary(BinaryExpression {
            left: Box::new(Expression::Binary(BinaryExpression {
                left: Box::new(Expression::Literal(Literal {
                    kind: LiteralKind::Integer(5),
                    span: s(0, 1),
                })),
                right: Box::new(Expression::Literal(Literal {
                    kind: LiteralKind::Integer(5),
                    span: s(2, 3),
                })),
                op: BinOp::Multiply,
                span: s(0, 3),
            })),
            right: Box::new(Expression::Call(FunctionCall {
                name: Identifier {
                    value: String::from("add"),
                    span: s(6, 9),
                },
                args: vec![
                    FunctionArg::Bare(Expression::Unary(UnaryExpression {
                        value: Box::new(Expression::Binary(BinaryExpression {
                            left: Box::new(Expression::Literal(Literal {
                                kind: LiteralKind::Integer(9),
                                span: s(12, 13),
                            })),
                            right: Box::new(Expression::Literal(Literal {
                                kind: LiteralKind::Integer(2),
                                span: s(15, 16),
                            })),
                            op: BinOp::Exponent,
                            span: s(12, 16),
                        })),
                        op: UnaryOp::Negate,
                        span: s(10, 17),
                    })),
                    FunctionArg::Bare(Expression::Binary(BinaryExpression {
                        left: Box::new(Expression::Literal(Literal {
                            kind: LiteralKind::Integer(-34),
                            span: s(19, 22),
                        })),
                        right: Box::new(Expression::Variable(Identifier {
                            value: String::from("pi"),
                            span: s(23, 25),
                        })),
                        op: BinOp::Divide,
                        span: s(19, 25),
                    })),
                ],
                span: s(6, 26),
            })),
            op: BinOp::Add,
            span: s(0, 26),
        })
    );
}
