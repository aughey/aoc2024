use crate::Result;
use crate::StopMapClone;
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

    let word_to_find = WORD.iter().copied();

    Ok(all_cells
        // Compute a count of the number of times the word appears in each cell in all directions
        .map(|cell| {
            // Count the number of directions that have the word.
            DIRECTIONS
                .iter()
                .copied()
                // filter directions that have a matching word.
                .filter(|&direction| {
                    // create an iterator of chars in this direction.
                    let chars_in_this_direction = input
                        .cells_in_direction(cell.xy, direction)
                        // Take enough chars to match the word.
                        .take(WORD.len())
                        // Get the letter of each cell.
                        .map(|c| c.letter);
                    // A match if the chars in this direction are equal to the word.
                    chars_in_this_direction.eq(word_to_find.clone())
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
                    .cells_at_deltas(cell.xy, deltas.iter().copied())
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
    /// Returns an iterator of all the cells starting at (x,y) moving in the direction (dx,dy)
    pub fn cells_in_direction(
        &self,
        (x, y): (usize, usize),
        (dx, dy): (isize, isize),
    ) -> impl Iterator<Item = &Cell> + Clone {
        // Create an iterator of deltas in the provided direction.
        let deltas = (0..).stop_map(move |offset| {
            let dx = dx.checked_mul(offset)?;
            let dy = dy.checked_mul(offset)?;
            Some((dx, dy))
        });
        // Use our own `cells_at_deltas` function to get the cells.
        self.cells_at_deltas((x, y), deltas)
    }

    /// Returns an iterator of all the cells starting at (x,y) with deltas supplied by the provided iterator.
    /// Stops providing cells when a computed x or y is out of bounds or the iterator runs out of deltas.
    pub fn cells_at_deltas(
        &self,
        (x, y): (usize, usize),
        deltas: impl Iterator<Item = (isize, isize)> + Clone,
    ) -> impl Iterator<Item = &Cell> + Clone {
        deltas.stop_map(move |(dx, dy)| {
            // Compute x,y
            // Heavily lean on ? to stop the iterator when we go out of bounds.
            let x = x.checked_add_signed(dx)?;
            let y = y.checked_add_signed(dy)?;
            self.cells.get(y)?.get(x)
        })
        // .take_while(|c| c.is_some())
        // .map(|c| c.unwrap())
    }
}

#[derive(Debug)]
struct Cell {
    pub xy: (usize, usize),
    pub letter: char,
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
                    .map(|(x, letter)| Cell { xy: (x, y), letter })
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
