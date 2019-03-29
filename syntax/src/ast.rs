use codespan::ByteSpan;
use serde_derive::{Deserialize, Serialize};
use std::any::Any;

pub trait AstNode: Any {
    fn span(&self) -> ByteSpan;
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct File {
    pub items: Vec<Item>,
    pub span: ByteSpan,
}

sum_type::sum_type! {
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub enum Item {
        Program,
        Function,
        FunctionBlock,
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Function {
    pub name: Identifier,
    pub return_value: Identifier,
    pub var_blocks: Vec<VarBlock>,
    pub body: Vec<Statement>,
    pub span: ByteSpan,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FunctionBlock {
    pub name: Identifier,
    pub var_blocks: Vec<VarBlock>,
    pub body: Vec<Statement>,
    pub span: ByteSpan,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Program {
    pub name: Identifier,
    pub var_blocks: Vec<VarBlock>,
    pub body: Vec<Statement>,
    pub span: ByteSpan,
}

sum_type::sum_type! {
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub enum Statement {
        Assignment,
        FunctionCall,
        ForLoop,
        WhileLoop,
        RepeatLoop,
        Exit,
        Return,
        IfStatement,
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IfStatement {
    pub condition: Expression,
    pub body: Vec<Statement>,
    pub span: ByteSpan,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Exit {
    pub span: ByteSpan,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Return {
    pub span: ByteSpan,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Identifier {
    pub value: String,
    pub span: ByteSpan,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DottedIdentifier {
    pub pieces: Vec<Identifier>,
    pub span: ByteSpan,
}

impl From<Identifier> for DottedIdentifier {
    fn from(id: Identifier) -> DottedIdentifier {
        let span = id.span;
        DottedIdentifier {
            pieces: vec![id],
            span: span,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Declaration {
    pub ident: Identifier,
    pub ty: Identifier,
    pub span: ByteSpan,
}

impl Declaration {
    pub fn new(
        ident: Identifier,
        ty: Identifier,
        span: ByteSpan,
    ) -> Declaration {
        Declaration { ident, ty, span }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Assignment {
    pub variable: DottedIdentifier,
    pub value: Expression,
    pub span: ByteSpan,
}

sum_type::sum_type! {
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub enum Expression {
        Literal(Literal),
        Variable(DottedIdentifier),
        Binary(BinaryExpression),
        Unary(UnaryExpression),
        FunctionCall(FunctionCall),
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BinaryExpression {
    pub left: Box<Expression>,
    pub right: Box<Expression>,
    pub op: BinOp,
    pub span: ByteSpan,
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum BinOp {
    Add,
    Subtract,
    Or,
    Xor,
    And,
    Equals,
    NotEquals,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
    Multiply,
    Divide,
    Modulo,
    Not,
    Exponent,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnaryExpression {
    pub value: Box<Expression>,
    pub op: UnaryOp,
    pub span: ByteSpan,
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum UnaryOp {
    Not,
    Negate,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Literal {
    pub kind: LiteralKind,
    pub span: ByteSpan,
}

impl Literal {
    pub fn new<K: Into<LiteralKind>>(kind: K, span: ByteSpan) -> Literal {
        Literal {
            kind: kind.into(),
            span,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LiteralKind {
    Boolean(bool),
    Integer(i64),
    Float(f64),
    String(String),
}

impl From<bool> for LiteralKind {
    fn from(other: bool) -> LiteralKind {
        LiteralKind::Boolean(other)
    }
}

impl From<i64> for LiteralKind {
    fn from(other: i64) -> LiteralKind {
        LiteralKind::Integer(other)
    }
}

impl From<f64> for LiteralKind {
    fn from(other: f64) -> LiteralKind {
        LiteralKind::Float(other)
    }
}

impl From<String> for LiteralKind {
    fn from(other: String) -> LiteralKind {
        LiteralKind::String(other)
    }
}

impl<'a> From<&'a str> for LiteralKind {
    fn from(other: &'a str) -> LiteralKind {
        LiteralKind::String(other.to_string())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FunctionCall {
    pub name: Identifier,
    pub args: Vec<FunctionArg>,
    pub span: ByteSpan,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FunctionArg {
    Bare(Expression),
    Named(Assignment),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ForLoop {
    pub variable: Identifier,
    pub start: Expression,
    pub end: Expression,
    pub step: Option<Expression>,
    pub body: Vec<Statement>,
    pub span: ByteSpan,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WhileLoop {
    pub condition: Expression,
    pub body: Vec<Statement>,
    pub span: ByteSpan,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RepeatLoop {
    pub condition: Expression,
    pub body: Vec<Statement>,
    pub span: ByteSpan,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VarBlock {
    pub kind: VarBlockKind,
    pub declarations: Vec<Declaration>,
    pub span: ByteSpan,
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum VarBlockKind {
    Local,
    Input,
    Output,
    InputOutput,
}

macro_rules! impl_ast_node {
    ($name:tt => $($variant:tt)|*) => {
        impl AstNode for $name {
            fn span(&self) -> ByteSpan {
                match self {
                    $(
                        $name::$variant(ref inner) => inner.span(),
                    )*
                }
            }
        }
    };
    ($($name:ty,)*) => {
        $(
            impl AstNode for $name {
                fn span(&self) -> ByteSpan {
                    self.span
                }
            }
        )*
    };
}

impl_ast_node!(
    Literal,
    Assignment,
    Declaration,
    Identifier,
    BinaryExpression,
    UnaryExpression,
    FunctionCall,
    Return,
    ForLoop,
    WhileLoop,
    RepeatLoop,
    Exit,
    VarBlock,
    Program,
    File,
    FunctionBlock,
    Function,
    DottedIdentifier,
    IfStatement,
);
impl_ast_node!(Item => Function | FunctionBlock | Program);
impl_ast_node!(Expression => Literal | Binary | Unary | Variable | FunctionCall);
impl_ast_node!(Statement => FunctionCall | Assignment | Return | ForLoop |
    WhileLoop | RepeatLoop | Exit | IfStatement);
impl_ast_node!(FunctionArg => Bare | Named);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::*;
    use pretty_assertions::assert_eq;

    macro_rules! parse_test {
        ($name:ident, $parser:tt, $input:expr => $expected:expr) => {
            #[test]
            fn $name() {
                #[allow(unused_imports)]
                use $crate::grammar::*;
                let got = $parser::new().parse($input).unwrap();
                assert_eq!(got, $expected.into());
            }
        };
    }

    parse_test!(simple_ident, IdentParser, "hello" => Identifier { value: "hello".to_string(), span: s(0, 5) });
    parse_test!(ident_with_numbers, IdentParser, "hello_45" => Identifier { value: "hello_45".to_string(), span: s(0, 8) });

    parse_test!(example_decl, DeclParser, "x: Bool" => Declaration {
        ident: Identifier { value: "x".to_string(), span: s(0, 1) },
        ty: Identifier { value: "Bool".to_string(), span: s(3, 7) },
        span: s(0, 7),
    });

    parse_test!(assign_literal, AssignmentParser, "meaning_of_life := 42" => Assignment {
        variable: Identifier{ value: "meaning_of_life".to_string(), span: s(0, 15) }.into(),
        value: Expression::Literal(Literal {
            kind: LiteralKind::Integer(42),
            span: s(19, 21),
        }),
        span: s(0, 21),
    });

    parse_test!(function_call, ExprParser, "foo()" => Expression::FunctionCall(FunctionCall {
        name: Identifier { value: "foo".to_string(), span: s(0, 3) },
        args: Vec::new(),
        span: s(0, 5),
    }));

    parse_test!(function_call_with_args, ExprParser, "foo(1, second := 2)" => Expression::FunctionCall(FunctionCall {
        name: Identifier { value: "foo".to_string(), span: s(0, 3) },
        args: vec![
            FunctionArg::Bare(Expression::Literal(Literal::new(1, s(4, 5)))),
            FunctionArg::Named(Assignment {
                variable: Identifier { value: "second".to_string(), span: s(7, 13) }.into(),
                value: Expression::Literal(Literal::new(2, s(17, 18))),
                span: s(7, 18),
            }),
        ],
        span: s(0, 19),
    }));

    parse_test!(binary_op, ExprParser, "5+5" => Expression::Binary(BinaryExpression {
        left: Box::new(Expression::Literal(Literal {
            kind: LiteralKind::Integer(5),
            span: s(0, 1),
        })),
        right: Box::new(Expression::Literal(Literal {
            kind: LiteralKind::Integer(5),
            span: s(2, 3),
        })),
        op: BinOp::Add,
        span: s(0, 3),
    }));

    parse_test!(super_complex_expression, ExprParser, "5*5 + add(-(9**2), -34/pi)" =>
        Expression::Binary(BinaryExpression {
            left: Box::new(Expression::Binary(BinaryExpression {
                left: Box::new(Expression::Literal(Literal {
                    kind: LiteralKind::Integer(5),
                    span: s(0, 1),
                })),
                right: Box::new(Expression::Literal(Literal {
                    kind: LiteralKind::Integer(5),
                    span: s(2, 3),
                })),
                op: BinOp::Multiply,
                span: s(0, 3),
            })),
            right: Box::new(Expression::FunctionCall(FunctionCall {
                name: Identifier {
                    value: String::from("add"),
                    span: s(6, 9),
                },
                args: vec![
                    FunctionArg::Bare(Expression::Unary(UnaryExpression {
                        value: Box::new(Expression::Binary(BinaryExpression {
                            left: Box::new(Expression::Literal(Literal {
                                kind: LiteralKind::Integer(9),
                                span: s(12, 13),
                            })),
                            right: Box::new(Expression::Literal(Literal {
                                kind: LiteralKind::Integer(2),
                                span: s(15, 16),
                            })),
                            op: BinOp::Exponent,
                            span: s(12, 16),
                        })),
                        op: UnaryOp::Negate,
                        span: s(10, 17),
                    })),
                    FunctionArg::Bare(Expression::Binary(BinaryExpression {
                        left: Box::new(Expression::Literal(Literal {
                            kind: LiteralKind::Integer(-34),
                            span: s(19, 22),
                        })),
                        right: Box::new(Expression::Variable(Identifier {
                            value: String::from("pi"),
                            span: s(23, 25),
                        }.into())),
                        op: BinOp::Divide,
                        span: s(19, 25),
                    })),
                ],
                span: s(6, 26),
            })),
            op: BinOp::Add,
            span: s(0, 26),
        })
    );

    parse_test!(exit_statement, StmtParser, "exit" => Statement::Exit(Exit { span: s(0, 4)}));
    parse_test!(return_statement, StmtParser, "reTUrn" => Statement::Return(Return { span: s(0, 6)}));

    parse_test!(simple_for_loop, IterationStatementParser, "for x:= 0 TO 5 do return; end_for" => 
    Statement::ForLoop(ForLoop {
        variable: Identifier {
            value: String::from("x"),
            span: s(4, 5),
        },
        start: Expression::Literal(
            Literal {
                kind: LiteralKind::Integer(0),
                span: s(8, 9),
            }
        ),
        end: Expression::Literal(
            Literal {
                kind: LiteralKind::Integer(5),
                span: s(13, 14),
            }
        ),
        step: None,
        body: vec![
            Statement::Return(
                Return {
                    span: s(18, 24),
                }
            )
        ],
        span: s(0, 33),
    }));

    parse_test!(while_loop, IterationStatementParser, "while true do end_while" => 
    Statement::WhileLoop(WhileLoop {
        condition: Expression::Literal(Literal {
            kind: LiteralKind::Boolean(true),
            span: s(6, 10),
        }),
        body: Vec::new(),
        span: s(0, 23),
    }));

    parse_test!(repeat_loop, IterationStatementParser, "repeat return; until true end_repeat" => 
    Statement::RepeatLoop(RepeatLoop {
        condition: Expression::Literal(Literal {
            kind: LiteralKind::Boolean(true),
            span: s(21, 25),
        }),
        body: vec![
            Statement::Return(Return { span: s(7, 13) }),
        ],
        span: s(0, 36),
    }));

    parse_test!(single_var_block, BlockParser, "var i: INT; end_var" => VarBlock {
        kind: VarBlockKind::Local,
        declarations: vec![Declaration {
            ident: Identifier {
                value: String::from("i"),
                span: s(4, 5),
            },
            ty: Identifier {
                value: String::from("INT"),
                span: s(7, 10),
            },
            span: s(4, 10),
        }],
        span: s(0, 19),
    });

    const EXAMPLE_PROGRAM: &str = "
PROGRAM main
    VAR
        i : INT;
    END_VAR

    i := 0;
    REPEAT
        i := i + 1;
    UNTIL i >= 10
    END_REPEAT;
END_PROGRAM";

    fn example_program_parsed() -> Program {
        Program {
            name: Identifier {
                value: String::from("main"),
                span: s(9, 13),
            },
            var_blocks: vec![VarBlock {
                kind: VarBlockKind::Local,
                declarations: vec![Declaration {
                    ident: Identifier {
                        value: String::from("i"),
                        span: s(30, 31),
                    },
                    ty: Identifier {
                        value: String::from("INT"),
                        span: s(34, 37),
                    },
                    span: s(30, 37),
                }],
                span: s(18, 50),
            }],
            body: vec![
                Statement::Assignment(Assignment {
                    variable: Identifier {
                        value: String::from("i"),
                        span: s(56, 57),
                    }
                    .into(),
                    value: Expression::Literal(Literal {
                        kind: LiteralKind::Integer(0),
                        span: s(61, 62),
                    }),
                    span: s(56, 62),
                }),
                Statement::RepeatLoop(RepeatLoop {
                    condition: Expression::Binary(BinaryExpression {
                        left: Box::new(Expression::Variable(
                            Identifier {
                                value: String::from("i"),
                                span: s(105, 106),
                            }
                            .into(),
                        )),
                        right: Box::new(Expression::Literal(Literal {
                            kind: LiteralKind::Integer(10),
                            span: s(110, 112),
                        })),
                        op: BinOp::GreaterThanOrEqual,
                        span: s(105, 112),
                    }),
                    body: vec![Statement::Assignment(Assignment {
                        variable: Identifier {
                            value: String::from("i"),
                            span: s(83, 84),
                        }
                        .into(),
                        value: Expression::Binary(BinaryExpression {
                            left: Box::new(Expression::Variable(
                                Identifier {
                                    value: String::from("i"),
                                    span: s(88, 89),
                                }
                                .into(),
                            )),
                            right: Box::new(Expression::Literal(Literal {
                                kind: LiteralKind::Integer(1),
                                span: s(92, 93),
                            })),
                            op: BinOp::Add,
                            span: s(88, 93),
                        }),
                        span: s(83, 93),
                    })],
                    span: s(68, 127),
                }),
            ],
            span: s(1, 140),
        }
    }

    parse_test!(trivial_program, ProgramParser, EXAMPLE_PROGRAM => example_program_parsed());

    parse_test!(dotted_identifier, ExprParser, "x.y.z" => Expression::Variable(DottedIdentifier {
        pieces: vec![
            Identifier {
                value: "x".to_string(),
                span: s(0, 1),
            },
            Identifier {
                value: "y".to_string(),
                span: s(2, 3),
            },
            Identifier {
                value: "z".to_string(),
                span: s(4, 5),
            },
        ],
        span: s(0, 5),
    }));

    parse_test!(if_statement, IfParser, "if true then return; end_if" => IfStatement {
        condition: Expression::Literal(Literal{ kind: LiteralKind::Boolean(true), span: s(3, 7) }),
        body: vec![
            Statement::Return(Return { span: s(13, 19) }),
        ],
        span: s(0, 27),
    });
}
