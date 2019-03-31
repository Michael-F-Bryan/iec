use crate::ecs::Resources;
use heapsize_derive::HeapSizeOf;
use typename::TypeName;

#[derive(Debug, TypeName, HeapSizeOf)]
pub struct CompilationUnit {
    pub resources: Resources,
}

#[derive(Debug, Clone, PartialEq, TypeName, HeapSizeOf)]
pub struct Program {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, TypeName, HeapSizeOf)]
pub struct Function {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, TypeName, HeapSizeOf)]
pub struct FunctionBlock {
    pub name: String,
}
