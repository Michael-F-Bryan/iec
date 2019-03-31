use super::{Pass, PassContext};
use crate::ecs::{Container, EntityId, ReadWrite, SingletonMut};
use crate::hir::{Function, FunctionBlock, Program};
use crate::Diagnostics;
use codespan_reporting::{Diagnostic, Label};
use heapsize_derive::HeapSizeOf;
use iec_syntax::Item;
use std::collections::HashMap;
use typename::TypeName;

/// A component which can be used to figure out what type of thing a particular
/// name resolves to.
#[derive(Debug, Copy, Clone, PartialEq, TypeName, HeapSizeOf)]
pub enum Symbol {
    Program(EntityId),
    Function(EntityId),
    FunctionBlock(EntityId),
}

/// A cache for looking up a symbol based on its identifier.
#[derive(Debug, Default, Clone, PartialEq, TypeName, HeapSizeOf)]
pub struct SymbolTable(HashMap<String, EntityId>);

impl SymbolTable {
    pub fn insert(&mut self, name: &str, id: EntityId) {
        self.0.insert(name.to_lowercase(), id);
    }

    pub fn get(&self, name: &str) -> Option<EntityId> {
        let name = name.to_lowercase();
        self.0.get(&name).cloned()
    }

    pub fn inner(&self) -> &HashMap<String, EntityId> {
        &self.0
    }

    pub fn inner_mut(&mut self) -> &mut HashMap<String, EntityId> {
        &mut self.0
    }

    pub fn check_for_duplicate_ident(
        &self,
        ident: &iec_syntax::Identifier,
    ) -> Option<Diagnostic> {
        if self.get(&ident.value).is_none() {
            None
        } else {
            Some(
                Diagnostic::new_error("Name is already declared").with_label(
                    Label::new_primary(ident.span)
                        .with_message("Duplicate declared here"),
                ),
            )
        }
    }
}

pub struct SymbolTableResolution;

impl<'r> Pass<'r> for SymbolTableResolution {
    type Arg = iec_syntax::File;
    type Storage = (
        ReadWrite<'r, Symbol>,
        SingletonMut<'r, SymbolTable>,
        ReadWrite<'r, Program>,
        ReadWrite<'r, Function>,
        ReadWrite<'r, FunctionBlock>,
    );
    const DESCRIPTION: &'static str = "Find all know identifiers";

    fn run(
        arg: &iec_syntax::File,
        ctx: PassContext<'r>,
        storage: Self::Storage,
    ) {
        let (
            mut symbols,
            mut symbol_table,
            mut programs,
            mut functions,
            mut function_blocks,
        ) = storage;

        for item in &arg.items {
            match item {
                Item::Program(ref p) => register_program(
                    p,
                    &mut programs,
                    ctx.diags,
                    &mut symbols,
                    &mut symbol_table,
                ),
                Item::Function(ref f) => register_function(
                    f,
                    &mut functions,
                    ctx.diags,
                    &mut symbols,
                    &mut symbol_table,
                ),
                Item::FunctionBlock(ref fb) => register_function_block(
                    fb,
                    &mut function_blocks,
                    ctx.diags,
                    &mut symbols,
                    &mut symbol_table,
                ),
            }
        }
    }
}

fn register_program(
    p: &iec_syntax::Program,
    programs: &mut Container<Program>,
    diags: &mut Diagnostics,
    symbols: &mut Container<Symbol>,
    symbol_table: &mut SymbolTable,
) {
    if let Some(d) = symbol_table.check_for_duplicate_ident(&p.name) {
        diags.push(d);
        return;
    }

    let program = Program {
        name: p.name.value.clone(),
    };
    let program_id = programs.insert(program);

    let symbol = Symbol::Program(program_id);
    let symbol_id = symbols.insert(symbol);

    symbol_table.insert(&p.name.value, symbol_id);
}

fn register_function_block(
    fb: &iec_syntax::FunctionBlock,
    function_blocks: &mut Container<FunctionBlock>,
    diags: &mut Diagnostics,
    symbols: &mut Container<Symbol>,
    symbol_table: &mut SymbolTable,
) {
    if let Some(d) = symbol_table.check_for_duplicate_ident(&fb.name) {
        diags.push(d);
        return;
    }

    let function_block = FunctionBlock {
        name: fb.name.value.clone(),
    };
    let function_block_id = function_blocks.insert(function_block);

    let symbol = Symbol::FunctionBlock(function_block_id);
    let symbol_id = symbols.insert(symbol);
    symbol_table.insert(&fb.name.value, symbol_id);
}

fn register_function(
    f: &iec_syntax::Function,
    functions: &mut Container<Function>,
    diags: &mut Diagnostics,
    symbols: &mut Container<Symbol>,
    symbol_table: &mut SymbolTable,
) {
    if let Some(d) = symbol_table.check_for_duplicate_ident(&f.name) {
        diags.push(d);
        return;
    }

    let function = Function {
        name: f.name.value.clone(),
    };
    let function_block_id = functions.insert(function);

    let symbol = Symbol::FunctionBlock(function_block_id);
    let symbol_id = symbols.insert(symbol);
    symbol_table.insert(&f.name.value, symbol_id);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ecs::Resources;
    use crate::Diagnostics;
    use iec_syntax::File;

    #[test]
    fn identify_function_blocks_and_programs() {
        let ast = File {
            items: vec![
                iec_syntax::quote!(program main {}).into(),
                iec_syntax::quote!(function_block FUnc {}).into(),
            ],
            span: Default::default(),
        };
        let mut resources = Resources::new();
        let mut diags = Diagnostics::new();

        crate::passes::run_pass::<SymbolTableResolution>(
            &mut resources,
            &ast,
            &mut diags,
        );

        // we should have updated the symbol table appropriately
        let symbol_table = resources.get_singleton::<SymbolTable>();
        assert_eq!(symbol_table.0.len(), 2);
        assert!(symbol_table.0.contains_key("main"));
        assert!(symbol_table.0.contains_key("func"));

        let symbols = resources.get::<Symbol>();
        assert_eq!(symbols.len(), 2);

        assert_eq!(resources.get::<Program>().len(), 1);
        assert_eq!(resources.get::<FunctionBlock>().len(), 1);
    }
}
