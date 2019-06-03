use crate::hir::Symbol;
use codespan_reporting::{Diagnostic, Label};
use std::collections::HashMap;
use typename::TypeName;

/// A cache for looking up a component based on its identifier.
#[derive(Debug, Default, Clone, PartialEq, TypeName)]
pub struct SymbolTable(HashMap<String, Symbol>);

impl SymbolTable {
    pub fn insert(&mut self, name: &str, sym: Symbol) {
        self.0.insert(name.to_lowercase(), sym);
    }

    pub fn get(&self, name: &str) -> Option<Symbol> {
        let name = name.to_lowercase();
        self.0.get(&name).cloned()
    }

    pub fn inner(&self) -> &HashMap<String, Symbol> {
        &self.0
    }

    pub fn inner_mut(&mut self) -> &mut HashMap<String, Symbol> {
        &mut self.0
    }

    pub fn check_for_duplicate_ident(&self, ident: &iec_syntax::Identifier) -> Option<Diagnostic> {
        if self.get(&ident.value).is_none() {
            None
        } else {
            Some(
                Diagnostic::new_error("Name is already declared").with_label(
                    Label::new_primary(ident.span).with_message("Duplicate declared here"),
                ),
            )
        }
    }
}
