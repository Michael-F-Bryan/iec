//! The internals for the `iec` compiler can be thought of as a series of
//! passes, where each pass does some processing on the provided input before
//! updating the world.

pub mod symbol_table;

pub use self::symbol_table::SymbolTableResolution;

use crate::ecs::FromResources;
use crate::ecs::Resources;
use crate::hir::CompilationUnit;
use crate::Diagnostics;

/// Contextual information given to each pass.
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

pub fn run_pass<'r, P: Pass<'r>>(
    r: &'r mut Resources,
    arg: &'r P::Arg,
    diags: &'r mut Diagnostics,
) {
    P::Storage::ensure_registered(r);

    let storage = P::Storage::from_resources(r);
    let ctx = PassContext { diags };

    P::run(arg, ctx, storage);
}

/// Process the provided AST and execute semantic analysis.
pub fn process(
    ast: &iec_syntax::File,
    diags: &mut Diagnostics,
) -> CompilationUnit {
    let mut resources = Resources::new();

    run_pass::<SymbolTableResolution>(&mut resources, ast, diags);

    CompilationUnit { resources }
}
