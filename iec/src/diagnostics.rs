use std::iter::Extend;
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

    pub fn drain<'this>(&'this mut self) -> impl Iterator<Item=Diagnostic> + 'this {
        self.0.drain(..)
    }
}

impl Extend<Diagnostic> for Diagnostics {
    fn extend<I: IntoIterator<Item=Diagnostic>>(&mut self, items: I) {
        for item in items {
            self.push(item);
        }
    }
}

impl IntoIterator for Diagnostics {
    type Item = Diagnostic;
    type IntoIter = <Vec<Diagnostic> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}