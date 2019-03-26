use codespan::{ByteIndex, ByteSpan};
use serde_derive::{Deserialize, Serialize};

pub(crate) fn s(start: usize, end: usize) -> ByteSpan {
    ByteSpan::new(ByteIndex(start as u32), ByteIndex(end as u32))
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

#[cfg(test)]
mod tests {
    use super::*;
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
}
