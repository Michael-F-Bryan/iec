//! The compiler's high-level intermediate representation.

use specs::{Component, Entity, VecStorage};
use specs_derive::Component;
use std::iter::FromIterator;
use typename::TypeName;

#[derive(Debug, TypeName)]
pub struct CompilationUnit {
    pub symbols: Vec<Entity>,
}

impl FromIterator<Entity> for CompilationUnit {
    fn from_iter<I: IntoIterator<Item = Entity>>(iter: I) -> CompilationUnit {
        CompilationUnit {
            symbols: iter.into_iter().collect(),
        }
    }
}

/// A marker indicating this entity is a program.
#[derive(Debug, Clone, PartialEq, TypeName, Component)]
#[storage(VecStorage)]
pub struct Program;

/// A marker indicating this entity is a function.
#[derive(Debug, Clone, PartialEq, TypeName, Component)]
#[storage(VecStorage)]
pub struct Function;

/// A marker indicating this entity is a function block.
#[derive(Debug, Clone, PartialEq, TypeName, Component)]
#[storage(VecStorage)]
pub struct FunctionBlock;

#[derive(Debug, Clone, PartialEq, TypeName, Component)]
#[storage(VecStorage)]
pub struct Type;

/// The set of variables defined within a particular scope.
#[derive(Debug, Clone, PartialEq, TypeName, Component)]
#[storage(VecStorage)]
pub struct Variables {
    pub scope: Entity,
    pub variables: Vec<Entity>,
}

#[derive(Debug, Clone, PartialEq, TypeName, Component)]
#[storage(VecStorage)]
pub struct Variable {
    /// The item this variable is defined in.
    pub parent: Entity,
    /// The variable's type.
    pub ty: Entity,
    /// The variable's name, if one exists.
    pub name: Option<String>,
}

/// Something with a globally accessible name.
#[derive(Debug, Clone, PartialEq, TypeName, Component)]
#[storage(VecStorage)]
pub struct Symbol {
    pub name: String,
}

/// A three address code instruction.
#[derive(Debug, TypeName, Copy, Clone, PartialEq, Eq, Hash, Component)]
#[storage(VecStorage)]
pub enum Instruction {}

#[derive(TypeName, Debug, Clone, PartialEq, Component)]
#[storage(VecStorage)]
pub struct BasicBlock {}
