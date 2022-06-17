use std::collections::HashMap;
use std::fs::File;

use fnv::FnvBuildHasher;

use flame as f;
use flamer::flame;

use super::*;

use std::mem;

const STACK_SIZE:  usize = 4096;
const HEAP_GROWTH: usize = 2;

const GC_TRIGGER_COUNT: usize = 1024;

pub struct CallFrame {
    closure: Handle<Object>,
    ip: usize,
    stack_start: usize,
}

impl CallFrame {
    pub fn new(closure: Handle<Object>, stack_start: usize) -> Self {
        Self {
            closure,
            ip: 0,
            stack_start,
        }
    }
}

pub struct VM {
    pub heap: Heap<Object>,
    next_gc: usize,

    pub globals: HashMap<String, Value, FnvBuildHasher>,
    pub open_upvalues: Vec<UpValue>,

    pub stack: Vec<Value>,
    pub frames: Vec<CallFrame>,
}

impl VM {
    pub fn new() -> Self {
        VM {
            stack:   Vec::with_capacity(STACK_SIZE),
            heap:    Heap::default(),
            next_gc: GC_TRIGGER_COUNT,
            globals: HashMap::with_hasher(FnvBuildHasher::default()),
            frames:  Vec::with_capacity(256),
            open_upvalues: Vec::with_capacity(16),
        }
    }
}