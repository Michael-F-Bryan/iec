use crate::hir::{Symbol, SymbolTable, Type};
use specs::{Entities, System, Write, WriteStorage};

/// A system for registering all builtin types and functions.
pub struct Builtins;

impl Builtins {
    pub const TYPES: &'static [&'static str] = &[
        "byte", "word", "dword", "int", "dint", "real", "lreal", "time", "date", "char", "string",
    ];
}

impl<'a> System<'a> for Builtins {
    type SystemData = (Entities<'a>, Write<'a, SymbolTable>, WriteStorage<'a, Type>);

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut symbol_table, mut types) = data;

        for name in Builtins::TYPES {
            let ent = entities
                .build_entity()
                .with(
                    Type {
                        name: name.to_string(),
                    },
                    &mut types,
                )
                .build();
            symbol_table.insert(name, Symbol::Type(ent));
        }
    }
}

impl<'a> crate::systems::Pass<'a> for Builtins {
    const NAME: &'static str = "builtins";
}
