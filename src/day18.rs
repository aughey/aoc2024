use crate::{add_xy, Direction, GetCell, GetCellMut, Position, Result};
use aoc_runner_derive::aoc;
use std::{
    collections::{BTreeSet, HashSet},
    fmt::Display,
};

pub const DAY: u32 = 18;

/// The size of our map (depending if we're in test or not).
#[cfg(test)]
const MAPSIZE: Position = (7, 7);
#[cfg(not(test))]
const MAPSIZE: Position = (71, 71);

/// For part 1, how many rocks should we drop before finding a path.
#[cfg(test)]
const FALL_COUNT: usize = 12;
#[cfg(not(test))]
const FALL_COUNT: usize = 1024;

/// A map is a rectangular bounded grid of cells.
trait MutMap {
    /// Add a rock to the map at the given position.
    /// Could fail if the rock is out of bounds.
    fn add_rock(&mut self, rock: &Position) -> Result<()>;
}
trait Map {
    /// Check if the given position is a valid position to move to.
    fn can_move_to(&self, pos: &Position) -> bool;
    /// Return the bounds of the map.
    fn bound(&self) -> Position;
}

/// A container that can check if it contains a value.
///
/// This is used to abstract over the different types of containers
trait HashContainer<T> {
    /// Returns true if this container contains the given value.
    fn contains(&self, value: &T) -> bool;
    /// Insert the given value into the container.
    fn insert(&mut self, value: T) -> bool;
}

/// Am implementation of HashContainer for HashSets
impl<T> HashContainer<T> for HashSet<T>
where
    T: std::hash::Hash + Eq,
{
    fn contains(&self, value: &T) -> bool {
        HashSet::contains(self, value)
    }
    fn insert(&mut self, value: T) -> bool {
        HashSet::insert(self, value)
    }
}

/// An implementation of HashContainer for BTreeSets
impl<T> HashContainer<T> for BTreeSet<T>
where
    T: Ord,
{
    fn contains(&self, value: &T) -> bool {
        BTreeSet::contains(self, value)
    }
    fn insert(&mut self, value: T) -> bool {
        BTreeSet::insert(self, value)
    }
}

/// A bounded map is something that implements HashContainer
/// and has a fixed bound.
#[allow(dead_code)]
struct BoundedMap<T> {
    map: T,
    bounds: Position,
}

/// Implementation for creating new bounded maps
impl<T> BoundedMap<T>
where
    T: HashContainer<Position> + Default,
{
    #[allow(dead_code)]
    /// Create a new BoundedMap with the given bounds.
    fn new(bounds: Position) -> Self {
        Self {
            map: Default::default(),
            bounds,
        }
    }
}

/// Implement Map for BoundedMap
impl<T> MutMap for BoundedMap<T>
where
    T: HashContainer<Position>,
{
    fn add_rock(&mut self, rock: &Position) -> Result<()> {
        self.map.insert(*rock);
        Ok(())
    }
}

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

impl<C> MutMap for C
where
    C: GetCellMut<bool>,
{
    fn add_rock(&mut self, rock: &Position) -> Result<()> {
        let c = self.get_cell_mut_result(rock)?;
        *c = true;
        Ok(())
    }
}

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

/// Map can be implemented for a Vec<Vec<bool>>.
impl MutMap for Vec<Vec<bool>> {
    fn add_rock(&mut self, xy: &Position) -> Result<()> {
        self.as_mut_slice().add_rock(xy)
    }
}

impl Map for Vec<Vec<bool>> {
    fn bound(&self) -> Position {
        Map::bound(&self.as_slice())
    }
    fn can_move_to(&self, pos: &Position) -> bool {
        self.as_slice().can_move_to(pos)
    }
}

impl<const XBOUND: usize, const YBOUND: usize> MutMap for [[bool; XBOUND]; YBOUND] {
    fn add_rock(&mut self, xy: &Position) -> Result<()> {
        let mut s = self.as_mut_slice();
        let c = s.get_cell_mut_result(xy)?;
        *c = true;
        Ok(())
    }
}

impl<const XBOUND: usize, const YBOUND: usize> Map for [[bool; XBOUND]; YBOUND] {
    fn bound(&self) -> Position {
        (XBOUND, YBOUND)
    }
    fn can_move_to(&self, pos: &Position) -> bool {
        let s = self.as_slice();
        let c = s.get_cell(pos);
        if let Some(occupied) = c {
            !*occupied
        } else {
            // out of bounds, can't move there.
            false
        }
    }
}

trait PathFinder {
    fn find_path(&self, map: &impl Map) -> Result<Vec<Position>>;
}

fn solve_part1_impl(
    falling: impl Iterator<Item = Result<Position>>,
    mut map: impl Map,
    path_finder: impl PathFinder,
) -> Result<usize> {
    for rock in falling.take(FALL_COUNT) {
        map.add_rock(&rock?)?;
    }
    let path = path_finder.find_path(&map)?;
    Ok(path.len() - 1)
}

#[allow(dead_code)]
fn print_map(map: &impl Map) {
    let bound = map.bound();
    for y in 0..bound.1 {
        for x in 0..bound.0 {
            let c = if map.can_move_to(&(x, y)) { '.' } else { '#' };
            print!("{}", c);
        }
        println!();
    }
}

