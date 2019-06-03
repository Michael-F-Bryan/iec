//! The compiler's high-level intermediate representation.

use specs::{Component, Entity, VecStorage};
use specs_derive::Component;
use std::iter::FromIterator;
use typename::TypeName;

#[derive(Debug, TypeName)]
pub struct CompilationUnit {
    pub symbols: Vec<Symbol>,
}

impl FromIterator<Symbol> for CompilationUnit {
    fn from_iter<I: IntoIterator<Item = Symbol>>(iter: I) -> CompilationUnit {
        CompilationUnit {
            symbols: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, TypeName, Component)]
#[storage(VecStorage)]
pub struct Program {
    pub name: String,
    pub variables: Vec<Entity>,
}

#[derive(Debug, Clone, PartialEq, TypeName, Component)]
#[storage(VecStorage)]
pub struct Function {
    pub name: String,
    pub variables: Vec<Entity>,
}

#[derive(Debug, Clone, PartialEq, TypeName, Component)]
#[storage(VecStorage)]
pub struct FunctionBlock {
    pub name: String,
    pub variables: Vec<Entity>,
}

#[derive(Debug, Clone, PartialEq, TypeName, Component)]
#[storage(VecStorage)]
pub struct Type {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, TypeName, Component)]
#[storage(VecStorage)]
pub struct Variable {
    /// The item this variable is defined in.
    pub parent: Symbol,
    /// The variable's type.
    pub ty: Entity,
    /// The variable's name, if one exists.
    pub name: Option<String>,
}

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