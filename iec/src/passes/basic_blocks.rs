use super::symbol_table::SymbolTable;
use super::{Pass, PassContext};
use crate::ecs::{Container, EntityId, Read, ReadWrite, Singleton};
use crate::hir::{BasicBlock, Instruction, Program, Symbol, Type, Variable};
use crate::Diagnostics;
use iec_syntax::Item;
use typename::TypeName;

#[derive(TypeName)]
pub enum BasicBlocks {}

impl<'r> Pass<'r> for BasicBlocks {
    type Arg = iec_syntax::File;
    type Storage = (
        ReadWrite<'r, Program>,
        Read<'r, Variable>,
        Singleton<'r, SymbolTable>,
        Read<'r, Type>,
        ReadWrite<'r, Instruction>,
        ReadWrite<'r, BasicBlock>,
    );
    const DESCRIPTION: &'static str = "Convert item bodies into basic blocks";

    fn run(ast: &Self::Arg, ctx: &mut PassContext<'_>, storage: Self::Storage) {
        let (mut programs, variables, symbols, types, instructions, blocks) =
            storage;

        for item in &ast.items {
            let (body, name) = match item {
                Item::Program(ref p) => (&p.body, &p.name.value),
                _ => unimplemented!(),
            };
            let symbol = symbols
                .get(name)
                .expect("the symbol table pass ensures this exists");

            let entry_block = to_basic_blocks(
                symbol,
                body,
                &variables,
                &types,
                &mut ctx.diags,
            );
        }
    }
}

fn to_basic_blocks(
    parent: Symbol,
    body: &[iec_syntax::Statement],
    variables: &Container<Variable>,
    types: &Container<Type>,
    diags: &mut Diagnostics,
) -> EntityId {
    unimplemented!()
}
