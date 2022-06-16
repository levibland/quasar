use super::super::garbage_collector::{*, tag::*, trace::*};
use super::*;

use std::{
    fmt::{Debug, Display},
    rc::Rc,
    cell::RefCell,
};

use im_rc::hashmap::HashMap;

pub enum Object {
    String(String),
    Function(Function),
    NativeFunction(NativeFunction),
    Closure(Closure),
    List(List),
    Dict(Dict),
}

#[derive(Debug, Clone)]
pub struct Function {
    name: String,
    chunk: Chunk,
    arity: u8,
    upvalue_count: usize,
}

#[derive(Clone)]
pub struct NativeFunction {
    pub nume: String,
    pub arity: u8,
    pub function: fn(&mut Heap<Object>, &[Value]) -> Value,
}

#[derive(Debug, Clone)]
pub struct UpValue {
    inner: Rc<RefCell<Result<Value, usize>>>,
}

pub struct Dict {
    pub content: HashMap<HashValue, Value>,
}

#[derive(Debug)]
pub struct List {
    pub content: Vec<Value>,
}

#[derive(Debug, Clone)]
pub struct Closure {
    function: Function,
    upvalues: Vec<UpValue>,
}