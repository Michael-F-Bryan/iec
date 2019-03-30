use cranelift::codegen::ir::{ExternalName, Function, Signature};
use cranelift::codegen::isa::CallConv;
use cranelift::frontend::{FunctionBuilder, FunctionBuilderContext};
use iec_syntax::Program;

struct Codegen {
    ctx: FunctionBuilderContext,
}

impl Codegen {
    pub fn new() -> Codegen {
        Codegen {
            ctx: FunctionBuilderContext::new(),
        }
    }

    fn generate_program(&mut self, program: &Program) -> Function {
        let sig = Signature::new(CallConv::SystemV);
        let mut func = Function::with_name_signature(
            ExternalName::testcase(&program.name.value),
            sig,
        );

        let builder = FunctionBuilder::new(&mut func, &mut self.ctx);

        func
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_an_empty_program() {
        let prog = iec_syntax::quote!(program main {});

        let got = Codegen::new().generate_program(&prog);

        println!("{}", got);
        panic!();
    }
}
