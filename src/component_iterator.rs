use crate::{BitSetVec, BitSet};
// TODO try to reuse code between the two iterators

/// Iterates over components using a provided bitset.
/// Each time the bitset has a 1 in index i, the iterator will fetch data
/// from the storage at index i and return it as an `Option`.
pub struct ComponentIterator<'a, T> {
    pub(crate) current_id: usize,
    pub(crate) max_id: usize,
    pub(crate) storage: &'a Vec<Option<T>>,
    pub(crate) bitset: std::rc::Rc<BitSetVec>,
}

impl<'a, T> Iterator for ComponentIterator<'a, T> {
    type Item = Option<&'a T>;
    fn next(&mut self) -> Option<Self::Item> {
        while !self.bitset.bit_test(self.current_id) && self.current_id <= self.max_id {
            self.current_id += 1;
        }
        let ret = if self.current_id < self.max_id {
            Some(self.storage[self.current_id].as_ref())
        } else {
            None
        };
        self.current_id += 1;
        ret
    }
}

/// Iterates over components using a provided bitset.
/// Each time the bitset has a 1 in index i, the iterator will fetch data
/// from the storage at index i and return it as an `Option`.
pub struct ComponentIteratorMut<'a, T> {
    pub(crate) current_id: usize,
    pub(crate) max_id: usize,
    pub(crate) storage: &'a mut Vec<Option<T>>,
    pub(crate) bitset: std::rc::Rc<BitSetVec>,
}

impl<'a, T> Iterator for ComponentIteratorMut<'a, T> {
    type Item = Option<&'a mut T>;
    fn next(&mut self) -> Option<Self::Item> {
        while !self.bitset.bit_test(self.current_id) && self.current_id <= self.max_id {
            self.current_id += 1;
        }
        let ret = if self.current_id < self.max_id {
            // Unsafe: Used to tell the compiler that we won't mutably borrow the
            // same element from storage twice.
            let r = self.storage[self.current_id].as_mut().map(|e| unsafe {
                let ptr: *mut T = e;
                &mut *ptr
            });
            Some(r)
        } else {
            None
        };
        self.current_id += 1;
        ret
    }
}
