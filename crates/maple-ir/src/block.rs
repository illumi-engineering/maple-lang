use crate::expr::Expr;

pub struct Block {
    pub code: Vec<Box<dyn Expr>>,
}

impl Expr for Block {}