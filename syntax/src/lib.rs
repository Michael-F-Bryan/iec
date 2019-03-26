#[macro_use]
extern crate lalrpop_util;
#[test]
#[macro_use]
extern crate pretty_assertions;

pub mod ast;

lalrpop_util::lalrpop_mod!(
    #[allow(dead_code)]
    grammar
);
