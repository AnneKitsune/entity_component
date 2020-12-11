use crate::{BitSetVec, Entity, BitSet};

/// Iterator over entities using the provided bitset.
pub struct EntityIterator<'a> {
    pub(crate) current_id: usize,
    pub(crate) max_id: usize,
    pub(crate) entities: &'a BitSetVec,
    pub(crate) generations: &'a Vec<u32>,
    //pub(crate) bitset: &'a BitSetVec,
    pub(crate) bitset: std::rc::Rc<BitSetVec>,
}

impl<'a> Iterator for EntityIterator<'a> {
    type Item = Option<Entity>;
    fn next(&mut self) -> Option<Self::Item> {
        while !self.bitset.bit_test(self.current_id) && self.current_id <= self.max_id {
            self.current_id += 1;
        }
        let ret = if self.current_id <= self.max_id {
            if self.entities.bit_test(self.current_id) {
                Some(Some(Entity::new(
                    self.current_id as u32,
                    self.generations[self.current_id],
                )))
            } else {
                Some(None)
            }
        } else {
            None
        };
        self.current_id += 1;
        ret
    }
}
