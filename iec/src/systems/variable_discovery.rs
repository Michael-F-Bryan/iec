use crate::hir::{Symbol, Type, Variable, Variables};
use crate::Diagnostics;
use codespan_reporting::{Diagnostic, Label};
use iec_syntax::Item;
use specs::{Entities, Entity, Join, ReadExpect, ReadStorage, System, Write, WriteStorage};
use std::collections::HashMap;

/// Visits each function, program, or function block and translates their
/// variable declarations.
#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct VariableDiscovery;

impl VariableDiscovery {
    pub const NAME: &'static str = "variable-discovery";
}

impl<'a> System<'a> for VariableDiscovery {
    type SystemData = (
        Entities<'a>,
        ReadExpect<'a, iec_syntax::File>,
        Write<'a, Diagnostics>,
        ReadStorage<'a, Symbol>,
        WriteStorage<'a, Variable>,
        WriteStorage<'a, Variables>,
        ReadStorage<'a, Type>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, file, mut diags, symbols, mut variable, mut variables, types) = data;

        for item in &file.items {
            if let Some((ent, vars)) = lookup(item, &symbols, &entities) {
                resolve_variables(
                    ent,
                    &entities,
                    vars,
                    &mut variable,
                    &mut variables,
                    &symbols,
                    &types,
                    &mut diags,
                );
            }
        }
    }
}

fn lookup<'item>(
    item: &'item Item,
    symbols: &ReadStorage<'_, Symbol>,
    ents: &Entities<'_>,
) -> Option<(Entity, &'item [iec_syntax::VarBlock])> {
    let (var_blocks, name) = match item {
        Item::Program(ref p) => (&p.var_blocks, &p.name.value),
        Item::Function(ref f) => (&f.var_blocks, &f.name.value),
        Item::FunctionBlock(ref fb) => (&fb.var_blocks, &fb.name.value),
    };

    for (ent, symbol) in (ents, symbols).join() {
        if symbol.name == *name {
            return Some((ent, var_blocks));
        }
    }

    None
}

fn resolve_variables(
    parent_scope: Entity,
    entities: &Entities<'_>,
    blocks: &[iec_syntax::VarBlock],
    variable_store: &mut WriteStorage<'_, Variable>,
    variables: &mut WriteStorage<'_, Variables>,
    symbols: &ReadStorage<'_, Symbol>,
    types: &ReadStorage<'_, Type>,
    diags: &mut Diagnostics,
) {
    let mut found = Vec::new();
    let mut names = HashMap::new();

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
                            Label::new_secondary(original_span)
                                .with_message("Original variable was declared here"),
                        ),
                );
                continue;
            }

            let type_name = decl.ty.value.to_lowercase();
            let type_entity = match (entities, symbols)
                .join()
                .find(|(_, sym)| sym.name.to_lowercase() == type_name)
                .map(|(ent, _)| ent)
            {
                Some(ent) => ent,
                None => {
                    diags.push(
                        Diagnostic::new_error("Unknown type")
                            .with_label(Label::new_primary(decl.ty.span)),
                    );
                    continue;
                }
            };

            if !types.contains(type_entity) {
                diags.push(
                    Diagnostic::new_error("Expected the name of a type")
                        .with_label(Label::new_primary(decl.ty.span)),
                );
                continue;
            }

            names.insert(to_lower, decl.ident.span);

            let id = entities
                .build_entity()
                .with(
                    Variable {
                        parent: parent_scope,
                        ty: type_entity,
                        name: Some(name.clone()),
                    },
                    variable_store,
                )
                .build();
            found.push(id);
        }
    }

    entities
        .build_entity()
        .with(
            Variables {
                scope: parent_scope,
                variables: found,
            },
            variables,
        )
        .build();
}
