use crate::{Entity, BitSetVec, create_bitset, BitSet, BITSET_SLICE_COUNT, BITSET_SIZE, EntityIterator};

/// Holds a list of alive entities.
/// It also holds a list of entities that were recently killed, which allows
/// to remove components of deleted entities at the end of a game frame.
pub struct Entities {
    alive: BitSetVec,
    generation: Vec<u32>,
    killed: Vec<Entity>,
    max_id: usize,
    /// helps to know if we should directly append after
    /// max_id or if we should look through the bitset.
    has_deleted: bool,
}

impl Default for Entities {
    fn default() -> Self {
        Self {
            alive: create_bitset(),
            generation: vec![0u32; BITSET_SIZE],
            killed: vec![],
            max_id: 0,
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
            let i = self.max_id;
            self.max_id += 1;
            self.alive.bit_set(i);
            Entity::new(i as u32, self.generation[i])
        } else {
            let mut section = 0;
            // Find section where at least one bit isn't set
            while self.alive[section].bit_all() {
                section += 1;
                if section > BITSET_SLICE_COUNT {
                    panic!("Exceeded maximum amount of concurrent entities.");
                }
            }
            let mut i = section * (32 * 8);
            while self.alive.bit_test(i) || self.killed.iter().any(|e| e.index() == i as u32) {
                i += 1;
            }
            self.alive.bit_set(i);
            if i >= self.max_id {
                self.max_id = i;
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
    //pub fn iter_with_bitset<'a>(&'a self, bitset: &'a BitSetVec) -> EntityIterator<'a> {
    pub fn iter_with_bitset<'a>(&'a self, bitset: std::rc::Rc<BitSetVec>) -> EntityIterator<'a> {
        EntityIterator {
            current_id: 0,
            max_id: self.max_id,
            entities: &self.alive,
            generations: &self.generation,
            bitset,
        }
    }
}
