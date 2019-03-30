use crate::ecs::{Component, Container, Resources};
use std::cell::{Ref, RefMut};
use std::ops::{Deref, DerefMut};
use typename::TypeName;

// imported so rustdoc can wire up links correctly
#[allow(unused_imports)]
use super::Pass;

/// An adaptor trait for retrieving a particular [`Component`] container from
/// the world.
///
/// This enables nice things like passing a tuple of [`Component`]s to a
/// [`Pass`].
pub trait FromResources<'r>: Sized {
    fn from_resources(r: &'r Resources) -> Self;
}

/// A read-only reference to a [`Container`] of [`Component`]s.
#[derive(Debug, TypeName)]
pub struct Read<'r, C: Component>(Ref<'r, Container<C>>);

impl<'r, C: Component> FromResources<'r> for Read<'r, C> {
    fn from_resources(r: &'r Resources) -> Self {
        Read(r.get())
    }
}

impl<'r, C: Component> Deref for Read<'r, C> {
    type Target = Container<C>;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

/// A reference to a [`Container`] of [`Component`]s which supports mutation.
#[derive(Debug, TypeName)]
pub struct ReadWrite<'r, C: Component>(RefMut<'r, Container<C>>);

impl<'r, C: Component> FromResources<'r> for ReadWrite<'r, C> {
    fn from_resources(r: &'r Resources) -> Self {
        ReadWrite(r.get_mut())
    }
}

impl<'r, C: Component> Deref for ReadWrite<'r, C> {
    type Target = Container<C>;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl<'r, C: Component> DerefMut for ReadWrite<'r, C> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.deref_mut()
    }
}

/// An immutable reference to a singleton component.
#[derive(Debug, TypeName)]
pub struct Singleton<'r, T: Component>(Ref<'r, T>);

impl<'r, T: Component> FromResources<'r> for Singleton<'r, T> {
    fn from_resources(r: &'r Resources) -> Self {
        Singleton(r.get_singleton())
    }
}

impl<'r, C: Component> Deref for Singleton<'r, C> {
    type Target = C;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

/// A mutable reference to a singleton component.
#[derive(Debug, TypeName)]
pub struct SingletonMut<'r, T: Component>(RefMut<'r, T>);

impl<'r, T: Component> FromResources<'r> for SingletonMut<'r, T> {
    fn from_resources(r: &'r Resources) -> Self {
        SingletonMut(r.get_singleton_mut())
    }
}

impl<'r, C: Component> Deref for SingletonMut<'r, C> {
    type Target = C;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl<'r, C: Component> DerefMut for SingletonMut<'r, C> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.deref_mut()
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
