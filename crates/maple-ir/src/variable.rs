use crate::expr::Expr;
use crate::types::Type;
use crate::values::Value;

pub enum VariableMetaType {
    Static,
}

pub struct VariableDeclaration<T : Type, V : Value<T>> {
    pub meta_type: VariableMetaType,
    pub _type: T,
    pub value: V,
}

impl<T : Type, V : Value<T>> Expr for VariableDeclaration<T, V> {}