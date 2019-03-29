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

lalrpop_util::lalrpop_mod!(
    #[allow(dead_code)]
    grammar
);

use codespan::ByteIndex;

pub fn parse(
    src: &str,
) -> Result<Program, lalrpop_util::ParseError<ByteIndex, String, &'static str>>
{
    crate::grammar::ProgramParser::new()
        .parse(src)
        .map_err(|e| e.map_location(|loc| ByteIndex(loc as u32)))
        .map_err(|e| e.map_token(|tok| tok.to_string()))
}
