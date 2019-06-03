use crate::hir::{Function, FunctionBlock, Program, Symbol, SymbolTable};
use crate::Diagnostics;
use iec_syntax::{File, Item};
use slog::Logger;
use specs::{Entities, ReadExpect, System, Write, WriteExpect, WriteStorage};

#[derive(Debug, Copy, Clone, Default)]
pub struct SymbolDiscovery;

impl<'a> System<'a> for SymbolDiscovery {
    type SystemData = (
        WriteExpect<'a, Diagnostics>,
        ReadExpect<'a, File>,
        ReadExpect<'a, Logger>,
        Entities<'a>,
        Write<'a, SymbolTable>,
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
            mut symbol_table,
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
                        &mut symbol_table,
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
    symbol_table: &mut SymbolTable,
) {
    if let Some(d) = symbol_table.check_for_duplicate_ident(&p.name) {
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
        .build();

    symbol_table.insert(&p.name.value, Symbol::Program(program_id));
    slog::debug!(logger, "Found a program"; 
        "name" => &p.name.value,
        "id" => format_args!("{:?}", program_id));
}
