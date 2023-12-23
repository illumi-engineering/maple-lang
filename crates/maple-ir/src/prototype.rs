use crate::types::Type;
use crate::values::Value;

pub struct Prototype <T : Type, V : Value<T>> {
    value: V,

}