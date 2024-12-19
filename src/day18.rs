mod generics;
use crate::{add_xy, Direction, Position, Result};
use aoc_runner_derive::aoc;
use generics::{HashContainer, Map, MutMap};
use std::fmt::Display;

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

/// A PathFinder is something that can find a path through a map.
pub trait PathFinder {
    fn find_path(&self, map: &impl Map) -> Result<Vec<Position>>;
}

fn solve_part1_impl(
    falling_rocks: impl Iterator<Item = Result<Position>>,
    mut map: impl Map + MutMap,
    path_finder: impl PathFinder,
) -> Result<usize> {
    // take fall count rocks from the iterator.
    let rocks_to_add = falling_rocks.take(FALL_COUNT);

    // Add all of these rocks to the map
    for rock in rocks_to_add {
        map.add_rock(rock?)?;
    }

    // Find the path.
    let path = path_finder.find_path(&map)?;

    // Return the length of the path (don't include the last step)
    path.len()
        .checked_sub(1)
        .ok_or_else(|| anyhow::anyhow!("empty path found"))
}

/// To keep our solutions reading like words, a PreviousPath trait
/// will keep track of previous paths and answer questions about them.
trait PreviousPath {
    /// A query, will the path we're remembering be affected by this rock?
    fn will_be_affected_by(&self, rock_at: &Position) -> bool;
    /// Remember this path for future queries.
    fn remember_path(&mut self, path: Vec<Position>);

    /// A simple negation of the above to make logic easier to read.
    fn will_not_be_affected_by(&self, rock_at: &Position) -> bool {
        !self.will_be_affected_by(rock_at)
    }
}

// /// Implement PathRemember for an Optional Vec<Position>.
// impl PreviousPath for Option<Vec<Position>> {
//     fn will_be_affected_by(&self, rock_at: &Position) -> bool {
//         // If we have a path...
//         if let Some(prev_path) = self.as_ref() {
//             // ...and the new rock is in the path, then it will affect the best path.
//             prev_path.contains(rock_at)
//         } else {
//             // If we don't have a path then by definition it is affected.
//             true
//         }
//     }

//     // Replace the path with a new one.
//     fn remember_path(&mut self, path: Vec<Position>) {
//         self.replace(path);
//     }
// }

impl<T> PreviousPath for Option<T>
where
    T: HashContainer<Position> + FromIterator<Position>,
{
    fn will_be_affected_by(&self, rock_at: &Position) -> bool {
        if let Some(prev_path) = self.as_ref() {
            prev_path.contains(rock_at)
        } else {
            true
        }
    }

    fn remember_path(&mut self, path: Vec<Position>) {
        self.replace(path.into_iter().collect());
    }
}

fn solve_part2_impl(
    falling: impl Iterator<Item = Result<Position>>,
    mut map: impl Map + MutMap,
    path_finder: impl PathFinder,
) -> Result<Position> {
    // Keep track of the path of a previous iteration in order
    // to avoid recomputing the path if it's not affected by a new rock.
    let mut prev_path: Option<Vec<Position>> = None;

    // Continue to add rocks to the map and try to find a path.
    for rock in falling {
        // Add the rock to the map.
        let rock = rock?;
        map.add_rock(rock)?;

        // If our previously computed path isn't affected by this
        // rock, then we don't need to recompute the path.
        if prev_path.will_not_be_affected_by(&rock) {
            continue;
        }

        let best_path_to_end = path_finder.find_path(&map);
        if let Ok(path) = best_path_to_end {
            // If we found a path, remember it for next time.
            prev_path.remember_path(path);
        } else {
            // No path found.  This rock is the answer.
            return Ok(rock);
        }
    }

    anyhow::bail!("no solution found");
}

/// Print the contents of the map.
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

/// Path finding using the Fringe algorithm.
#[allow(dead_code)]
struct FringePathFinder {}
impl PathFinder for FringePathFinder {
    fn find_path(&self, map: &impl Map) -> Result<Vec<Position>> {
        let (start, end) = (map.start(), map.end());
        let shortest = pathfinding::directed::fringe::fringe(
            &start,
            |xy| valid_map_steps(map, *xy).map(add_cost),
            |_| 0,
            |coord| *coord == end,
        )
        .ok_or_else(|| anyhow::anyhow!("no path found"))?;
        Ok(shortest.0)
    }
}

/// Path finding using the Dijkstra algorithm.
#[allow(dead_code)]
struct DijkstraPathFinder {}
impl PathFinder for DijkstraPathFinder {
    fn find_path(&self, map: &impl Map) -> Result<Vec<Position>> {
        let (start, end) = (map.start(), map.end());
        let shortest = pathfinding::directed::dijkstra::dijkstra(
            &start,
            |xy| valid_map_steps(map, *xy).map(add_cost),
            |coord| *coord == end,
        )
        .ok_or_else(|| anyhow::anyhow!("no path found"))?;
        Ok(shortest.0)
    }
}

