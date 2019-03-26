use codespan::{ByteIndex, ByteSpan};

pub(crate) fn s(start: usize, end: usize) -> ByteSpan {
    ByteSpan::new(ByteIndex(start as u32), ByteIndex(end as u32))
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Declaration(Declaration),
}

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! parse_test {
        ($name:ident, $parser:tt, $input:expr => $expected:expr) => {
            #[test]
            fn $name() {
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
}
