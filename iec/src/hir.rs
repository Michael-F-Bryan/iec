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
}

#[derive(Debug, Clone, PartialEq, TypeName, HeapSizeOf)]
pub struct FunctionBlock {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, TypeName, HeapSizeOf)]
pub struct Type {
    pub name: String,
}

#[derive(
    Debug, Clone, PartialEq, TypeName, Serialize, Deserialize, HeapSizeOf,
)]
pub struct Variable {
    pub parent: Symbol,
    pub ty: EntityId,
    pub name: String,
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
