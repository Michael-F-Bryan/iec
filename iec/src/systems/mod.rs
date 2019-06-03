mod builtins;

pub use self::builtins::Builtins;

use crate::hir::Symbol;
use crate::{CompilationUnit, Diagnostics};
use iec_syntax::File;
use slog::Logger;
use specs::{DispatcherBuilder, Join, ReadStorage, World, System};

pub fn initialize_systems(_builder: &mut DispatcherBuilder<'_, '_>) {}

/// Process an Abstract Syntax Tree, applying typechecking and various other
/// checks/transformations as part of the compilation process.
pub fn process(file: File, diags: &mut Diagnostics, logger: &Logger) -> (World, CompilationUnit) {
    let _guard = slog_scope::set_global_logger(logger.clone());

    // set up the world and give it some initial resources
    let mut world = World::new();
    world.add_resource(file);
    world.add_resource(Diagnostics::new());

    // create the dispatcher and wire up our systems
    let mut builder = DispatcherBuilder::new();
    initialize_systems(&mut builder);

    builder.build().dispatch(&world.res);

    // propagate errors and extract the results
    diags.extend(world.write_resource::<Diagnostics>().drain());
    let cu = resolve_compilation_unit(&world);

    (world, cu)
}

fn resolve_compilation_unit(world: &World) -> CompilationUnit {
    let entities = world.entities();
    let symbols = world.system_data::<ReadStorage<Symbol>>();

    (&entities, &symbols)
        .join()
        .map(|(_, symbol)| symbol)
        .cloned()
        .collect()
}

pub(crate) trait Pass<'a>: System<'a> {
    const NAME: &'static str;
}