use crate::{create_bitset, Entity, BitSetVec, BitSet, ComponentIterator, ComponentIteratorMut, BITSET_SIZE};

use std::collections::HashMap;
use std::any::{TypeId, Any};
use std::sync::Mutex;
use atomic_refcell_try::AtomicRefMut;

lazy_static::lazy_static! {
    #[doc(hidden)]
    pub static ref COMPONENT_REGISTRY: Mutex<HashMap<TypeId, Box<dyn Fn(AtomicRefMut<dyn Any+'static>, &[Entity]) + Send + Sync>>> = Mutex::new(HashMap::default());
}

/// Holds components of a given type indexed by `Entity`.
/// We do not check if the given entity is alive here, this should be done using
/// `Entities`.
pub struct Components<T> {
    bitset: BitSetVec,
    components: Vec<Option<T>>,
}

impl<T: 'static> Default for Components<T> {
    fn default() -> Self {
        // Registers all the component downcasting and cleaning code in one globally accessible
        // place. This seems to be the best way of doing it that doesn't involve
        // heavily modifying how the `world_dispatcher` crate works.
        COMPONENT_REGISTRY.lock().unwrap().insert(TypeId::of::<Self>(), Box::new(|any, entities| {
            let mut me = AtomicRefMut::map(any, |j| j.downcast_mut::<Self>().unwrap());
            for e in entities {
                me.remove(*e);
            }
        }));
        Self {
            bitset: create_bitset(),
            // Approximation of a good default.
            components: Vec::with_capacity(BITSET_SIZE >> 4),
        }
    }
}

impl<T> Components<T> {
    /// Inserts a component for the given `Entity` index.
    /// Returns the previous component, if any.
    pub fn insert(&mut self, entity: Entity, component: T) -> Option<T> {
        let mut insertion = Some(component);
        if self.bitset.bit_test(entity.index() as usize) {
            std::mem::swap(
                &mut insertion,
                &mut self.components[entity.index() as usize],
            );
            insertion
        } else {
            self.allocate_enough(entity.index() as usize);
            self.bitset.bit_set(entity.index() as usize);
            self.components[entity.index() as usize] = insertion;
            None
        }
    }
    /// Ensures that we have the vec filled at least until the `until`
    /// variable. Usually, set this to `entity.index`.
    fn allocate_enough(&mut self, until: usize) {
        if self.components.len() <= until {
            let qty = (until - self.components.len()) + 1;
            for _ in 0..qty {
                self.components.push(None);
            }
        }
    }
    /// Gets an immutable reference to the component of `Entity`.
    pub fn get(&self, entity: Entity) -> Option<&T> {
        if self.bitset.bit_test(entity.index() as usize) {
            self.components[entity.index() as usize].as_ref()
        } else {
            None
        }
    }
    /// Gets a mutable reference to the component of `Entity`.
    pub fn get_mut(&mut self, entity: Entity) -> Option<&mut T> {
        if self.bitset.bit_test(entity.index() as usize) {
            self.components[entity.index() as usize].as_mut()
        } else {
            None
        }
    }
    /// Removes the component of `Entity`.
    /// Returns `Some(T)` if the entity did have the component.
    /// Returns `None` if the entity did not have the component.
    pub fn remove(&mut self, entity: Entity) -> Option<T> {
        let idx = entity.index() as usize;
        if self.bitset.bit_test(idx) {
            self.bitset.bit_reset(idx);
            let mut ret = None;
            std::mem::swap(&mut ret, &mut self.components[idx]);
            ret
        } else {
            None
        }
    }
    /// Iterates immutably over all components of this type.
    /// Very fast but doesn't allow joining with other component types.
    pub fn iter<'a>(&'a self) -> impl Iterator<Item = &'a T> {
        self.components.iter().flatten()
    }
    /// Iterates mutably over all components of this type.
    /// Very fast but doesn't allow joining with other component types.
    pub fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut T> {
        self.components.iter_mut().flatten()
    }
    /// Iterates immutably over the components of this type where `bitset`
    /// indicates the indices of entities.
    /// Slower than `iter()` but allows joining between multiple component types.
    pub fn iter_with_bitset<'a>(&'a self, bitset: std::rc::Rc<BitSetVec>) -> ComponentIterator<'a, T> {
        ComponentIterator {
            current_id: 0,
            max_id: self.components.len(),
            storage: &self.components,
            bitset,
        }
    }
    /// Iterates mutable over the components of this type where `bitset`
    /// indicates the indices of entities.
    /// Slower than `iter()` but allows joining between multiple component types.
    pub fn iter_mut_with_bitset<'a>(
        &'a mut self,
        bitset: std::rc::Rc<BitSetVec>,
    ) -> ComponentIteratorMut<'a, T> {
        ComponentIteratorMut {
            current_id: 0,
            max_id: self.components.len(),
            storage: &mut self.components,
            bitset,
        }
    }
    /// Returns the bitset indicating which entity indices have a component
    /// associated to them.
    /// Useful to build conditions between multiple `Components`' bitsets.
    ///
    /// For example, take two bitsets from two different `Components` types.
    /// Then, bitset1.clone().bit_and(bitset2);
    /// And finally, you can use bitset1 in `iter_with_bitset` and `iter_mut_with_bitset`.
    /// This will iterate over the components of the entity only for entities that have both
    /// components.
    pub fn bitset(&self) -> &BitSetVec {
        &self.bitset
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn create_remove_components() {
        #[derive(Debug, Clone, PartialEq, Eq)]
        struct A;

        let mut entities = Entities::default();
        let e1 = entities.create();
        let e2 = entities.create();
        
        let mut storage = Components::<A>::default();
        storage.insert(e1, A);
        storage.insert(e2, A);
        assert!(storage.get(e1).is_some());
        storage.remove(e1);
        assert!(storage.get(e1).is_none());
        assert_eq!(storage.iter().cloned().collect::<Vec<_>>(), vec![A])
    }
}


