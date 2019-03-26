#[macro_use]
extern crate lalrpop_util;

mod ast;

pub use crate::ast::*;

lalrpop_util::lalrpop_mod!(
    #[allow(dead_code)]
    grammar
);
