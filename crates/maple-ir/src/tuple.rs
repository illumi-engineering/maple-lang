use std::collections::HashMap;
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

pub struct TuplePrototypeDefinition {
    associated_type: TupleDefinition,
}

impl Expr for TuplePrototypeDefinition {}

impl PrototypeDefinition<TupleDefinition, TupleValue> for TuplePrototypeDefinition {
    fn build_proto<'a>(self, receiver: TupleValue) -> Prototype<'a, TupleDefinition, TupleValue> {
        Prototype {
            receiver,
            associated_type: self.associated_type,
            methods: HashMap::new(),
        }
    }
}

impl TuplePrototypeDefinition {
    pub fn new(_type: TupleDefinition) -> Self {
        Self {
            associated_type: _type,
        }
    }
}