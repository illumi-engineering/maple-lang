pub trait Type {}

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

impl Type for PrimitiveType {}

pub struct Tuple {
    type_name: String,
    size: i8,
    shape: Vec<Box<dyn Type>>,
}

impl Type for Tuple {}

