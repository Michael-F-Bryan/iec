/// A macro for concisely generating syntax trees.
///
/// # Examples
///
/// ```rust
/// use iec_syntax::{Declaration, VarBlock, Program, Statement};
///
/// // declarations look identical to normal Structured Text
/// let decl: Declaration = iec_syntax::quote!(x: int);
/// assert_eq!(decl.ident.value, "x");
/// assert_eq!(decl.ty.value, "int");
///
/// // var blocks use "{" and "}" instead of var/end_var
/// let var: VarBlock = iec_syntax::quote!(var { x: int; });
/// assert_eq!(var.declarations.len(), 1);
///
/// // An empty program is equally as simple
/// let program: Program = iec_syntax::quote!(program asd {});
/// assert_eq!(program.name.value, "asd");
/// assert!(program.body.is_empty());
///
/// // programs can also have var blocks
/// let program_2: Program = iec_syntax::quote!(program asd { var {}});
/// assert_eq!(program_2.var_blocks.len(), 1);
///
/// // statements are followed by a semicolon
/// let assign: Statement = iec_syntax::quote!(meaning_of_life := 42;);
/// ```
#[macro_export]
macro_rules! quote {
    (program $name:ident {
        var { $($vars:tt)* }

        $($tail:tt)*
    }) => {
        $crate::Program {
            name: $crate::quote!(@IDENT $name),
            var_blocks: vec![$crate::quote!(var { $($vars) * })],
            body: Vec::new(),
            span: Default::default(),
        }
    };
    (program $name:ident {
        $($tail:tt)*
    }) => {
        $crate::Program {
            name: $crate::quote!(@IDENT $name),
            var_blocks: Vec::new(),
            body: Vec::new(),
            span: Default::default(),
        }
    };
    (var { $($tail:tt)* }) => {
        $crate::VarBlock {
            kind: $crate::VarBlockKind::Local,
            declarations: $crate::quote!($($tail)*),
            span: Default::default(),
        }
    };
    (function_block $name:ident { $($tail:tt)* }) => {
        $crate::FunctionBlock {
            name: $crate::quote!(@IDENT $name),
            var_blocks: Vec::new(),
            body: vec![],
            span: Default::default(),
        }
    };
    ($( $name:ident : $type:ident; )*) => {
        vec![
            $( $crate::quote!($name : $type) ),*
        ]
    };
    ($name:ident := $value:expr; ) => {
        $crate::Statement::Assignment($crate::Assignment {
            variable: $crate::quote!($name),
            value: $crate::Expression::Literal($crate::Literal {
                kind: $value.into(),
                span: Default::default(),
            }),
            span: Default::default(),
        })
    };
    ($name:ident : $type:ident) => {
        $crate::Declaration {
            ident: $crate::quote!(@IDENT $name),
            ty: $crate::quote!(@IDENT $type),
            span: Default::default(),
        }
    };
    ($ident:ident $( . $rest:ident )*) => {
        $crate::DottedIdentifier {
            pieces: vec![
                $crate::quote!(@IDENT $ident),
                $($crate::quote!(@IDENT $rest)),*
            ],
            span: Default::default(),
        }
    };
    (@IDENT $id:ident) => {
        $crate::Identifier {
            value: stringify!($id).to_string(),
            span: Default::default(),
        }
    };
}
