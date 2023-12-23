use std::iter::Map;
use crate::block::Block;
use crate::types::Type;
use crate::values::Value;

pub struct Closure<'a> {
    pub name: &'a str,
    pub closed_values: Map<String, Box<dyn Value<dyn Type>>>,
    pub body: Block,
}