use crate::hir::{Symbol, Type};
use specs::{Entities, System, WriteStorage};

/// A system for registering all builtin types and functions.
pub struct Builtins;

impl Builtins {
    pub const NAME: &'static str = "builtins";
    pub const TYPES: &'static [&'static str] = &[
        "byte", "word", "dword", "int", "dint", "real", "lreal", "time", "date", "char", "string",
    ];
}

impl<'a> System<'a> for Builtins {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Symbol>,
        WriteStorage<'a, Type>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut symbols, mut types) = data;

        for name in Builtins::TYPES {
            entities
                .build_entity()
                .with(
                    Type {
                        name: name.to_string(),
                    },
                    &mut types,
                )
                .with(
                    Symbol {
                        name: name.to_string(),
                    },
                    &mut symbols,
                )
                .build();
        }
    }
}
