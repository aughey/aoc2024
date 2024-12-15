use crate::Result;
use anyhow::Context as _;
use aoc_runner_derive::aoc;
use std::fmt::Display;
use tracing::info;

pub const DAY: u32 = 15;

trait Get {
    fn get_cell(&self, xy: (usize, usize)) -> Option<&Cell>;
    fn get_cell_mut(&mut self, xy: (usize, usize)) -> Option<&mut Cell>;
}
impl Get for Vec<Vec<Cell>> {
    fn get_cell(&self, xy: (usize, usize)) -> Option<&Cell> {
        Some(self.get(xy.1)?.get(xy.0)?)
    }
    fn get_cell_mut(&mut self, xy: (usize, usize)) -> Option<&mut Cell> {
        Some(self.get_mut(xy.1)?.get_mut(xy.0)?)
    }
}

fn playerxy(map: &Vec<Vec<Cell>>) -> Option<(usize, usize)> {
    map.iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter().enumerate().filter_map(move |(x, cell)| {
                if let Cell::Player = cell {
                    Some((x, y))
                } else {
                    None
                }
            })
        })
        .next()
}

fn solve_part1_impl(input: &Data) -> Result<usize> {
    let mut map = input.map.clone();
    let player_xy = playerxy(&map).ok_or_else(|| anyhow::anyhow!("player not found"))?;

    let mut player_xy = player_xy;

    for m in input.movements.iter() {
        let direction = match m {
            Movement::Up => (0, -1),
            Movement::Down => (0, 1),
            Movement::Left => (-1, 0),
            Movement::Right => (1, 0),
        };

        if let Ok(xy) = move_cell(&mut map, player_xy, direction) {
            player_xy = xy;
        }
    }

    // evaluate the map
    let boxes = map.iter().enumerate().flat_map(|(y, row)| {
        row.iter()
            .enumerate()
            .filter_map(move |(x, cell)| matches!(cell, Cell::Box).then(|| (x, y)))
    });
    let gps = boxes.map(|(x, y)| x + 100 * y).sum();

    Ok(gps)
}

fn print_map(map: &Vec<Vec<Cell>>) {
    for row in map {
        for cell in row {
            let c = match cell {
                Cell::Wall => '#',
                Cell::Box => 'O',
                Cell::Empty => '.',
                Cell::Player => '@',
                Cell::BoxLeft => '[',
                Cell::BoxRight => ']',
            };
            print!("{}", c);
        }
        println!();
    }
}
type Position = (usize, usize);
struct Movement {
    from: Position,
    to: Position,
}

fn move_cell(
    map: &mut Vec<Vec<Cell>>,
    from: (usize, usize),
    direction: (isize, isize),
) -> Result<Vec<Movement>> {
    let newxy = add_xy(from, direction).ok_or_else(|| anyhow::anyhow!("invalid movement"))?;
    let me = map
        .get_cell(from)
        .ok_or_else(|| anyhow::anyhow!("no cell"))?
        .clone();

    let nextcell = map
        .get_cell(newxy)
        .ok_or_else(|| anyhow::anyhow!("no cell"))?
        .clone();

    // Easy case for left and right
    if direction == (1, 0) || direction == (-1, 0) {
        match me {
            Cell::BoxLeft => {
                if direction == (0, 1) {
                    // try moving the box right
                    assert!(matches!(nextcell, Cell::BoxRight));
                    let mut movements = move_cell(map, newxy, direction)?;
                    *map.get_cell_mut(from).unwrap() = Cell::Empty;
                    *map.get_cell_mut(newxy).unwrap() = Cell::BoxLeft;
                    movements.push(Movement { from, to: newxy });
                    return Ok(movements);
                }
            }
            Cell::BoxRight => {
                if direction == (0, -1) {
                    // try moving the box left
                    assert!(matches!(nextcell, Cell::BoxLeft));
                    let movements = move_cell(map, newxy, direction)?;
                    *map.get_cell_mut(from).unwrap() = Cell::Empty;
                    *map.get_cell_mut(newxy).unwrap() = Cell::BoxRight;
                    movements.push(Movement { from, to: newxy });
                }
            }
            Cell::Empty => return Ok(vec![]),
            Cell::Box => {
                // Try to move the box
                let movements = move_cell(map, newxy, direction)?;
                *map.get_cell_mut(from).unwrap() = Cell::Empty;
                *map.get_cell_mut(newxy).unwrap() = Cell::Box;
                movements.push(Movement { from, to: newxy });
                return Ok(movements);
            }
            _ => (),
        }
    }

    // Look at that cell and make way
    let movement = match nextcell {
        Cell::Wall => anyhow::bail!("cannot move into wall"),
        Cell::Box => {
            // Try to move the box
            (move_cell(map, newxy, direction), Ok(vec![]))
        }
        Cell::Empty => {
            // good
            (Ok(vec![]), Ok(vec![]))
        }
        Cell::Player => panic!("Shouldn't call move_cell to a player"),
        Cell::BoxLeft => {
            // try moving the box left
            let left = move_cell(map, newxy, direction);
            // try moving box right
            let right = move_cell(map, add_xy(newxy, (1, 0)).unwrap(), direction);
            (left, right)
        }
        Cell::BoxRight => {
            let left = move_cell(map, add_xy(newxy, (-1, 0)).unwrap(), direction);
            let right = move_cell(map, newxy, direction);
            (left, right)
        }
    };

    match (movement.0, movement.1) {
        (Ok(movements), Err(_)) => {
            // Unmove
            // move the box
            *map.get_cell_mut(from).unwrap() = Cell::Empty;
            *map.get_cell_mut(newxy).unwrap() = Cell::Box;
            let mut movements = movements;
            movements.push(newxy);
            Ok(movements)
        }
        (Err(_), Ok(movements)) => {
            // move the box
            *map.get_cell_mut(from).unwrap() = Cell::Empty;
            *map.get_cell_mut(newxy).unwrap() = Cell::Box;
            let mut movements = movements;
            movements.push(newxy);
            Ok(movements)
        }
        (Err(e), Err(_)) => Err(e),
        (Ok(_), Ok(_)) => panic!("invalid movement"),
    }

    let from_cell = map
        .get_cell_mut(from)
        .ok_or_else(|| anyhow::anyhow!("from cell not found"))?;

    let cell_value = from_cell.clone();
    *from_cell = Cell::Empty;

    let to_cell = map
        .get_cell_mut(newxy)
        .ok_or_else(|| anyhow::anyhow!("to cell not found"))?;

    assert!(matches!(*to_cell, Cell::Empty));
    *to_cell = cell_value;

    Ok(newxy)
}

