use std::{
    cmp::{PartialEq, Eq},
    rc::Rc,
    hash::{Hash, Hasher},
    collections::{HashMap, HashSet},
};

type Generation = usize;

#[derive(Clone)]
pub struct Heap<T> {
    last_sweep: usize,
    object_sweeps: HashMap<Handle<T>, usize>,
    object_counter: Generation,
    objects: HashSet<Handle<T>>,
    rooted: HashMap<Handle<T>, Rc<()>>,
}

impl<T> Default for Heap<T> {
    fn default() -> Self {
        Self {
            last_sweep: 0,
            object_sweeps: HashMap::default(),
            object_counter: 0,
            objects: HashSet::default(),
            rooted: HashMap::default(),
        }
    }
}

impl<T> Drop for Heap<T> {
    fn drop(&mut self) {
        for handle in &self.objects {
            drop(unsafe { Box::from_raw(handle.ptr) });
        }
    }
}

#[derive(Debug)]
pub struct Handle<T> {
    gen: Generation,
    ptr: *mut T,
}

impl<T> Handle<T> {
    pub unsafe fn get_unchecked(&self) -> &T {
        &*self.ptr
    }

    pub unsafe fn get_mut_unchecked(&self) -> &mut T {
        &mut *self.ptr
    }
}

impl<T> Copy for Handle<T> {}

impl<T> Clone for Handle<T> {
    fn clone(&self) -> Self {
        Self { gen: self.gen, ptr: self.ptr }
    }
}

impl<T> PartialEq<Self> for Handle<T> {
    fn eq(&self, other: &Self) -> bool {
        self.gen == other.gen && self.ptr == other.ptr
    }
}

impl<T> Eq for Handle<T> {}

impl<T> Hash for Handle<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.gen.hash(state);
        self.ptr.hash(state);
    }
}

impl<T> AsRef<Handle<T>> for Handle<T> {
    fn as_ref(&self) -> &Handle<T> {
        self
    }
}