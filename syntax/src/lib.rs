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

mod ast;
mod utils;

pub use crate::ast::*;

lalrpop_util::lalrpop_mod!(
    #[allow(dead_code)]
    grammar
);
