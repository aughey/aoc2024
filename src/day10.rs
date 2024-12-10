use crate::Result;
use anyhow::Context as _;
use aoc_runner_derive::aoc;
use std::{collections::HashSet, fmt::Display, ops::Add};
use tracing::info;

pub const DAY: u32 = 10;

/// A trail score consisting of:
/// - count: The number of ends reachable by this trailhead.
/// - rating: The number of unique paths to the end reachable by this trailhead.
#[derive(Debug, Default)]
struct Score {
    count: usize,
    rating: usize,
}
impl Add for Score {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Score {
            count: self.count + other.count,
            rating: self.rating + other.rating,
        }
    }
}

/// Walk all the trail heads and return an accumulated score
/// for all the trails.  This generates scores for both part 1
/// and part 2 at the same time.
fn walk_trails(input: &Data) -> Result<Score> {
    Ok(input
        // For all the trail heads
        .trail_heads()
        // Compute a score for this trail head.
        .map(|head| {
            // Keep track of all the unique ends we reach
            let mut reached_ends = HashSet::new();
            // Recursively walk the trail, returns the rating and accumulates the ends
            let rating = recursive_walk(head, &mut reached_ends);
            // Count is the number of unique ends we reached
            let count = reached_ends.len();

            Score { count, rating }
        })
        .inspect(|v| info!("Count: {v:?}"))
        // The parts ask for an accumulated score.
        // the fold is the same as sum
        .fold(Score::default(), Score::add))
}

/// Recursively walk the trail starting at the given cell and walking
/// up until we are at cell 9.
///
/// Returns the number of unique paths that result in reaching an end.
/// Collects the coordinates of all reached ends in the `reached` set.
fn recursive_walk(cell: Cell, reached: &mut HashSet<XY>) -> usize {
    // Ending condition
    if cell.height() == 9 {
        reached.insert(cell.xy());
        return 1;
    }
    // The return value is accumulating how many times we reach the
    // end of the trail
    cell.next_trail_positions()
        .map(|next| recursive_walk(next, reached))
        .sum()
}

/// Solution to part 1 is walking the trails and returning count
fn solve_part1_impl(input: &Data) -> Result<usize> {
    Ok(walk_trails(input)?.count)
}

/// Solution to part 1 is walking the trails and returning rating
fn solve_part2_impl(input: &Data) -> Result<usize> {
    Ok(walk_trails(input)?.rating)
}

/// Solution to part 1
#[aoc(day10, part1)]
fn solve_part1(input: &str) -> Result<usize> {
    let input = Data::parse(input).context("input parsing")?;
    solve_part1_impl(&input)
}

/// Solution to part 2
#[aoc(day10, part2)]
fn solve_part2(input: &str) -> Result<usize> {
    let input = Data::parse(input).context("input parsing")?;
    solve_part2_impl(&input)
}

/// Our problem is a grid of heights.
type Grid = Vec<Vec<u8>>;
/// A coordinate of the grid
type XY = (usize, usize);

/// Problem input
#[derive(Debug)]
struct Data {
    // XXX: Change this to the actual data structure
    grid: Grid,
}
impl Data {
    /// Parse the input into the data structure.  This is the typical
    /// nested map pattern.
    fn parse(s: &str) -> Result<Self> {
        let s = s.lines();
        // Create the generator for the grid
        let grid = s.map(|line| {
            line.chars().map(|c| {
                // Convert the character to a digit, could be bad
                let digit = c.to_digit(10).ok_or_else(|| anyhow::anyhow!("bad digit"))?;
                Ok(digit.try_into()?)
            })
        });
        // Collect the grid into a 2D vector
        let grid = grid
            .map(|r| r.collect::<Result<Vec<_>>>())
            .collect::<Result<Vec<_>>>()?;

        Ok(Data { grid })
    }

    /// Provide an iterator that is all of the trail heads in this grid.
    fn trail_heads(&self) -> impl Iterator<Item = Cell> + '_ {
        let grid = &self.grid;
        // This is simply a nested row, column flat map with a filter.
        grid.iter().enumerate().flat_map(move |(y, row)| {
            row.iter()
                .enumerate()
                // Heads are height of 0
                .filter_map(move |(x, &height)| (height == 0).then(|| Cell { x, y, grid }))
        })
    }
}

/// A cell is a position on the grid
struct Cell<'a> {
    x: usize,
    y: usize,
    grid: &'a Grid,
}
impl Cell<'_> {
    /// Get the height of the cell
    fn height(&self) -> u8 {
        self.grid[self.y][self.x]
    }
    /// Get the XY coordinates of the cell
    fn xy(&self) -> XY {
        (self.x, self.y)
    }

    /// Get the next trail positions from this cell.
    /// The next trail positions are defined as the cells that are
    /// adjacent (left,right,up,down) to this cell and have a height
    /// that is one greater
    fn next_trail_positions(&self) -> impl Iterator<Item = Cell> {
        // The directions we could go in.
        const DIRECTIONS: &[(isize, isize)] = &[(0, -1), (0, 1), (-1, 0), (1, 0)];
        // The next height is the current height + 1
        let next_height = self.height().checked_add(1).unwrap();

        // Filter the directions to only those that are valid.
        // The short circuit `?` and `then` allows coordinates and
        // conditions to be checked without getting in the way of
        // the happy path.
        DIRECTIONS.iter().filter_map(move |(dx, dy)| {
            // Compute the next position and height with short circuiting.
            let x = self.x.checked_add_signed(*dx)?;
            let y = self.y.checked_add_signed(*dy)?;
            let height = self.grid.get(y)?.get(x)?;

            // If the height is the next height, return the cell
            (height == &next_height).then(|| Cell {
                x,
                y,
                grid: self.grid,
            })
        })
    }
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
        assert_eq!(
            solve_part1(&test_data(super::DAY).unwrap()).unwrap(),
            36 // XXX: Update this to the expected value for part 1 sample data.
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            solve_part2(&test_data(super::DAY).unwrap()).unwrap(),
            81 // XXX: Update this to the expected value for part 2 sample data.
        );
    }
}
