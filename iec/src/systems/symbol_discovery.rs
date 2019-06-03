use crate::hir::{Function, FunctionBlock, Program, Symbol};
use crate::Diagnostics;
use codespan_reporting::{Diagnostic, Label};
use iec_syntax::{File, Identifier, Item};
use slog::Logger;
use specs::{Entities, Join, ReadExpect, System, Write, WriteStorage};

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
            mut functions,
            mut function_blocks,
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
                Item::Function(ref f) => {
                    register_function(
                        f,
                        &entities,
                        &logger,
                        &mut diagnostics,
                        &mut functions,
                        &mut symbols,
                    );
                }
                Item::FunctionBlock(ref f) => {
                    register_function_block(
                        f,
                        &entities,
                        &logger,
                        &mut diagnostics,
                        &mut function_blocks,
                        &mut symbols,
                    );
                }
            }
        }
    }
}

fn duplicate_symbol_check(
    name: &Identifier,
    entities: &Entities<'_>,
    symbols: &WriteStorage<'_, Symbol>,
) -> Option<Diagnostic> {
    if (entities, symbols)
        .join()
        .any(|(_, symbol)| symbol.name == name.value)
    {
        Some(
            Diagnostic::new_error("Name is already declared")
                .with_label(Label::new_primary(name.span).with_message("Duplicate declared here")),
        )
    } else {
        None
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
    if let Some(d) = duplicate_symbol_check(&p.name, entities, symbols) {
        diags.push(d);
        return;
    }

    let program_id = entities
        .build_entity()
        .with(Program, programs)
        .with(
            Symbol {
                name: p.name.value.clone(),
            },
            symbols,
        )
        .build();

    slog::debug!(logger, "Found a program"; 
        "name" => &p.name.value,
        "id" => format_args!("{:?}", program_id));
}

fn register_function(
    f: &iec_syntax::Function,
    entities: &Entities<'_>,
    logger: &Logger,
    diags: &mut Diagnostics,
    functions: &mut WriteStorage<'_, Function>,
    symbols: &mut WriteStorage<'_, Symbol>,
) {
    if let Some(d) = duplicate_symbol_check(&f.name, entities, symbols) {
        diags.push(d);
        return;
    }

    let function_id = entities
        .build_entity()
        .with(Function, functions)
        .with(
            Symbol {
                name: f.name.value.clone(),
            },
            symbols,
        )
        .build();

    slog::debug!(logger, "Found a function"; 
        "name" => &f.name.value,
        "id" => format_args!("{:?}", function_id));
}

fn register_function_block(
    f: &iec_syntax::FunctionBlock,
    entities: &Entities<'_>,
    logger: &Logger,
    diags: &mut Diagnostics,
    function_blocks: &mut WriteStorage<'_, FunctionBlock>,
    symbols: &mut WriteStorage<'_, Symbol>,
) {
    if let Some(d) = duplicate_symbol_check(&f.name, entities, symbols) {
        diags.push(d);
        return;
    }

    let function_id = entities
        .build_entity()
        .with(FunctionBlock, function_blocks)
        .with(
            Symbol {
                name: f.name.value.clone(),
            },
            symbols,
        )
        .build();

    slog::debug!(logger, "Found a function block"; 
        "name" => &f.name.value,
        "id" => format_args!("{:?}", function_id));
}
