use crate::expr::Expr;

pub trait Type : Expr {
    fn name(self) -> String;
}

pub trait TypeInfo<T : Type> {
    fn get_type(self) -> T;
}

pub enum PrimitiveType {
    String,
    Integer,
    Float,
    Boolean,
    Nil,
}

impl Expr for PrimitiveType {}

impl Type for PrimitiveType {
    fn name(self) -> String {
        match self {
            PrimitiveType::String => "String".to_owned(),
            PrimitiveType::Integer => "Int".to_owned(),
            PrimitiveType::Float => "Float".to_owned(),
            PrimitiveType::Boolean => "Boolean".to_owned(),
            PrimitiveType::Nil => "Nil".to_owned()
        }
    }
}
