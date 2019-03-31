use super::{Pass, PassContext};

pub struct SymbolTableResolution;

impl<'r> Pass<'r> for SymbolTableResolution {
    type Arg = iec_syntax::File;
    type Storage = ();
    const DESCRIPTION: &'static str = "Find all know identifiers";

    fn run(arg: &Self::Arg, ctx: PassContext<'r>, storage: Self::Storage) {
        unimplemented!()
    }
}
