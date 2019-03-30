//! The internals for the `iec` compiler can be thought of as a series of
//! passes, where each pass does some processing on the provided input before
//! updating the world.

use crate::ecs::{Component, Container, Resources};
use crate::Diagnostics;
use std::cell::{Ref, RefMut};

/// The "system" part of your typical Entity-Component-System application.
///
/// Each [`Pass`] should be its own state-less chunk of logic, essentially a
/// fancy function for mutating the world.
pub trait Pass<'r> {
    type Context: ?Sized;
    type Storage: FromResources<'r>;

    /// Execute the pass.
    fn run(
        ctx: &Self::Context,
        diags: &mut Diagnostics,
        storage: Self::Storage,
    );
}

/// An adaptor trait for retrieving a particular [`Component`] container from
/// the world.
///
/// This enables nice things like passing a tuple of [`Component`]s to a
/// [`Pass`].
pub trait FromResources<'r>: Sized {
    fn from_resources(r: &'r Resources) -> Self;
}

impl<'r, C: Component> FromResources<'r> for Ref<'r, Container<C>> {
    fn from_resources(r: &'r Resources) -> Self {
        r.get()
    }
}

impl<'r, C: Component> FromResources<'r> for RefMut<'r, Container<C>> {
    fn from_resources(r: &'r Resources) -> Self {
        r.get_mut()
    }
}

macro_rules! tuple_from_resource {
    ($($letter:ident),*) => {
        impl<'r, $( $letter ),* > FromResources<'r> for ( $( $letter ),* )
        where
            $(
                $letter : FromResources<'r>,
            )*
        {
            fn from_resources(r: &'r Resources) -> Self {
                ( $( $letter::from_resources(r) ),* )
            }
        }
    };
}

tuple_from_resource!(A, B);
tuple_from_resource!(A, B, C);
tuple_from_resource!(A, B, C, D);
tuple_from_resource!(A, B, C, D, E);
tuple_from_resource!(A, B, C, D, E, F);
