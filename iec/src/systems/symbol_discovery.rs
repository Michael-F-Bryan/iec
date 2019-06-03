use crate::hir::{Function, FunctionBlock, Program, Symbol, };
use crate::Diagnostics;
use iec_syntax::{File, Item};
use slog::Logger;
use specs::{Entities, ReadExpect, System, Write, WriteStorage, Join};
use codespan_reporting::{Label, Diagnostic};

#[derive(Debug, Copy, Clone, Default)]
pub struct SymbolDiscovery;

impl SymbolDiscovery {
    pub const NAME: &'static str = "symbol-discovery";
}

impl<'a> System<'a> for SymbolDiscovery {
    type SystemData = (
        Write<'a, Diagnostics>,
        ReadExpect<'a, File>,
        ReadExpect<'a, Logger>,
        Entities<'a>,
        WriteStorage<'a, Symbol>,
        WriteStorage<'a, Program>,
        WriteStorage<'a, Function>,
        WriteStorage<'a, FunctionBlock>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut diagnostics,
            file,
            logger,
            entities,
            mut symbols,
            mut programs,
            _functions,
            _function_blocks,
        ) = data;

        for item in &file.items {
            match item {
                Item::Program(ref p) => {
                    register_program(
                        p,
                        &entities,
                        &logger,
                        &mut diagnostics,
                        &mut programs,
                        &mut symbols,
                    );
                }
                Item::Function(_) => {
                    unimplemented!();
                    //register_function(f, &mut functions, &mut symbol_table)
                }
                Item::FunctionBlock(_) => {
                    unimplemented!();
                    //register_function_block( fb, &mut function_blocks, &mut symbol_table);
                }
            }
        }
    }
}

fn register_program(
    p: &iec_syntax::Program,
    entities: &Entities<'_>,
    logger: &Logger,
    diags: &mut Diagnostics,
    programs: &mut WriteStorage<'_, Program>,
    symbols: &mut WriteStorage<'_, Symbol>,
) {
    if (entities, &mut *symbols).join().any(|(_, symbol)| symbol.name == p.name.value) {
                let d = Diagnostic::new_error("Name is already declared").with_label(
                    Label::new_primary(p.name.span).with_message("Duplicate declared here"),
                );
        diags.push(d);
        return;
    }

    let program_id = entities
        .build_entity()
        .with(
            Program {
                name: p.name.value.clone(),
                variables: Vec::new(),
            },
            programs,
        )
        .with(Symbol { name: p.name.value.clone()}, symbols)
        .build();

    slog::debug!(logger, "Found a program"; 
        "name" => &p.name.value,
        "id" => format_args!("{:?}", program_id));
}
