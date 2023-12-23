use std::collections::HashMap;
use std::marker::PhantomData;
use std::ops::Index;
use crate::expr::Expr;
use crate::prototype::{Prototype, PrototypeDefinition};
use crate::types::{Type, TypeInfo};
use crate::values::Value;

type TupleInnerValue = Box<dyn Value<Box<dyn Type>>>;

pub struct TupleValue {
    pub size: i8,
    pub values: Vec<TupleInnerValue>,
    pub _type: TupleDefinition,
}

impl TypeInfo<TupleDefinition> for TupleValue {
    fn get_type(self) -> TupleDefinition {
        self._type
    }
}

impl Value<TupleDefinition> for TupleValue {}

impl Index<usize> for TupleValue {
    type Output = TupleInnerValue;

    fn index(&self, index: usize) -> &Self::Output {
        &self.values[index]
    }
}

pub struct TupleDefinition {
    pub type_name: String,
    pub size: i8,
    pub shape: Vec<Box<dyn Type>>,
}

impl Expr for TupleDefinition {}

impl Type for TupleDefinition {
    fn name(self) -> String {
        self.type_name
    }
}

///
///
/// # Type Arguments
/// `T` The Tuple type this prototype definition is associated with
pub struct TuplePrototypeDefinition<T : Type, V> where V : Value<T> {
    recipient: V,
    type_name: String,
    __marker: PhantomData<T>,
}

impl<T : Type, V : Value<T>> Expr for TuplePrototypeDefinition<T, V> {}

impl<T : Type, V : Value<T>> PrototypeDefinition<T, V> for TuplePrototypeDefinition<T, V> {
    fn build_proto<'a>(self, _type: T) -> Prototype<'a, T, V> {
        Prototype {
            value: self.recipient,
            associated_type: _type,
            methods: HashMap::new(),
        }
    }
}

impl<T: Type, V: Value<T>> TuplePrototypeDefinition<T, V> {
    pub fn new(recipient: V, _type: T) -> Self {
        Self {
            recipient,
            type_name: _type.name(),
            __marker: PhantomData::default(),
        }
    }
}