use crate::hir::{
    CompilationUnit, Counter, NodeId, Program, Type, TypeId, Variable,
};
use codespan::ByteSpan;
use codespan_reporting::{Diagnostic, Label, Severity};
use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct Diagnostics(Vec<Diagnostic>);

impl Diagnostics {
    pub fn new() -> Diagnostics {
        Diagnostics::default()
    }

    pub fn push(&mut self, diag: Diagnostic) {
        self.0.push(diag);
    }

    fn has(&self, severity: Severity) -> bool {
        self.0.iter().any(|diag| diag.severity >= severity)
    }

    pub fn has_errors(&self) -> bool {
        self.has(Severity::Error)
    }

    pub fn diagnostics(&self) -> &[Diagnostic] {
        &self.0
    }
}

pub fn compile(
    program: &iec_syntax::Program,
    diags: &mut Diagnostics,
) -> CompilationUnit {
    let mut analyser = Analyser::new(diags);

    let program = analyser.lower_program(program);

    CompilationUnit {
        programs: vec![program],
        types: analyser.types,
        spans: analyser.spans,
    }
}

struct Analyser<'diag> {
    ids: Counter,
    diags: &'diag mut Diagnostics,
    spans: HashMap<NodeId, ByteSpan>,
    types: HashMap<TypeId, Type>,
}

impl<'diag> Analyser<'diag> {
    fn new(diags: &'diag mut Diagnostics) -> Analyser<'diag> {
        let mut an = Analyser {
            diags,
            ids: Counter::default(),
            spans: HashMap::default(),
            types: HashMap::default(),
        };

        an.add_builtins();

        an
    }

    fn add_builtins(&mut self) {
        self.add_type("int");
    }

    fn add_type<T: Into<Type>>(&mut self, ty: T) -> TypeId {
        let id = self.ids.next_node();
        self.types.insert(id, ty.into());
        id
    }

    fn lookup_type(&self, name: &str) -> Option<TypeId> {
        let name = name.to_lowercase();
        self.types
            .iter()
            .find(|(_, ty)| ty.name.to_lowercase() == name)
            .map(|(id, _)| *id)
    }

    fn lower_program(&mut self, original: &iec_syntax::Program) -> Program {
        let mut variables = Vec::new();
        let blocks = HashMap::new();

        if let Some(vars) = original.var.as_ref() {
            self.analyse_vars(vars, &mut variables);
        }

        let dummy_node = self.ids.next_node();

        Program {
            name: original.name.value.clone(),
            entry_point: dummy_node,
            blocks,
            variables,
        }
    }

    fn analyse_vars(
        &mut self,
        block: &iec_syntax::VarBlock,
        vars: &mut Vec<Variable>,
    ) {
        for decl in &block.declarations {
            // check for duplicate definitions

            //
            match self.lookup_type(&decl.ty.value) {
                Some(id) => {
                    vars.push(Variable {
                        name: decl.ident.value.clone(),
                        ty: id,
                    });
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
    use iec_syntax::{Declaration, Identifier, VarBlock};

    fn program() -> iec_syntax::Program {
        let src = "PROGRAM main VAR i: int; END_VAR END_PROGRAM";
        iec_syntax::parse(src).unwrap()
    }

    #[test]
    fn identify_variables() {
        let program = program();
        let mut diags = Diagnostics::new();
        let mut an = Analyser::new(&mut diags);

        let got = an.lower_program(&program);

        assert_eq!(got.name, program.name.value);
        assert_eq!(got.variables.len(), 1);
        let var = &got.variables[0];

        assert_eq!(var.name, "i");
        assert_eq!(var.ty, an.lookup_type("int").unwrap());
    }

    #[test]
    fn detect_unknown_variable_types() {
        let mut diags = Diagnostics::new();
        let mut an = Analyser::new(&mut diags);
        let block = VarBlock {
            declarations: vec![Declaration {
                ident: Identifier {
                    value: String::from("asd"),
                    span: ByteSpan::default(),
                },
                ty: Identifier {
                    value: String::from("qwerty"),
                    span: ByteSpan::default(),
                },
                span: ByteSpan::default(),
            }],
            span: ByteSpan::default(),
        };

        let mut got = Vec::new();

        an.analyse_vars(&block, &mut got);

        assert!(got.is_empty());
        assert_eq!(an.diags.diagnostics().len(), 1);
        let diag = &an.diags.diagnostics()[0];
        assert_eq!(diag.severity, Severity::Error);
    }
}
