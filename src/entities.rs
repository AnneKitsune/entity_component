use crate::{
    create_bitset, BitSet, BitSetVec, Entity, EntityIterator, BITSET_SIZE, BITSET_SLICE_COUNT,
};

/// Holds a list of alive entities.
/// It also holds a list of entities that were recently killed, which allows
/// to remove components of deleted entities at the end of a game frame.
pub struct Entities {
    alive: BitSetVec,
    generation: Vec<u32>,
    killed: Vec<Entity>,
    next_id: usize,
    /// helps to know if we should directly append after
    /// next_id or if we should look through the bitset.
    has_deleted: bool,
}

impl Default for Entities {
    fn default() -> Self {
        Self {
            alive: create_bitset(),
            generation: vec![0u32; BITSET_SIZE],
            killed: vec![],
            next_id: 0,
            has_deleted: false,
        }
    }
}

impl Entities {
    /// Creates a new `Entity` and returns it.
    /// This function will not reuse the index of an entity that is still in
    /// the killed entities.
    pub fn create(&mut self) -> Entity {
        if !self.has_deleted {
            let i = self.next_id;
            self.next_id += 1;
            self.alive.bit_set(i);
            Entity::new(i as u32, self.generation[i])
        } else {
            let mut section = 0;
            // Find section where at least one bit isn't set
            while self.alive[section].bit_all() {
                section += 1;
                if section >= BITSET_SLICE_COUNT {
                    panic!("Exceeded maximum amount of concurrent entities.");
                }
            }
            let mut i = section * (32 * 8);
            while self.alive.bit_test(i) || self.killed.iter().any(|e| e.index() == i as u32) {
                i += 1;
            }
            self.alive.bit_set(i);
            if i >= self.next_id {
                self.next_id = i + 1;
                self.has_deleted = false;
            }
            Entity::new(i as u32, self.generation[i])
        }
    }
    /// Checks if the `Entity` is still alive.
    /// Returns true if it is alive.
    /// Returns false if it has been killed.
    pub fn is_alive(&self, entity: Entity) -> bool {
        self.alive.bit_test(entity.index() as usize)
            && self.generation[entity.index() as usize] == entity.generation()
    }
    /// Kill an entity.
    pub fn kill(&mut self, entity: Entity) {
        if self.alive.bit_test(entity.index() as usize) {
            self.alive.bit_reset(entity.index() as usize);
            self.generation[entity.index() as usize] += 1;
            self.killed.push(entity);
            self.has_deleted = true;
        }
    }
    /// Returns entities in the killed list.
    pub fn killed(&self) -> &Vec<Entity> {
        &self.killed
    }
    /// Clears the killed entity list.
    pub fn clear_killed(&mut self) {
        self.killed.clear();
    }
    /// Returns a bitset where each index where the bit is set to 1 indicates
    /// the index of an alive entity.
    /// Useful for joining over `Entity` and `Component<T>` at the same time.
    pub fn bitset(&self) -> &BitSetVec {
        &self.alive
    }
    /// Iterates over entities using the provided bitset.
    pub fn iter_with_bitset<'a>(&'a self, bitset: std::rc::Rc<BitSetVec>) -> EntityIterator<'a> {
        EntityIterator {
            current_id: 0,
            next_id: self.next_id,
            entities: &self.alive,
            generations: &self.generation,
            bitset,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn create_kill_entities() {
        let mut entities = Entities::default();
        let e1 = entities.create();
        let e2 = entities.create();
        let e3 = entities.create();
        assert_eq!(e1.index(), 0);
        assert_eq!(e2.index(), 1);
        assert_eq!(e3.index(), 2);
        assert_eq!(e1.generation(), 0);
        assert!(entities.is_alive(e1));
        assert!(entities.is_alive(e2));
        assert!(entities.is_alive(e3));
        entities.kill(e1);
        assert!(!entities.is_alive(e1));
        assert!(entities.is_alive(e2));
        assert!(entities.is_alive(e3));
        let e4 = entities.create();
        assert!(!entities.is_alive(e1));
        assert!(entities.is_alive(e2));
        assert!(entities.is_alive(e3));
        assert!(entities.is_alive(e4));

        assert_eq!(*entities.killed(), vec![e1]);
        entities.clear_killed();
        assert_eq!(*entities.killed(), vec![]);
    }

    #[test]
    fn test_interleaved_create_kill() {
        let mut entities = Entities::default();

        let e1 = entities.create();
        assert_eq!(e1.index(), 0);
        let e2 = entities.create();
        assert_eq!(e2.index(), 1);
        entities.kill(e1);
        entities.kill(e2);
        assert_eq!(entities.is_alive(e1), false);
        assert_eq!(entities.is_alive(e2), false);

        let e3 = entities.create();
        assert_eq!(e3.index(), 2);
        let e4 = entities.create();
        assert_eq!(e4.index(), 3);
        entities.kill(e3);
        entities.kill(e4);
        assert_eq!(entities.is_alive(e3), false);
        assert_eq!(entities.is_alive(e4), false);
    }
}
