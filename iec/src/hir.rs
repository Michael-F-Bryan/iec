use codespan::ByteSpan;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NodeId(u32);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Program {
    pub name: String,
    pub variables: Vec<Variable>,
    pub blocks: HashMap<NodeId, Block>,
    pub entry_point: NodeId,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CompilationUnit {
    pub types: HashMap<TypeId, Type>,
    pub programs: Vec<Program>,
    pub spans: HashMap<NodeId, ByteSpan>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Variable {
    pub name: String,
    pub ty: TypeId,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TypeId(u32);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Type {
    pub name: String,
}

impl<'a> From<&'a str> for Type {
    fn from(other: &'a str) -> Type {
        Type {
            name: String::from(other),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Block {}

#[derive(Default)]
pub(crate) struct Counter(u32);

impl Counter {
    pub(crate) fn next_node<I: Id>(&mut self) -> I {
        self.0 += 1;
        I::from_u32(self.0)
    }
}

pub(crate) trait Id: Sized {
    fn from_u32(n: u32) -> Self;
}

impl Id for TypeId {
    fn from_u32(n: u32) -> TypeId {
        TypeId(n)
    }
}

impl Id for NodeId {
    fn from_u32(n: u32) -> NodeId {
        NodeId(n)
    }
}
