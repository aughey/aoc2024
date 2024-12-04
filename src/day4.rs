use crate::Result;
use anyhow::Context as _;
use aoc_runner_derive::{aoc, aoc_generator};
use std::{fmt::Display, str::FromStr};
use tracing::info;

pub const DAY: u32 = 4;

/// Parsing logic uses the FromStr trait
#[aoc_generator(day4)]
fn parse(input: &str) -> Result<Data> {
    info!("Parsing input");
    Data::from_str(input).context("input parsing")
}

/// Solution to part 1
#[aoc(day4, part1)]
fn solve_part1(input: &Data) -> Result<usize> {
    const WORD: &[char] = &['X', 'M', 'A', 'S'];
    const DIRECTIONS: &[(isize, isize)] = &[
        (0, 1),
        (1, 0),
        (1, 1),
        (0, -1),
        (-1, 0),
        (-1, -1),
        (1, -1),
        (-1, 1),
    ];

    let cells = &input.cells;
    let all_cells = cells.iter().flat_map(|row| row.iter());

    Ok(all_cells
        .map(|cell| {
            DIRECTIONS
                .iter()
                .copied()
                // filter directions that have a matching word.
                .filter(|&direction| {
                    // create an iterator of chars in this direction.
                    let chars_in_this_direction = input
                        .cells_in_direction(cell.xy(), direction)
                        .map(|c| c.letter)
                        .take(WORD.len());
                    // A match if the chars in this direction are equal to the word.
                    chars_in_this_direction.eq(WORD.iter().copied())
                })
                // Count the number of matches.
                .count()
        })
        // Sum all the matches for all cells.
        .sum())
}

/// Solution to part 2
#[aoc(day4, part2)]
fn solve_part2(input: &Data) -> Result<usize> {
    let cells = &input.cells;
    let all_cells = cells.iter().flat_map(|row| row.iter());

    // Valid words are MAS and SAM
    const WORDS: [[char; 3]; 2] = [['M', 'A', 'S'], ['S', 'A', 'M']];
    // These are the two diagonals we need to check
    const DIAGONAL_DELTAS: &[&[(isize, isize)]] =
        &[&[(0, 0), (1, 1), (2, 2)], &[(0, 2), (1, 1), (2, 0)]];

    Ok(all_cells
        // At each cell we build the diagonals and check if they are valid words.
        // Filter out any cell that doesn't have a valid X
        .filter(|cell| {
            // Build iterators of chars along the diagonals
            let mut diagonals_to_test = DIAGONAL_DELTAS.iter().map(|deltas| {
                input
                    .cells_at_deltas(cell.xy(), deltas.iter().copied())
                    .map(|c| c.letter)
            });
            // This cell if valid if all of the diagonals match one of the words.
            diagonals_to_test.all(|diagonal| {
                WORDS
                    .iter()
                    // Tests if any word matches the diagonal
                    .any(|&word| word.iter().copied().eq(diagonal.clone()))
            })
        })
        // Count up all the cells that have a valid X
        .count())
}

/// Problem input
#[derive(Debug)]
struct Data {
    cells: Vec<Vec<Cell>>,
}
impl Data {
    pub fn cells_in_direction(
        &self,
        (x, y): (usize, usize),
        (dx, dy): (isize, isize),
    ) -> impl Iterator<Item = &Cell> + Clone {
        let deltas = (0..)
            .map(move |i| {
                let dx = dx.checked_mul(i)?.try_into().ok()?;
                let dy = dy.checked_mul(i)?.try_into().ok()?;
                Some((dx, dy))
            })
            .take_while(|c| c.is_some())
            .map(|c| c.unwrap());
        self.cells_at_deltas((x, y), deltas)
    }
    pub fn cells_at_deltas(
        &self,
        (x, y): (usize, usize),
        deltas: impl Iterator<Item = (isize, isize)> + Clone,
    ) -> impl Iterator<Item = &Cell> + Clone {
        deltas
            .map(move |(dx, dy)| {
                let x = x.checked_add_signed(dx)?;
                let y = y.checked_add_signed(dy)?;
                self.cells.get(y)?.get(x)
            })
            .take_while(|c| c.is_some())
            .map(|c| c.unwrap())
    }
}

#[derive(Debug)]
struct Cell {
    pub x: usize,
    pub y: usize,
    pub letter: char,
}
impl Cell {
    pub fn xy(&self) -> (usize, usize) {
        (self.x, self.y)
    }
}

impl FromStr for Data {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let s = s.lines();

        let cells = s
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, letter)| Cell { x, y, letter })
                    .collect()
            })
            .collect();

        Ok(Data { cells })
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
            18
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            solve_part2(&parse(&test_data(super::DAY).unwrap()).unwrap()).unwrap(),
            9
        );
    }
}
