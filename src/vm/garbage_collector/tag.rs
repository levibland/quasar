use super::Handle;

use std::mem;

#[derive(Debug)]
pub struct TaggedHandle<T> {
    handle: Handle<T>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tag<T> {
    Tag(u8),
    Float(f64),
    Handle(Handle<T>),
}

// Dynamic typing shenanigans
// Fake NAN, used in NAN boxing
const NANISH: u64 = 0x7ffc000000000000;
const SIGN: u64 = 1 << 63;

impl<T> TaggedHandle<T> {
    pub unsafe fn from_raw(raw: u64) -> Self {
        Self {
            handle: Handle::new(0, raw as *mut T),
        }
    }

    pub fn to_raw(&self) -> u64 {
        self.handle.ptr as u64
    }

    pub fn from_handle(handle: Handle<T>) -> Self {
        let u = (handle.ptr as u64) | NANISH | SIGN;
        Self {
            handle: Handle::new(handle.gen, u as *mut T),
        }
    }

    pub fn from_float(float: f64) -> Self {
        Self {
            handle: Handle::new(0, unsafe { mem::transmute(float) }),
        }
    }

    pub fn from_tag(tag: u8) -> Self {
        Self {
            handle: Handle::new(0, unsafe { mem::transmute(NANISH | (tag as u64)) }),
        }
    }

    pub fn decode(self) -> Tag<T> {
        let u = self.handle.ptr as u64;
        if u & NANISH != NANISH {
            return Tag::Float(unsafe { mem::transmute(u) });
        }
        if (u & (NANISH | SIGN)) == (NANISH | SIGN) {
            let ptr = u & (!(NANISH | SIGN));
            return Tag::Handle(Handle::new(self.handle.gen, ptr as *mut T));
        }
        let tag: u8 = (u & 7) as u8;
        Tag::Tag(tag)
    }
}

impl<T> Clone for TaggedHandle<T> {
    fn clone(&self) -> Self {
        Self { handle: self.handle }
    }
}
impl<T> Copy for TaggedHandle<T> {}

impl<T> PartialEq<Self> for TaggedHandle<T> {
    fn eq(&self, other: &Self) -> bool {
        self.handle == other.handle
    }
}
impl<T> Eq for TaggedHandle<T> {}

impl<T> From<Handle<T>> for TaggedHandle<T> {
    fn from(handle: Handle<T>) -> Self {
        Self::from_handle(handle)
    }
}

impl<T> From<f64> for TaggedHandle<T> {
    fn from(float: f64) -> Self {
        Self::from_float(float)
    }
}