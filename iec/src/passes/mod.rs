//! The internals for the `iec` compiler can be thought of as a series of
//! passes, where each pass does some processing on the provided input before
//! updating the world.

mod symbol_table;

pub use self::symbol_table::SymbolTableResolution;

use crate::ecs::FromResources;
use crate::ecs::{EntityGenerator, Resources};
use crate::hir::CompilationUnit;
use crate::Diagnostics;

#[derive(Debug)]
pub struct PassContext<'a> {
    pub diags: &'a mut Diagnostics,
}

/// The "system" part of your typical Entity-Component-System application.
///
/// Each [`Pass`] should be its own state-less chunk of logic, essentially a
/// fancy function for mutating the world.
pub trait Pass<'r> {
    type Arg: ?Sized;
    type Storage: FromResources<'r>;
    const DESCRIPTION: &'static str;

    /// Execute the pass.
    fn run(args: &Self::Arg, ctx: PassContext<'r>, storage: Self::Storage);
}

/// Process the provided AST and execute semantic analysis.
pub fn process(
    _ast: &iec_syntax::File,
    _diags: &mut Diagnostics,
) -> CompilationUnit {
    let mut resources = Resources::new();

    resources.register_singleton(EntityGenerator::new());

    unimplemented!()
}
