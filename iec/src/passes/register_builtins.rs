use super::symbol_table::SymbolTable;
use super::{Pass, PassContext};
use crate::ecs::{ReadWrite, SingletonMut};
use crate::hir::{Symbol, Type};
use typename::TypeName;

#[derive(TypeName)]
pub enum RegisterBuiltins {}

pub const BUILTIN_TYPES: &[&str] = &[
    "byte", "word", "dword", "int", "dint", "real", "lreal", "time", "date",
    "char", "string",
];

impl<'r> Pass<'r> for RegisterBuiltins {
    type Arg = ();
    type Storage = (SingletonMut<'r, SymbolTable>, ReadWrite<'r, Type>);
    const DESCRIPTION: &'static str = "Register builtin types and functions";

    fn run(_: &Self::Arg, _ctx: &mut PassContext<'_>, storage: Self::Storage) {
        let (mut symbol_table, mut types) = storage;

        for name in BUILTIN_TYPES {
            let type_id = types.insert(Type {
                name: name.to_string(),
            });
            symbol_table.insert(name, Symbol::Type(type_id));
        }
    }
}
