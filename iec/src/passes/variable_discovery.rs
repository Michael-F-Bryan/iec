use super::symbol_table::SymbolTable;
use super::{Pass, PassContext};
use crate::ecs::{Container, EntityId, ReadWrite, Singleton};
use crate::hir::{Function, FunctionBlock, Program, Symbol, Variable};
use crate::Diagnostics;
use codespan_reporting::{Diagnostic, Label};
use iec_syntax::Item;
use std::collections::HashMap;
use typename::TypeName;

#[derive(TypeName)]
pub enum VariableDiscovery {}

impl<'r> Pass<'r> for VariableDiscovery {
    type Arg = iec_syntax::File;
    type Storage = (
        Singleton<'r, SymbolTable>,
        ReadWrite<'r, Variable>,
        ReadWrite<'r, Program>,
        ReadWrite<'r, Function>,
        ReadWrite<'r, FunctionBlock>,
    );
    const DESCRIPTION: &'static str = "Resolve variable declarations in each program, function, or function block";

    fn run(
        args: &Self::Arg,
        ctx: &mut PassContext<'_>,
        storage: Self::Storage,
    ) {
        let (
            symbol_table,
            mut variables,
            mut programs,
            mut functions,
            mut function_blocks,
        ) = storage;

        for item in &args.items {
            let (var_blocks, name) = match item {
                Item::Program(ref p) => (&p.var_blocks, &p.name.value),
                Item::Function(ref f) => (&f.var_blocks, &f.name.value),
                Item::FunctionBlock(ref fb) => (&fb.var_blocks, &fb.name.value),
            };
            let symbol = symbol_table.get(name)
                .expect("We should have found all symbols when constructing the symbol table");

            let variable_ids = resolve_variables(
                symbol,
                &symbol_table,
                var_blocks,
                &mut variables,
                ctx.diags,
            );

            slog::debug!(ctx.logger, "Analysed item";
                "name" => name,
                "symbol" => format_args!("{:?}", symbol),
                "variable-count" => variable_ids.len());

            const ERR_MSG: &str =
                "The item should have been added during symbol table discovery";

            match symbol {
                Symbol::Program(p) => {
                    let p = programs.get_mut(p).expect(ERR_MSG);
                    p.variables = variable_ids;
                }
                Symbol::Function(f) => {
                    let f = functions.get_mut(f).expect(ERR_MSG);
                    f.variables = variable_ids;
                }
                Symbol::FunctionBlock(fb) => {
                    let fb = function_blocks.get_mut(fb).expect(ERR_MSG);
                    fb.variables = variable_ids;
                }
                Symbol::Type(_) => unreachable!(),
            }
        }
    }
}

fn resolve_variables(
    parent_scope: Symbol,
    symbol_table: &SymbolTable,
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

            let ty = symbol_table.get(&decl.ty.value);

            let type_id = match ty {
                Some(Symbol::Type(id)) => id,
                Some(_) => {
                    diags.push(
                        Diagnostic::new_error("Expected the name of a type")
                            .with_label(Label::new_primary(decl.ty.span)),
                    );
                    continue;
                }
                None => {
                    diags.push(
                        Diagnostic::new_error("Unknown type")
                            .with_label(Label::new_primary(decl.ty.span)),
                    );
                    continue;
                }
            };

            names.insert(to_lower, decl.ident.span);
            let id = variables.insert(Variable {
                parent: parent_scope,
                ty: type_id,
                name: Some(name.clone()),
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
    fn discover_some_variables() {
        let block = iec_syntax::quote!(var { x: int; });
        let mut variables = Container::default();
        let mut diags = Diagnostics::new();
        let mut symbols = SymbolTable::default();

        symbols.insert("int", Symbol::Type(EntityId::default()));

        let got = resolve_variables(
            Symbol::Function(EntityId::default()),
            &symbols,
            &[block],
            &mut variables,
            &mut diags,
        );

        assert_eq!(diags.len(), 0);
        assert_eq!(variables.len(), 1);
        assert_eq!(got.len(), 1);

        let (id, var) = variables.iter().next().unwrap();

        assert!(got.contains(&id));
        assert_eq!(var.name, Some(String::from("x")));
    }

    #[test]
    fn duplicate_variable_declarations() {
        let block = iec_syntax::quote!(var { x: int; X: int; });
        let mut variables = Container::default();
        let mut diags = Diagnostics::new();
        let mut symbols = SymbolTable::default();
        symbols.insert("int", Symbol::Type(EntityId::default()));

        let got = resolve_variables(
            Symbol::Function(EntityId::default()),
            &symbols,
            &[block],
            &mut variables,
            &mut diags,
        );

        assert!(diags.has_errors());
        assert_eq!(got.len(), 1);
        assert_eq!(variables.len(), 1);
    }

    #[test]
    fn unknown_type() {
        let block = iec_syntax::quote!(var { x: int; y: string; });
        let mut variables = Container::default();
        let mut diags = Diagnostics::new();
        let symbols = SymbolTable::default();

        let got = resolve_variables(
            Symbol::Function(EntityId::default()),
            &symbols,
            &[block],
            &mut variables,
            &mut diags,
        );

        assert!(diags.has_errors());
        assert_eq!(diags.len(), 2);
        assert!(got.is_empty());
        assert!(variables.is_empty());
    }
}
