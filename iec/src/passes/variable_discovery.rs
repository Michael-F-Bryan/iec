use super::symbol_table::SymbolTable;
use super::{Pass, PassContext};
use crate::ecs::{Container, EntityId, ReadWrite, Singleton};
use crate::hir::{Symbol, Variable};
use crate::Diagnostics;
use codespan_reporting::{Diagnostic, Label};
use iec_syntax::Item;
use std::collections::HashMap;

pub enum VariableDiscovery {}

impl<'r> Pass<'r> for VariableDiscovery {
    type Arg = iec_syntax::File;
    type Storage = (Singleton<'r, SymbolTable>, ReadWrite<'r, Variable>);
    const DESCRIPTION: &'static str = "Resolve variable declarations in each program, function, or function block";

    fn run(args: &Self::Arg, ctx: PassContext<'r>, storage: Self::Storage) {
        let (symbol_table, mut variables) = storage;

        for item in &args.items {
            let (var_blocks, name) = match item {
                Item::Program(ref p) => (&p.var_blocks, &p.name.value),
                Item::Function(ref f) => (&f.var_blocks, &f.name.value),
                Item::FunctionBlock(ref fb) => (&fb.var_blocks, &fb.name.value),
            };
            let symbol = symbol_table.get(name)
                .expect("We should have found all symbols when constructing the symbol table");

            let got = resolve_variables(
                symbol,
                var_blocks,
                &mut variables,
                ctx.diags,
            );
        }
    }
}

fn resolve_variables(
    parent_scope: Symbol,
    blocks: &[iec_syntax::VarBlock],
    variables: &mut Container<Variable>,
    diags: &mut Diagnostics,
) -> Vec<EntityId> {
    let mut names = HashMap::new();
    let mut ids = Vec::new();

    for block in blocks {
        for decl in &block.declarations {
            let name = &decl.ident.value;
            let to_lower = name.to_lowercase();

            if let Some(&original_span) = names.get(&to_lower) {
                diags.push(
                    Diagnostic::new_error("Duplicate variable declarations")
                        .with_label(
                            Label::new_primary(decl.ident.span)
                                .with_message("Duplicate declared here"),
                        )
                        .with_label(
                            Label::new_secondary(original_span).with_message(
                                "Original variable was declared here",
                            ),
                        ),
                );
                continue;
            }

            names.insert(to_lower, decl.ident.span);
            let id = variables.insert(Variable {
                parent: parent_scope,
                name: name.clone(),
            });
            ids.push(id);
        }
    }

    ids
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn discover_some_variables() {}
}
