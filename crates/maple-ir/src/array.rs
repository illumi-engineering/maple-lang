use std::ops::Index;
use crate::expr::Expr;
use crate::types::Type;
use crate::values::Value;

pub struct ArrayType<T : Type, V : Value<T>> {
    element_type: T,
    values: Vec<V>,
}

impl<T: Type, V: Value<T>> Expr for ArrayType<T, V> {}

impl<T : Type, V : Value<T>> Type for ArrayType<T, V> {
    fn name(self) -> String {
        format!("{}[]", self.element_type.name())
    }
}

impl<T : Type, V : Value<T>> Index<usize> for ArrayType<T, V> {
    type Output = V;

    fn index(&self, index: usize) -> &Self::Output {
        &self.values[index]
    }
}