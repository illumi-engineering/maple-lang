use std::collections::HashMap;
use crate::closure::Closure;
use crate::spread::Spread;
use crate::types::Type;
use crate::values::Value;

pub struct PrototypeMethod<'a,> {
    closure: &'a mut Closure<'a>,
    reciever_values: Spread,
}

impl<'a> PrototypeMethod<'a> {
    pub fn new(closure: &'a mut Closure<'a>) -> Self {
        Self {
            closure,
            reciever_values: Spread {},
        }
    }

    pub fn get_name(&self) -> String {
        self.closure.name.to_owned()
    }
}

pub struct Prototype<'a, T : Type, V : Value<T>> {
    pub(crate) receiver: V,
    pub(crate) methods: HashMap<String, &'a mut PrototypeMethod<'a>>,
    pub(crate) associated_type: T,
}

impl<'a, T : Type, V : Value<T>> Prototype<'a, T, V> {
    pub fn new(receiver: V, associated_type: T) -> Self {
        Self {
            receiver,
            associated_type,
            methods: HashMap::new(),
        }
    }

    pub fn add_method(mut self, method: &'a mut PrototypeMethod<'a>) -> Result<(), String> {
        let method_name = method.get_name().to_owned();
        if self.methods.contains_key(&method_name) {
            return Err(format!("Method with name '{}' already exists for type {}", &method_name, self.associated_type.name()));
        }
        self.methods.insert(method_name, method);
        Ok(())
    }
}

pub trait PrototypeDefinition<T : Type, V : Value<T>> {
    fn build_proto<'a>(self, receiver: V) -> Prototype<'a, T, V>;
}