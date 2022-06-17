use super::super::garbage_collector::{*, tag::*, trace::*};
use super::*;

use std::{
    fmt::{Debug, Display},
    mem,
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Value {
    handle: TaggedHandle<Object>,
}

pub struct WithHeap<'h, T> {
    pub heap: &'h Heap<Object>,
    pub item: T,
}

impl<'h, T> WithHeap<'h, T> {
    pub fn new(heap: &'h Heap<Object>, item: T) -> WithHeap<'h, T> {
        WithHeap { heap, item }
    }

    pub fn with<U>(&self, item: U) -> WithHeap<U> {
        WithHeap { heap: self.heap, item }
    }
}

impl<'h, 'a> Display for WithHeap<'h, &'a Object> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use self::Object::*;

        match self.item {
            String(ref s) => write!(f, "{}", s),
            NativeFunction(ref na) => write!(f, "<native fn {}>", na.name),
            Function(ref fun) => write!(f, "<fn {}>", fun.name),
            Closure(ref cl) => write!(f, "<fn {}>", cl.function.name),
            List(ref ls) => write!(f, "<list [{}]>", ls.content.len()),
            Dict(ref ls) => write!(f, "<dict [{}]>", ls.content.len()),
        }
    }
}