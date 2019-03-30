use crate::hir::{
    CompilationUnit, Counter, NodeId, Program, Type, TypeId, Variable,
};
use crate::Diagnostics;
use codespan_reporting::{Diagnostic, Label};
use std::collections::HashMap;

pub fn compile(
    program: &iec_syntax::Program,
    diags: &mut Diagnostics,
) -> CompilationUnit {
    let mut analyser = Analyser::new(diags);

    analyser.lower_program(program);

    analyser.finalise()
}

struct Analyser<'diag> {
    ids: Counter,
    diags: &'diag mut Diagnostics,
    compilation_unit: CompilationUnit,
}

impl<'diag> Analyser<'diag> {
    fn new(diags: &'diag mut Diagnostics) -> Analyser<'diag> {
        let mut an = Analyser {
            diags,
            ids: Counter::default(),
            compilation_unit: CompilationUnit::default(),
        };

        an.add_builtins();

        an
    }

    fn finalise(self) -> CompilationUnit {
        self.compilation_unit
    }

    fn add_builtins(&mut self) {
        self.add_type("int");
    }

    fn add_type<T: Into<Type>>(&mut self, ty: T) -> TypeId {
        let id = self.ids.next_node();
        self.compilation_unit.types.insert(id, ty.into());
        id
    }

    fn lookup_type(&self, name: &str) -> Option<TypeId> {
        let name = name.to_lowercase();
        self.compilation_unit
            .types
            .iter()
            .find(|(_, ty)| ty.name.to_lowercase() == name)
            .map(|(id, _)| *id)
    }

    fn lower_program(&mut self, original: &iec_syntax::Program) {
        let mut variables = HashMap::new();
        let blocks = HashMap::new();

        for var_block in &original.var_blocks {
            self.analyse_vars(var_block, &mut variables);
        }

        let dummy_node = self.ids.next_node();

        let p = Program {
            name: original.name.value.clone(),
            entry_point: dummy_node,
            blocks,
            variables,
        };

        let id = self.ids.next_node();
        self.compilation_unit.programs.insert(id, p);
    }

    fn analyse_vars(
        &mut self,
        block: &iec_syntax::VarBlock,
        vars: &mut HashMap<NodeId, Variable>,
    ) {
        for decl in &block.declarations {
            // check for duplicate definitions
            let possible_dupe = vars
                .iter()
                .find(|(_, value)| {
                    value.name.to_lowercase() == decl.ident.value.to_lowercase()
                })
                .map(|(id, _)| id);
            if let Some(id) = possible_dupe {
                let original_span = self.compilation_unit.spans[id];
                let d = Diagnostic::new_error("Duplicate variable definition")
                    .with_label(Label::new_primary(decl.span))
                    .with_label(
                        Label::new_secondary(original_span)
                            .with_message("First definition"),
                    );
                self.diags.push(d);

                continue;
            }

            match self.lookup_type(&decl.ty.value) {
                Some(id) => {
                    let var_id = self.ids.next_node();
                    vars.insert(
                        var_id,
                        Variable {
                            name: decl.ident.value.clone(),
                            ty: id,
                        },
                    );
                    self.compilation_unit.spans.insert(var_id, decl.span);
                }
                None => {
                    let d = Diagnostic::new_error("Unknown type")
                        .with_label(Label::new_primary(decl.ty.span));
                    self.diags.push(d);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use codespan_reporting::Severity;

    fn program() -> iec_syntax::Program {
        "PROGRAM main VAR i: int; END_VAR END_PROGRAM"
            .parse()
            .unwrap()
    }

    #[test]
    fn identify_variables() {
        let program = program();
        let mut diags = Diagnostics::new();
        let mut an = Analyser::new(&mut diags);

        an.lower_program(&program);

        let got = an.compilation_unit.programs.values().next().unwrap();

        assert_eq!(got.name, program.name.value);
        assert_eq!(got.variables.len(), 1);
        let var = got.variables.values().next().unwrap();

        assert_eq!(var.name, "i");
        assert_eq!(var.ty, an.lookup_type("int").unwrap());
    }

    #[test]
    fn detect_unknown_variable_types() {
        let mut diags = Diagnostics::new();
        let mut an = Analyser::new(&mut diags);
        let block = iec_syntax::quote!(var { x: doesnt_exist; });

        let mut got = HashMap::new();

        an.analyse_vars(&block, &mut got);

        assert!(got.is_empty());
        assert_eq!(an.diags.diagnostics().len(), 1);
        let diag = &an.diags.diagnostics()[0];
        assert_eq!(diag.severity, Severity::Error);
    }

    #[test]
    fn detect_duplicate_definitions() {
        let block = iec_syntax::quote!(var { x: int; x: int; });
        let mut diags = Diagnostics::new();
        let mut an = Analyser::new(&mut diags);
        let mut got = HashMap::new();

        an.analyse_vars(&block, &mut got);

        assert_eq!(got.len(), 1);

        assert_eq!(an.diags.diagnostics().len(), 1);
        let diag = &an.diags.diagnostics()[0];
        assert_eq!(diag.severity, Severity::Error);
    }
}
