mod builtins;
mod symbol_discovery;
mod variable_discovery;

pub use self::builtins::Builtins;
pub use self::symbol_discovery::SymbolDiscovery;
pub use self::variable_discovery::VariableDiscovery;

use crate::hir::Symbol;
use crate::{CompilationUnit, Diagnostics};
use iec_syntax::File;
use slog::Logger;
use specs::{DispatcherBuilder, Join, ReadStorage, World};

pub fn initialize_systems(builder: &mut DispatcherBuilder<'_, '_>) {
    builder.add(Builtins, Builtins::NAME, &[]);
    builder.add(SymbolDiscovery, SymbolDiscovery::NAME, &[]);
    builder.add(
        VariableDiscovery,
        VariableDiscovery::NAME,
        &[Builtins::NAME, SymbolDiscovery::NAME],
    );
}

/// Process an Abstract Syntax Tree, applying typechecking and various other
/// checks/transformations as part of the compilation process.
pub fn process(file: File, diags: &mut Diagnostics, logger: &Logger) -> (World, CompilationUnit) {
    let _guard = slog_scope::set_global_logger(logger.clone());

    // set up the world and give it some initial resources
    let mut world = World::new();
    world.add_resource(file);
    world.add_resource(Diagnostics::new());
    world.add_resource(Logger::clone(logger));

    // create the dispatcher and wire up our systems
    let mut builder = DispatcherBuilder::new();
    initialize_systems(&mut builder);
    let mut dispatcher = builder.build();
    dispatcher.setup(&mut world.res);
    dispatcher.dispatch(&world.res);

    // propagate errors and extract the results
    assert!(world.res.has_value::<Diagnostics>());
    diags.extend(world.write_resource::<Diagnostics>().drain());
    let cu = resolve_compilation_unit(&world);

    (world, cu)
}

fn resolve_compilation_unit(world: &World) -> CompilationUnit {
    let entities = world.entities();
    let symbols = world.system_data::<ReadStorage<Symbol>>();

    (&entities, &symbols)
        .join()
        .map(|(entity, _)| entity)
        .collect()
}
