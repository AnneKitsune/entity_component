/// An entity index.
/// They are created using the `Entities` struct.
/// They are used as indices with `Components` structs.
///
/// Entities are conceptual "things" which possess attributes (Components).
/// As an exemple, a Car (Entity) has a Color (Component), a Position
/// (Component) and a Speed (Component).
#[cfg_attr(feature = "ser", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Entity(u32, u32);
impl Entity {
    /// Creates a new `Entity` from the provided index and generation.
    pub(crate) fn new(index: u32, generation: u32) -> Entity {
        Entity(index, generation)
    }

    /// Returns the index of this `Entity`.
    ///
    /// In most cases, you do not want to use this directly.
    /// However, it can be useful to create caches to improve performances.
    pub fn index(&self) -> u32 {
        self.0
    }

    /// Returns the generation of this `Entity`.
    ///
    ///
    /// In most cases, you do not want to use this directly.
    /// However, it can be useful to create caches to improve performances.
    pub fn generation(&self) -> u32 {
        self.1
    }
}