fn add_xy(xy: (usize, usize), direction: (isize, isize)) -> Option<(usize, usize)> {
    Some((
        xy.0.checked_add_signed(direction.0)?,
        xy.1.checked_add_signed(direction.1)?,
    ))
}

fn solve_part2_impl(input: &Data) -> Result<usize> {
    let mut map = input
        .map
        .iter()
        .map(|row| {
            row.iter()
                .flat_map(|cell| {
                    match cell {
                        Cell::Wall => [Cell::Wall, Cell::Wall],
                        Cell::Box => [Cell::BoxLeft, Cell::BoxRight],
                        Cell::Empty => [Cell::Empty, Cell::Empty],
                        Cell::Player => [Cell::Player, Cell::Empty],
                        Cell::BoxLeft | Cell::BoxRight => panic!("invalid cell"),
                    }
                    .into_iter()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    print_map(&map);

    let mut player_xy = playerxy(&map).ok_or_else(|| anyhow::anyhow!("player not found"))?;

    for m in input.movements.iter() {
        let direction = match m {
            Movement::Up => (0, -1),
            Movement::Down => (0, 1),
            Movement::Left => (-1, 0),
            Movement::Right => (1, 0),
        };

        if let Ok(xy) = move_cell(&mut map, player_xy, direction) {
            player_xy = xy;
        }
    }

    Ok(1)
}

/// Solution to part 1
#[aoc(day15, part1)]
fn solve_part1(input: &str) -> Result<usize> {
    let input = Data::parse(input).context("input parsing")?;
    solve_part1_impl(&input)
}

/// Solution to part 2
#[aoc(day15, part2)]
fn solve_part2(input: &str) -> Result<usize> {
    let input = Data::parse(input).context("input parsing")?;
    solve_part2_impl(&input)
}

#[derive(Debug, Clone)]
enum Cell {
    Wall,
    Box,
    BoxLeft,
    BoxRight,
    Empty,
    Player,
}
impl TryFrom<char> for Cell {
    type Error = anyhow::Error;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(Cell::Wall),
            'O' => Ok(Cell::Box),
            '.' => Ok(Cell::Empty),
            '@' => Ok(Cell::Player),
            _ => anyhow::bail!("invalid cell {}", value),
        }
    }
}

#[derive(Debug)]
enum Movement {
    Up,
    Down,
    Left,
    Right,
}
impl TryFrom<char> for Movement {
    type Error = anyhow::Error;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '^' => Ok(Movement::Up),
            'v' => Ok(Movement::Down),
            '<' => Ok(Movement::Left),
            '>' => Ok(Movement::Right),
            _ => anyhow::bail!("invalid movement"),
        }
    }
}

/// Problem input
#[derive(Debug)]
struct Data {
    map: Vec<Vec<Cell>>,
    movements: Vec<Movement>,
}

impl Data {
    fn parse(s: &str) -> Result<Self> {
        // split s into two things separated by a blank line
        let mut parts = s.split("\n\n");
        let mapcontent = parts.next().ok_or_else(|| anyhow::anyhow!("missing map"))?;
        let movementscontent = parts
            .next()
            .ok_or_else(|| anyhow::anyhow!("missing movements"))?;

        // parse map
        let map = mapcontent
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| Cell::try_from(c).context("invalid cell"))
                    .collect::<Result<Vec<_>>>()
            })
            .collect::<Result<Vec<_>>>()?;

        // parse movements
        let movements = movementscontent
            .lines()
            .flat_map(|line| {
                line.chars()
                    .map(|c| Movement::try_from(c).context("invalid movement"))
            })
            .collect::<Result<Vec<_>>>()?;

        // XXX: Update the returned Data to include the parsed data.
        Ok(Data { map, movements })
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
        assert_eq!(solve_part1(&test_data(super::DAY).unwrap()).unwrap(), 10092);
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            solve_part2(&test_data(super::DAY).unwrap()).unwrap(),
            0 // XXX: Update this to the expected value for part 2 sample data.
        );
    }
}
