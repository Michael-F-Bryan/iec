//! The internals for the `iec` compiler can be thought of as a series of
//! passes, where each pass does some processing on the provided input before
//! updating the world.

use crate::ecs::FromResources;
use crate::Diagnostics;

/// The "system" part of your typical Entity-Component-System application.
///
/// Each [`Pass`] should be its own state-less chunk of logic, essentially a
/// fancy function for mutating the world.
pub trait Pass<'r> {
    type Context: ?Sized;
    type Storage: FromResources<'r>;
    const Description: &'static str;

    /// Execute the pass.
    fn run(
        ctx: &Self::Context,
        diags: &mut Diagnostics,
        storage: Self::Storage,
    );
}
