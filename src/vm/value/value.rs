use super::super::garbage_collector::{*, tag::*, trace::*};
use super::*;

use std::{
    fmt::{Debug, Display},
    mem,
};

pub struct Value {
    handle: TaggedHandle<Object>,
}