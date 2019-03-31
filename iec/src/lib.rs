//! The "*middle-end*" of the `iec` compiler.
//!
//! The `iec` crate's main job is to process the *Abstract Syntax Tree* parsed
//! by the frontend ([`iec_syntax`]), turn it into a more compiler-friendly
//! format, apply typechecking and other semantic analyses, then pass it over
//! to a backend (e.g. [`cranelift`]) for code generation.
//!
//! The compiler takes a lot of inspiration from the *Entity-Component-System*
//! (ECS) architecture used in modern games. Perhaps unsurprisingly, this
//! architecture turns out to be equally well suited to compilers.
//!
//! > *Note:* If you aren't familiar with ECS, the [`specs`] crate is a very
//! > well designed ECS implementation with a lot of good tutorials and
//! > documentation. Feel free to browse that project if you want a more
//! > in-depth understanding of how an ECS works.
//!
//! [`specs`]: https://github.com/slide-rs/specs
//! [`cranelift`]: https://github.com/CraneStation/cranelift

mod diagnostics;
pub mod ecs;
pub mod hir;
pub mod passes;

pub use crate::diagnostics::Diagnostics;
pub use crate::ecs::EntityId;
pub use crate::hir::CompilationUnit;
pub use crate::passes::process;
