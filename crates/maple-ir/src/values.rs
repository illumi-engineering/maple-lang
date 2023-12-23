use crate::types::{PrimitiveType, Type, TypeInfo};

pub trait Value<T : Type> : TypeInfo<T> {}

pub enum PrimitiveValue {
    StringValue(String),
    IntegerValue(usize),
    FloatValue(f64),
    BooleanValue(bool),
    Nil, // monotype null
}

impl Value<PrimitiveType> for PrimitiveValue {}

impl TypeInfo<PrimitiveType> for PrimitiveValue {
    fn get_type(self) -> PrimitiveType {
        match self {
            PrimitiveValue::StringValue(_) => PrimitiveType::String,
            PrimitiveValue::IntegerValue(_) => PrimitiveType::Integer,
            PrimitiveValue::FloatValue(_) => PrimitiveType::Float,
            PrimitiveValue::BooleanValue(_) => PrimitiveType::Boolean,
            PrimitiveValue::Nil => PrimitiveType::Nil,
        }
    }
}