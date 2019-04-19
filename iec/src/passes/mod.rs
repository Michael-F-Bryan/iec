//! The internals for the `iec` compiler can be thought of as a series of
//! passes, where each pass does some processing on the provided input before
//! updating the world.

pub mod basic_blocks;
pub mod register_builtins;
pub mod symbol_table;
pub mod variable_discovery;

pub use self::basic_blocks::BasicBlocks;
pub use self::register_builtins::RegisterBuiltins;
pub use self::symbol_table::SymbolTableResolution;
pub use self::variable_discovery::VariableDiscovery;

use crate::ecs::FromResources;
use crate::ecs::Resources;
use crate::hir::CompilationUnit;
use crate::Diagnostics;
use heapsize::HeapSizeOf;
use slog::{Discard, Logger};
use std::time::Instant;
use typename::TypeName;

/// The "system" part of your typical Entity-Component-System application.
///
/// Each [`Pass`] should be its own state-less chunk of logic, essentially a
/// fancy function for updating the world.
pub trait Pass<'r>: TypeName {
    /// Extra arguments passed into the [`Pass<'_>`] from the outside.
    type Arg: ?Sized;
    /// State which should be retrieved from [`Resources`] to be updated/read by
    /// the [`Pass`<'_>].
    type Storage: FromResources<'r>;
    /// A one-line description of what the pass is meant to do.
    const DESCRIPTION: &'static str;

    /// Execute the pass.
    fn run(args: &Self::Arg, ctx: &mut PassContext<'_>, storage: Self::Storage);
}

pub fn run_pass<'r, P: Pass<'r>>(
    r: &'r mut Resources,
    arg: &'r P::Arg,
    ctx: &mut PassContext<'_>,
) {
    let mut ctx = ctx.with(slog::o!("pass" => P::type_name()));
    slog::debug!(ctx.logger, "Pass started";
        "description" => P::DESCRIPTION,
        "resource-usage" => r.heap_size_of_children());
    let start = Instant::now();

    P::Storage::ensure_registered(r);
    let storage = P::Storage::from_resources(r);
    P::run(arg, &mut ctx, storage);

    let duration = Instant::now() - start;
    slog::debug!(ctx.logger, "Pass complete";
        "execution-time" => format_args!("{}.{:06}s", duration.as_secs(), duration.subsec_micros()),
        "resource-usage" => r.heap_size_of_children());
}

/// Process the provided AST and execute semantic analysis.
pub fn process(
    ast: &iec_syntax::File,
    ctx: &mut PassContext<'_>,
) -> CompilationUnit {
    let mut resources = Resources::new();

    run_pass::<RegisterBuiltins>(&mut resources, &(), ctx);
    run_pass::<SymbolTableResolution>(&mut resources, ast, ctx);
    run_pass::<VariableDiscovery>(&mut resources, ast, ctx);
    run_pass::<BasicBlocks>(&mut resources, ast, ctx);

    CompilationUnit { resources }
}

/// Contextual information given to each pass.
#[derive(Debug)]
pub struct PassContext<'a> {
    pub diags: &'a mut Diagnostics,
    pub logger: Logger,
}

impl<'a> PassContext<'a> {
    pub fn new_nop_logger(diags: &'a mut Diagnostics) -> PassContext<'a> {
        PassContext {
            diags,
            logger: Logger::root(Discard, slog::o!()),
        }
    }

    pub fn with<T>(&mut self, pairs: slog::OwnedKV<T>) -> PassContext<'_>
    where
        T: slog::SendSyncRefUnwindSafeKV + 'static,
    {
        PassContext {
            diags: self.diags,
            logger: self.logger.new(pairs),
        }
    }
}
