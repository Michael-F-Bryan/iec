//! The compiler's high-level intermediate representation.

use crate::ecs::{EntityId, Resources};
use heapsize_derive::HeapSizeOf;
use serde_derive::{Deserialize, Serialize};
use typename::TypeName;

#[derive(Debug, TypeName, HeapSizeOf)]
pub struct CompilationUnit {
    pub resources: Resources,
}

#[derive(Debug, Clone, PartialEq, TypeName, HeapSizeOf)]
pub struct Program {
    pub name: String,
    pub variables: Vec<EntityId>,
}

#[derive(Debug, Clone, PartialEq, TypeName, HeapSizeOf)]
pub struct Function {
    pub name: String,
    pub variables: Vec<EntityId>,
}

#[derive(Debug, Clone, PartialEq, TypeName, HeapSizeOf)]
pub struct FunctionBlock {
    pub name: String,
    pub variables: Vec<EntityId>,
}

#[derive(Debug, Clone, PartialEq, TypeName, HeapSizeOf)]
pub struct Type {
    pub name: String,
}

#[derive(
    Debug, Clone, PartialEq, TypeName, Serialize, Deserialize, HeapSizeOf,
)]
pub struct Variable {
    /// The item this variable is defined in.
    pub parent: Symbol,
    /// The variable's type.
    pub ty: EntityId,
    /// The variable's name, if one exists.
    pub name: Option<String>,
}

#[derive(
    Debug, Copy, Clone, PartialEq, TypeName, HeapSizeOf, Serialize, Deserialize,
)]
pub enum Symbol {
    Program(EntityId),
    Function(EntityId),
    FunctionBlock(EntityId),
    Type(EntityId),
}

impl From<Symbol> for EntityId {
    fn from(s: Symbol) -> EntityId {
        match s {
            Symbol::Program(id)
            | Symbol::Type(id)
            | Symbol::Function(id)
            | Symbol::FunctionBlock(id) => id,
        }
    }
}

/// A three address code instruction.
#[derive(
    Debug,
    TypeName,
    Copy,
    Clone,
    PartialEq,
    Eq,
    Hash,
    HeapSizeOf,
    Serialize,
    Deserialize,
)]
pub enum Instruction {}

#[derive(
    TypeName, Debug, Clone, PartialEq, HeapSizeOf, Serialize, Deserialize,
)]
pub struct BasicBlock {}
