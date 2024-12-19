use crate::GetCell;
use crate::GetCellMut;

use std::collections::BTreeSet;

use std::collections::HashSet;

use crate::Result;

use crate::Position;

/// Writable side of maps that can add rocks to it.
pub trait MutMap {
    /// Add a rock to the map at the given position.
    /// Could fail if the rock is out of bounds.
    fn add_rock(&mut self, rock: Position) -> Result<()>;
}

/// Query methods for maps.
pub trait Map {
    /// Check if the given position is a valid position to move to.
    fn can_move_to(&self, pos: &Position) -> bool;
    /// Return the bounds of the map.
    fn bound(&self) -> Position;
    /// Starting position is always (0, 0).
    fn start(&self) -> Position {
        (0, 0)
    }
    /// Ending position is always the last cell in the map
    fn end(&self) -> Position {
        let bound = self.bound();
        // Clamp to 0 in case the bound is 0.
        (bound.0.saturating_sub(1), bound.1.saturating_sub(1))
    }
}

pub trait Contains<T> {
    fn contains(&self, value: &T) -> bool;
}

/// A container that behaves like a HashSet.
///
/// This is used to abstract over the different types of containers.
/// This is to provide a trait for things that can be put into a container.
pub trait HashContainer<T>: Contains<T> {
    /// Insert the given value into the container.
    fn insert(&mut self, value: T) -> bool;
}

impl<T> Contains<T> for HashSet<T>
where
    T: std::hash::Hash + Eq,
{
    fn contains(&self, value: &T) -> bool {
        HashSet::contains(self, value)
    }
}

/// Teach the compiler how HashSets look like HashContainers.
///
/// Simply delegates to the HashSet implementation.
/// Not rocket science.
impl<T> HashContainer<T> for HashSet<T>
where
    T: std::hash::Hash + Eq,
{
    fn insert(&mut self, value: T) -> bool {
        HashSet::insert(self, value)
    }
}

impl<T> Contains<T> for BTreeSet<T>
where
    T: Ord,
{
    fn contains(&self, value: &T) -> bool {
        BTreeSet::contains(self, value)
    }
}

/// Teach the compiler how BTreeSets look like HashContainers.
///
/// Simply delegates to the BTreeSet implementation.
impl<T> HashContainer<T> for BTreeSet<T>
where
    T: Ord,
{
    fn insert(&mut self, value: T) -> bool {
        BTreeSet::insert(self, value)
    }
}

impl<T> Contains<T> for Vec<T>
where
    T: PartialEq,
{
    fn contains(&self, value: &T) -> bool {
        self.iter().any(|v| v == value)
    }
}

/// For funzies, we can implement HashContainer for Vec.
impl<T> HashContainer<T> for Vec<T>
where
    T: PartialEq,
{
    fn insert(&mut self, value: T) -> bool {
        Vec::push(self, value);
        true
    }
}

/// A bounded map will implement Map and MutMap, and is
/// backed by something that implements HashContainer.
#[allow(dead_code)]
pub struct BoundedMap<T>
where
    T: HashContainer<Position>,
{
    pub map: T,
    pub bounds: Position,
}

/// Since the backing of bounded maps only implement HashContainer,
/// our `new` constructor will take a bound and create ourselves with
/// a default HashContainer.
impl<T> BoundedMap<T>
where
    T: HashContainer<Position> + Default,
{
    #[allow(dead_code)]
    /// Create a new BoundedMap with the given bounds.
    pub fn new(bounds: Position) -> Self {
        Self {
            map: Default::default(),
            bounds,
        }
    }
}

// Bounded maps simply insert the rocks into the backing.
impl<T> MutMap for BoundedMap<T>
where
    T: HashContainer<Position>,
{
    fn add_rock(&mut self, rock: Position) -> Result<()> {
        self.map.insert(rock);
        Ok(())
    }
}

// Bounded maps will query maps using the backing container
// to determine occupancy.
impl<T> Map for BoundedMap<T>
where
    T: HashContainer<Position>,
{
    fn bound(&self) -> Position {
        self.bounds
    }
    fn can_move_to(&self, pos: &Position) -> bool {
        // If the position is out of bounds, we can't move there.
        if pos.0 >= self.bounds.0 || pos.1 >= self.bounds.1 {
            false
        } else {
            // Otherwise, we can move there if the position is not occupied.
            !self.map.contains(pos)
        }
    }
}

// Blanket implemention for MutMap for things that implement GetCellMut.
impl<C> MutMap for C
where
    C: GetCellMut<bool>,
{
    fn add_rock(&mut self, rock: Position) -> Result<()> {
        let c = self.get_cell_mut_result(&rock)?;
        *c = true;
        Ok(())
    }
}

// Blanket implemention for Map for things that implement GetCell.
impl<C> Map for C
where
    C: GetCell<bool>,
{
    fn can_move_to(&self, pos: &Position) -> bool {
        let c = self.get_cell(pos);
        if let Some(occupied) = c {
            !*occupied
        } else {
            // out of bounds, can't move there.
            false
        }
    }

    fn bound(&self) -> Position {
        GetCell::bound(self)
    }
}

/// MutMap can be implemented for a Vec<Vec<bool>>.
///
/// This delegates to our GetCellMut implementations.
impl MutMap for Vec<Vec<bool>> {
    fn add_rock(&mut self, xy: Position) -> Result<()> {
        self.as_mut_slice().add_rock(xy)
    }
}

/// Map can be implemented for a Vec<Vec<bool>>.
///
/// This delegates to our GetCell implementations.
impl Map for Vec<Vec<bool>> {
    fn bound(&self) -> Position {
        Map::bound(&self.as_slice())
    }
    fn can_move_to(&self, pos: &Position) -> bool {
        self.as_slice().can_move_to(pos)
    }
}

/// Similarly, we implement MutMap for two dimensional arrays.
///
/// This delegates to our GetCellMut implementations.
impl<const XBOUND: usize, const YBOUND: usize> MutMap for [[bool; XBOUND]; YBOUND] {
    fn add_rock(&mut self, xy: Position) -> Result<()> {
        self.as_mut_slice().add_rock(xy)
    }
}

/// Similarly, we implement Map for two dimensional arrays.
///
/// This delegates to our GetCell implementations.
impl<const XBOUND: usize, const YBOUND: usize> Map for [[bool; XBOUND]; YBOUND] {
    fn bound(&self) -> Position {
        (XBOUND, YBOUND)
    }
    fn can_move_to(&self, pos: &Position) -> bool {
        self.as_slice().can_move_to(pos)
    }
}