/// Path finding using the A* algorithm.
#[allow(dead_code)]
struct AStarPathFinder {}
impl PathFinder for AStarPathFinder {
    fn find_path(&self, map: &impl Map) -> Result<Vec<Position>> {
        let (start, end) = (map.start(), map.end());
        let shortest = pathfinding::directed::astar::astar(
            &start,
            |xy| valid_map_steps(map, *xy).map(add_cost),
            |_| 0,
            |coord| *coord == end,
        )
        .ok_or_else(|| anyhow::anyhow!("no path found"))?;
        Ok(shortest.0)
    }
}

/// Helper function to add a cost of 1 to the value for pathfinding.
fn add_cost<T>(value: T) -> (T, usize) {
    (value, 1)
}

/// Given a map and a current position, return an iterator of valid steps from that position.
///
/// A valid step is one that is within the bounds of the map and is not blocked by a rock.
/// Or more generically, a position that the map says we can move to.
fn valid_map_steps(map: &impl Map, cur_xy: Position) -> impl Iterator<Item = Position> {
    // All directions we could go.
    const DIRECTIONS: [Direction; 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    // Get the possible steps from the current position in each direction.
    let possible_steps = DIRECTIONS
        .iter()
        .filter_map(move |dir| add_xy(&cur_xy, dir));

    // Filter our possible steps by those that are valid to move to in our map.
    let mut valid_positions = possible_steps.filter(move |new_xy| map.can_move_to(new_xy));

    // There could be up to 4 of these (because 4 directions).  Fill them with the next
    // valid positions.
    [
        valid_positions.next(),
        valid_positions.next(),
        valid_positions.next(),
        valid_positions.next(),
    ]
    .into_iter()
    // Filter out the None values.
    .flatten()
}

/// Create a map (multiple possible options here).
fn create_map() -> impl Map + MutMap {
    //use generics::BoundedMap;
    //use std::collections::BTreeSet;
    //BoundedMap::<BTreeSet<Position>>::new(MAPSIZE) // Occupied positions backed by a BTreeSet
    // use std::collections::HashSet;
    // BoundedMap::<HashSet<Position>>::new(MAPSIZE) // Occupied positions backed by a hash set
    //BoundedMap::<Vec<Position>>::new(MAPSIZE) // Occupied positions backed by a Vec
    vec![vec![false; MAPSIZE.0]; MAPSIZE.1] // 2D array represented by vectors
                                            //[[false; MAPSIZE.0]; MAPSIZE.1] // 2D array
}

/// Create a path finder (multiple possible options here).
fn create_finder() -> impl PathFinder {
    DijkstraPathFinder {}
    //AStarPathFinder{}
    //FringePathFinder{}
}

/// Solution to part 1
#[aoc(day18, part1)]
fn solve_part1(input: &str) -> Result<usize> {
    solve_part1_impl(parse(input), create_map(), create_finder())
}

/// Solution to part 2
#[aoc(day18, part2)]
fn solve_part2(input: &str) -> Result<String> {
    let falling = parse(input);
    let solution = solve_part2_impl(falling, create_map(), create_finder())?;
    Ok(format!("{},{}", solution.0, solution.1))
}

/// Parse the input, providing an iterator that provides positions.
/// The parsing could fail, so each position is a fallible result.
fn parse(s: &str) -> impl Iterator<Item = Result<Position>> + '_ {
    // Each line has an x,y coordinate.
    let s = s.lines();
    let coords = s
        // Split the line in two separated by a comma.
        .map(|line| {
            line.split_once(",")
                .ok_or_else(|| anyhow::anyhow!("bad split"))
        })
        // Parse the two values into a position.
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
    use std::collections::HashSet;

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

    fn test_affected_by_generic<T: PreviousPath + Default>() {
        let mut prev_path = T::default();
        // always starts with returning true
        assert_eq!(prev_path.will_be_affected_by(&(0, 0)), true);
        assert_eq!(prev_path.will_be_affected_by(&(1, 1)), true);
        // remember a path
        prev_path.remember_path(vec![(0, 0), (1, 1)]);
        // now it should return true for the path remembered
        assert_eq!(prev_path.will_be_affected_by(&(0, 0)), true);
        assert_eq!(prev_path.will_be_affected_by(&(1, 1)), true);
        // but not for others
        assert_eq!(prev_path.will_be_affected_by(&(0, 1)), false);
        assert_eq!(prev_path.will_be_affected_by(&(1, 0)), false);
        // try another
        prev_path.remember_path(vec![(0, 1), (1, 0)]);
        // now it should return true for the path remembered
        assert_eq!(prev_path.will_be_affected_by(&(0, 1)), true);
        assert_eq!(prev_path.will_be_affected_by(&(1, 0)), true);
        // and not the one before
        assert_eq!(prev_path.will_be_affected_by(&(0, 0)), false);
        assert_eq!(prev_path.will_be_affected_by(&(1, 1)), false);
    }

    #[test]
    fn test_affected_by() {
        test_affected_by_generic::<Option<Vec<Position>>>();
        test_affected_by_generic::<Option<HashSet<Position>>>();
    }
}