fn solve_part2_impl(
    falling: impl Iterator<Item = Result<Position>>,
    mut map: impl Map,
    path_finder: impl PathFinder,
) -> Result<Position> {
    let mut prev_path: Option<Vec<Position>> = None;

    for rock in falling {
        let rock = rock?;
        map.add_rock(&rock)?;
        // Don't try to compute this path if this rock didn't
        // block the already-best-path.
        if let Some(prev_path) = prev_path.as_ref() {
            if !prev_path.contains(&rock) {
                continue;
            }
        }
        // Find the path
        let path = path_finder.find_path(&map);
        // If we found a path, update the best path,
        // otherwise this rock blocked our path and is the answer.
        if let Ok(path) = path {
            prev_path = Some(path);
        } else {
            return Ok(rock);
        }
    }

    anyhow::bail!("no solution found");
}

#[allow(dead_code)]
struct FringePathFinder {
    start: Position,
}
impl PathFinder for FringePathFinder {
    fn find_path(&self, map: &impl Map) -> Result<Vec<Position>> {
        let end = map.bound();
        let end = (end.0 - 1, end.1 - 1);
        let shortest = pathfinding::directed::fringe::fringe(
            &self.start,
            |xy| valid_map_steps(map, *xy).map(add_cost),
            |_| 0,
            |coord| *coord == end,
        )
        .ok_or_else(|| anyhow::anyhow!("no path found"))?;
        Ok(shortest.0)
    }
}

#[allow(dead_code)]
struct DijkstraPathFinder {
    start: Position,
}
impl PathFinder for DijkstraPathFinder {
    fn find_path(&self, map: &impl Map) -> Result<Vec<Position>> {
        let end = map.bound();
        let end = (end.0 - 1, end.1 - 1);
        let shortest = pathfinding::directed::dijkstra::dijkstra(
            &self.start,
            |xy| valid_map_steps(map, *xy).map(add_cost),
            |coord| *coord == end,
        )
        .ok_or_else(|| anyhow::anyhow!("no path found"))?;
        Ok(shortest.0)
    }
}

#[allow(dead_code)]
struct AStarPathFinder {
    start: Position,
}
impl PathFinder for AStarPathFinder {
    fn find_path(&self, map: &impl Map) -> Result<Vec<Position>> {
        let end = map.bound();
        let end = (end.0 - 1, end.1 - 1);
        let shortest = pathfinding::directed::astar::astar(
            &self.start,
            |xy| valid_map_steps(map, *xy).map(add_cost),
            |_| 0,
            |coord| *coord == end,
        )
        .ok_or_else(|| anyhow::anyhow!("no path found"))?;
        Ok(shortest.0)
    }
}

/// Add a cost of 1 to the value for pathfinding.
fn add_cost<T>(value: T) -> (T, usize) {
    (value, 1)
}

/// Given a map and a current position, return an iterator of valid steps from that position.
///
/// A valid step is one that is within the bounds of the map and is not blocked by a rock.
/// Or more generically, a position that the map says we can move to.
fn valid_map_steps(map: &impl Map, cur_xy: Position) -> impl Iterator<Item = Position> {
    const DIRECTIONS: [Direction; 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    // Get the possible steps from the current position in each direction.
    let possible_steps = DIRECTIONS
        .iter()
        .filter_map(move |dir| add_xy(&cur_xy, dir));

    // Look at each possible step and take only the ones that are valid locations we can move to.
    let mut valid_positions = possible_steps.filter(move |new_xy| map.can_move_to(&new_xy));

    // There could be up to 4 of these (because 4 directions).  Compute now those that are valid.
    [
        valid_positions.next(),
        valid_positions.next(),
        valid_positions.next(),
        valid_positions.next(),
    ]
    .into_iter()
    .flatten()
}

/// Solution to part 1
#[aoc(day18, part1)]
fn solve_part1(input: &str) -> Result<usize> {
    let falling = parse(input);
    solve_part1_impl(falling, create_map(), create_finder())
}

fn create_map() -> impl Map {
    //BoundedMap::<BTreeSet<Position>>::new(MAPSIZE)
    //    BoundedMap::<HashSet<Position>>::new(MAPSIZE)
    //vec![vec![false; MAPSIZE.0]; MAPSIZE.1]
    [[false; MAPSIZE.0]; MAPSIZE.1]
}

fn create_finder() -> impl PathFinder {
    DijkstraPathFinder { start: (0, 0) }
    //AStarPathFinder { start: (0, 0) }
    //FringePathFinder { start: (0, 0) }
}

/// Solution to part 2
#[aoc(day18, part2)]
fn solve_part2(input: &str) -> Result<String> {
    let falling = parse(input);
    let solution = solve_part2_impl(falling, create_map(), create_finder())?;
    Ok(format!("{},{}", solution.0, solution.1))
}

fn parse(s: &str) -> impl Iterator<Item = Result<Position>> + '_ {
    let s = s.lines();
    let coords = s
        .map(|line| {
            line.split_once(",")
                .ok_or_else(|| anyhow::anyhow!("bad split"))
        })
        .map(|xy| {
            let (x, y) = xy?;
            Ok((x.parse()?, y.parse()?))
        });

    coords
}

/// codspeed compatible function
pub fn part1(input: &str) -> impl Display {
    solve_part1(input).unwrap()
}

/// codspeed compatible function
pub fn part2(input: &str) -> impl Display {
    solve_part2(input).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::test_data;
    use test_log::test;

    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(solve_part1(&test_data(super::DAY).unwrap()).unwrap(), 22);
    }

    #[test]
    fn part2_example() {
        assert_eq!(solve_part2(&test_data(super::DAY).unwrap()).unwrap(), "6,1");
    }
}
