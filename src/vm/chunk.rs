use super::*;
use garbage_collector::trace::{Trace, Tracer};

#[derive(Debug, Clone)]
pub struct Chunk {
    code: Vec<u8>,
    name: String,
    constants: Vec<Value>,
    line: Vec<Line>,
}

#[derive(Debug, Copy, Clone)]
struct Line {
    pub start: usize,
    pub line: usize,
}