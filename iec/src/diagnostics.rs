use codespan_reporting::{Diagnostic, Severity};

/// A collection of user diagnostics.
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

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
