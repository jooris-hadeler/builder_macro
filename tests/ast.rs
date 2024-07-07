use builder_macro::Builder;

#[derive(PartialEq, Debug)]
pub enum Expr {
    Binary(BinaryExpr),
    Literal(i32),
}

impl From<BinaryExpr> for Expr {
    fn from(expr: BinaryExpr) -> Self {
        Expr::Binary(expr)
    }
}

impl From<i32> for Expr {
    fn from(literal: i32) -> Self {
        Expr::Literal(literal)
    }
}

#[derive(Builder, PartialEq, Debug)]
pub struct BinaryExpr {
    pub lhs: Box<Expr>,
    pub op: BinOp,
    pub rhs: Box<Expr>,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
}

#[test]
fn test_ast() {
    let expr = BinaryExpr::builder()
        .with_lhs(Box::new(12.into()))
        .with_op(BinOp::Add)
        .with_rhs(Box::new(
            BinaryExpr::builder()
                .with_lhs(Box::new(34.into()))
                .with_op(BinOp::Mul)
                .with_rhs(Box::new(56.into()))
                .build()
                .unwrap()
                .into(),
        ))
        .build()
        .unwrap();

    assert_eq!(
        expr,
        BinaryExpr {
            lhs: Box::new(Expr::Literal(12)),
            op: BinOp::Add,
            rhs: Box::new(Expr::Binary(BinaryExpr {
                lhs: Box::new(Expr::Literal(34)),
                op: BinOp::Mul,
                rhs: Box::new(Expr::Literal(56)),
            })),
        }
    );
}
