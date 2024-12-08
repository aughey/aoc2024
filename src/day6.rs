use crate::Result;
use anyhow::Context as _;
use aoc_runner_derive::{aoc, aoc_generator};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator as _};
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    str::FromStr,
};
use tracing::info;

pub const DAY: u32 = 6;

/// Parsing logic uses the FromStr trait
#[aoc_generator(day6)]
fn parse(input: &str) -> Result<Data> {
    info!("Parsing input");
    Data::from_str(input).context("input parsing")
}

/// Given a grid of cells, a current position, and a direction, return the next cell and position.
fn get_next_cell<V>(
    cells: &[V],
    pos: (usize, usize),
    direction: (isize, isize),
) -> Option<(&Cell, (usize, usize))>
where
    V: AsRef<[Cell]>,
{
    let next_pos = (
        pos.0.checked_add_signed(direction.0)?,
        pos.1.checked_add_signed(direction.1)?,
    );
    let cell = cells.get(next_pos.1)?.as_ref().get(next_pos.0)?;

    Some((cell, next_pos))
}

/// Solution to part 1
#[aoc(day6, part1)]
fn solve_part1(input: &Data) -> Result<usize> {
    let seen: SeenMap = run_part1(input)
        .try_into_seenmap()
        .ok_or_else(|| anyhow::anyhow!("No seen map found for part 1"))?;
    Ok(seen.len())
}

/// Points we've seen stored in a HashMap with a HashSet of directions we've were facing in that point
type SeenMap = HashMap<Position, HashSet<Direction>>;
/// Position is a tuple of x, y
type Position = (usize, usize);
/// Direction is a tuple of x, y isizes with +y down
type Direction = (isize, isize);

/// Given a grid of cells and a starting position, create an iterator that will walk the map
/// providing a position and direction of each step.
fn walk_map<V>(cells: &[V], start_pos: Position) -> impl Iterator<Item = (Position, Direction)> + '_
where
    V: AsRef<[Cell]>,
{
    // Try to take a step from the current position in the given direction.
    let try_step = move |(pos, mut direction)| {
        // You can turn up to 4 times before it's a failure
        for _ in 0..4 {
            // See what the next valid cell is in that direction
            let (next_cell, next_pos) = get_next_cell(cells, pos, direction)?;
            // If the next cell is empty, we can move there.
            if next_cell == &Cell::Empty {
                // move there
                return Some((next_pos, direction));
            } else {
                // change direction and try again
                // 90 degree direction change (with y down) is swap x and -y
                direction = (-direction.1, direction.0);
            }
        }
        // We've tried all directions and failed, return none
        None
    };

    // Start at the start position and go up
    let mut pos_dir = (start_pos, (0, -1));
    std::iter::from_fn(move || {
        // Try to take a step and remember where we are.
        pos_dir = try_step(pos_dir)?;
        // We did it, we did it, we did it, yay!
        Some(pos_dir)
    })
}

/// The result of walking the map will either be a loop or we walked off the map.
#[derive(Debug, Clone, PartialEq)]
enum WalkResult {
    Loop,
    OffMap(SeenMap),
}
impl WalkResult {
    /// If this has a SeenMap component, return the seen map
    pub fn try_into_seenmap(self) -> Option<SeenMap> {
        match self {
            WalkResult::Loop => None,
            WalkResult::OffMap(seen) => Some(seen),
        }
    }
}

/// Do the operation of running part 1 and return the grid of seen cells
fn run_part1(input: &Data) -> WalkResult {
    // Keep track of cells we've been to and the direction we were traveling in that cell.
    let mut seen = SeenMap::new();

    // Walk the map getting each position and direction of each step.
    let positions = walk_map(&input.cells, input.start_point);
    // This is like a reduce operation that can short circuit if we've been to a cell in the same direction
    for (pos, dir) in positions {
        let cell_directions = seen.entry(pos).or_default();

        // Insert returns false if this direction is already in the set.
        // (we've already been in this cell going in the same direction)
        if !cell_directions.insert(dir) {
            //  If we're been at this cell in the same direction, we're in a loop and return none
            return WalkResult::Loop;
        }
    }

    // We fell of the edge of the grid, return the seen map of points we've visited
    WalkResult::OffMap(seen)
}

/// Solution to part 2
#[aoc(day6, part2)]
fn solve_part2(input: &Data) -> Result<usize> {
    // do a walk of the current map
    let seen: SeenMap = run_part1(input)
        .try_into_seenmap()
        .ok_or_else(|| anyhow::anyhow!("No valid walk found for part 2"))?;
    // The keys of the seen map are the xy positions we've been to
    let walk_locations = seen.keys().collect::<Vec<_>>();

    // For each walk location, put an obstacle there and try to walk again
    let results_with_obstacle = walk_locations.par_iter().map(|(x, y)| {
        // Duplicate our map
        let mut cells = input.cells.clone();
        // throw caution to the wind
        cells[*y][*x] = Cell::Filled;

        run_part1(&Data {
            cells,
            start_point: input.start_point,
        })
    });

    // Only count the number of loops
    let loop_maps = results_with_obstacle.filter(|result| matches!(result, WalkResult::Loop));

    Ok(loop_maps.count())
}

/// Cell is either empty or filled
#[derive(Debug, Clone, PartialEq)]
enum Cell {
    Empty,
    Filled,
}

/// Create a cell from a character (for parsing)
impl TryFrom<char> for Cell {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self> {
        match value {
            '.' | '^' => Ok(Cell::Empty),
            '#' => Ok(Cell::Filled),
            _ => Err(anyhow::anyhow!("Invalid cell")),
        }
    }
}

/// Problem input
#[derive(Debug)]
struct Data {
    /// An xy grid of cells
    cells: Vec<Vec<Cell>>,
    /// The starting point of the walker
    start_point: (usize, usize),
}

/// Parse our data from a string blob that looks like:
/// ```pre
/// .#..#
/// ^....
/// .#...
/// ```
impl FromStr for Data {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self> {
        let s = input.lines();
        // Parse the cells, mapping each line to a vector of cells
        let cells = s
            .map(|line| line.chars().map(Cell::try_from).collect::<Result<Vec<_>>>())
            .collect::<Result<Vec<_>>>()?;

        // Find the start point in a seperate iteration
        let s = input.lines();
        // Find the x y location of ^ in our input
        let start_point = s
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().filter_map(
                    move |(x, c)| {
                        if c == '^' {
                            Some((x, y))
                        } else {
                            None
                        }
                    },
                )
            })
            .next()
            .ok_or_else(|| anyhow::anyhow!("No start point found"))?;

        Ok(Data { cells, start_point })
    }
}

/// codspeed compatible function
pub fn part1(input: &str) -> impl Display {
    solve_part1(&parse(input).unwrap()).unwrap()
}

/// codspeed compatible function
pub fn part2(input: &str) -> impl Display {
    solve_part2(&parse(input).unwrap()).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::test_data;
    use test_log::test;

    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            solve_part1(&parse(&test_data(super::DAY).unwrap()).unwrap()).unwrap(),
            41
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            solve_part2(&parse(&test_data(super::DAY).unwrap()).unwrap()).unwrap(),
            6
        );
    }
}
