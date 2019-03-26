use crate::{BinOp, BinaryExpression, Expression, UnaryExpression, UnaryOp};
use codespan::{ByteIndex, ByteSpan};

pub(crate) fn s(start: usize, end: usize) -> ByteSpan {
    ByteSpan::new(ByteIndex(start as u32), ByteIndex(end as u32))
}

pub(crate) fn bop<L, R>(l: L, r: R, op: BinOp, span: ByteSpan) -> Expression
where
    L: Into<Expression>,
    R: Into<Expression>,
{
    let expr = BinaryExpression {
        left: Box::new(l.into()),
        right: Box::new(r.into()),
        op,
        span,
    };

    Expression::Binary(expr)
}

pub(crate) fn unop<E>(expr: E, op: UnaryOp, span: ByteSpan) -> Expression
where
    E: Into<Expression>,
{
    Expression::Unary(UnaryExpression {
        value: Box::new(expr.into()),
        op,
        span,
    })
}
