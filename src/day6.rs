use crate::Result;
use anyhow::Context as _;
use aoc_runner_derive::{aoc, aoc_generator};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator as _};
use std::{collections::HashSet, fmt::Display, str::FromStr};
use tracing::info;

pub const DAY: u32 = 6;

/// Parsing logic uses the FromStr trait
#[aoc_generator(day6)]
fn parse(input: &str) -> Result<Data> {
    info!("Parsing input");
    Data::from_str(input).context("input parsing")
}

fn walk_to_next_cell(
    cells: &[Vec<Cell>],
    pos: (usize, usize),
    direction: (isize, isize),
) -> Option<(&Cell, (usize, usize))> {
    let next_pos = (
        pos.0.checked_add_signed(direction.0)?,
        pos.1.checked_add_signed(direction.1)?,
    );
    let cell = cells.get(next_pos.1).and_then(|row| row.get(next_pos.0))?;

    Some((cell, next_pos))
}

/// Solution to part 1
#[aoc(day6, part1)]
fn solve_part1(input: &Data) -> Result<usize> {
    let seen = run_part1(input)?.unwrap();
    let seen_count = seen.iter().flatten().filter(|c| !c.is_empty()).count();
    Ok(seen_count)
}

type SeenList = Vec<Vec<HashSet<(isize, isize)>>>;

fn run_part1(input: &Data) -> Result<Option<SeenList>> {
    // XXX: Solving logic for part 1
    let cells = &input.cells;
    // create a grid of seen cells
    let mut seen: SeenList = input
        .cells
        .iter()
        .map(|row| row.iter().map(|_| Default::default()).collect())
        .collect();

    let mut pos = input.start_point;
    let mut direction = (0isize, -1isize);
    loop {
        // If we're been at this cell in the same direction, we're in a loop and return none
        if seen[pos.1][pos.0].contains(&direction) {
            return Ok(None);
        }

        // mark out current position
        seen[pos.1][pos.0].insert(direction);

        // get the cell at the current position
        if let Some(next) = walk_to_next_cell(cells, pos, direction) {
            let (next_cell, next_pos) = next;
            if matches!(next_cell, Cell::Empty) {
                // move there
                pos = next_pos;
            } else {
                // change direction
                direction = match direction {
                    (0, 1) => (-1, 0),
                    (-1, 0) => (0, -1),
                    (0, -1) => (1, 0),
                    (1, 0) => (0, 1),
                    _ => anyhow::bail!("Bad direction"),
                }
            }
        } else {
            break;
        }
    }

    Ok(Some(seen))
}

/// Solution to part 2
#[aoc(day6, part2)]
fn solve_part2(input: &Data) -> Result<usize> {
    // do a walk of the current map
    let seen = run_part1(input)?.ok_or_else(|| anyhow::anyhow!("No solution for part 1"))?;
    let walk_locations =
        seen.iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter().enumerate().filter_map(move |(x, cell)| {
                    if !cell.is_empty() {
                        Some((x, y))
                    } else {
                        None
                    }
                })
            })
            .collect::<Vec<_>>();

    // For each walk location, put an obstical there and try to walk again
    let loop_maps = walk_locations.par_iter().filter(|(x, y)| {
        let mut cells = input.cells.clone();
        // throw caution to the wind
        cells[*y][*x] = Cell::Filled;
        run_part1(&Data {
            cells,
            start_point: input.start_point,
        })
        .unwrap()
        .is_none()
    });

    Ok(loop_maps.count())
}

#[derive(Debug, Clone)]
enum Cell {
    Empty,
    Filled,
}
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
    // XXX: Change this to the actual data structure
    cells: Vec<Vec<Cell>>,
    start_point: (usize, usize),
}

impl FromStr for Data {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self> {
        let s = input.lines();
        let cells = s
            .map(|line| line.chars().map(Cell::try_from).collect::<Result<Vec<_>>>())
            .collect::<Result<Vec<_>>>()?;
        // Find the start point in a seperate iteration
        let s = input.lines();
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
            41 // XXX: Update this to the expected value for part 1 sample data.
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            solve_part2(&parse(&test_data(super::DAY).unwrap()).unwrap()).unwrap(),
            6 // XXX: Update this to the expected value for part 2 sample data.
        );
    }
}
