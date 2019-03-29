#[macro_use]
extern crate lalrpop_util;

#[macro_export]
macro_rules! defer {
    ($value:expr, $ty:ident, $( $variant:ident )|* => |$name:ident| $eval:block) => {
        match $value {
            $(
                $ty::$variant(ref $name) => $eval,
            )*
        }
    };
}

#[macro_use]
pub mod macros;

mod ast;
mod utils;

pub use crate::ast::*;
pub type ParseError = lalrpop_util::ParseError<ByteIndex, String, &'static str>;

lalrpop_util::lalrpop_mod!(
    #[allow(dead_code)]
    grammar
);

use codespan::ByteIndex;

macro_rules! impl_from_str {
    ($name:ident => $parser:ident) => {
        impl ::std::str::FromStr for $crate::$name {
            type Err = $crate::ParseError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                $crate::grammar::$parser::new()
                    .parse(s)
                    .map_err(|e| e.map_location(|loc| ByteIndex(loc as u32)))
                    .map_err(|e| e.map_token(|tok| tok.to_string()))
            }
        }
    };
    ($( $name:ident => $parser:ident;)*) => {
        $(
            impl_from_str!($name => $parser);
        )*
    };
}

impl_from_str! {
    File => FileParser;
    Program => ProgramParser;
    Expression => ExprParser;
    Statement => StmtParser;
}
