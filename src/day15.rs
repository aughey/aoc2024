use crate::{add_xy, add_xy_result, Direction, GetCell, GetCellMut, Position, Result};
use anyhow::Context as _;
use aoc_runner_derive::aoc;
use std::fmt::Display;
use tracing::debug;

pub const DAY: u32 = 15;

type Map = Vec<Vec<Cell>>;
type MapRef<'a> = &'a [Vec<Cell>];
type MapMutRef<'a> = &'a mut [Vec<Cell>];

/// Find the player's position in the map
fn playerxy(map: MapRef) -> Option<Position> {
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

/// Solution to part 1
fn solve_part1_impl(input: &Data) -> Result<usize> {
    let mut map = input.map.clone();
    let player_xy = playerxy(&map).ok_or_else(|| anyhow::anyhow!("player not found"))?;

    let mut player_xy = player_xy;

    for m in input.movements.iter() {
        //print_map(&map);
        let direction = m.direction();
        if can_move_in_direction(&map.as_slice(), &player_xy, &direction)? {
            move_cell(&mut map.as_mut_slice(), &player_xy, &direction)?;
            player_xy = add_xy_result(&player_xy, &direction)?;
        }
    }

    print_map(&map);
    // evaluate the map
    let boxes = map.iter().enumerate().flat_map(|(y, row)| {
        row.iter().enumerate().filter_map(move |(x, cell)| {
            matches!(cell, Cell::Box | Cell::BoxLeft).then_some((x, y))
        })
    });
    let gps = boxes.map(|(x, y)| x + 100 * y).sum();

    Ok(gps)
}

/// Print the map
fn print_map(map: MapRef) {
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

/// Can the cell at xy move in the direction given?
fn can_move_in_direction(
    map: &impl GetCell<Cell>,
    xy: &Position,
    direction: &Direction,
) -> Result<bool> {
    match map.get_cell_result(xy)? {
        Cell::Wall => Ok(false),
        Cell::Empty => Ok(true),
        cell => {
            for newxy in cell.all_next_xy_in_direction(xy, direction) {
                if !can_move_in_direction(map, &newxy, direction)? {
                    return Ok(false);
                }
            }
            Ok(true)
        }
    }
}

/// Move a cell in the given position in the given direction.
///
/// This will fail in the middle of the move if it encounters a cell that is not empty.
/// This Error condition will result in the map being in an inconsistent state.
fn move_cell<M>(map: &mut M, from: &Position, direction: &Direction) -> Result<()>
where
    M: GetCellMut<Cell> + GetCell<Cell>,
{
    //    assert!(can_move_in_direction(map, from, direction)?);

    let my_cell = match map.get_cell_mut_result(from)? {
        Cell::Empty => return Ok(()),
        Cell::Wall => anyhow::bail!("cannot move a wall cell"),
        c => c.clone(),
    };

    // First move all next coordinates out of the way
    for next_xy in my_cell.all_next_xy_in_direction(from, direction) {
        move_cell(map, &next_xy, direction)?;
    }

    // Now move each coordinate of my cell in the direction given
    for my_xy in my_cell.all_coords(from, direction) {
        let my_cell = map.get_cell_result(&my_xy)?.clone();

        let next_xy = add_xy_result(&my_xy, direction)?;

        let next_cell = map.get_cell_mut_result(&next_xy)?;
        if !matches!(*next_cell, Cell::Empty) {
            anyhow::bail!("cannot move to a non-empty cell");
        }
        *next_cell = my_cell.clone();

        let this_cell = map.get_cell_mut_result(&my_xy)?;
        *this_cell = Cell::Empty;
    }
    Ok(())
}

fn solve_part2_impl(input: &Data) -> Result<usize> {
    // Expand the map
    let map = input
        .map
        .iter()
        .map(|row| {
            row.iter()
                .flat_map(|cell| {
                    match cell {
                        Cell::Wall => &[Cell::Wall, Cell::Wall],
                        Cell::Box => &[Cell::BoxLeft, Cell::BoxRight],
                        Cell::Empty => &[Cell::Empty, Cell::Empty],
                        Cell::Player => &[Cell::Player, Cell::Empty],
                        Cell::BoxLeft | Cell::BoxRight => {
                            panic!("invalid cell to expand in part 2")
                        }
                    }
                    .into_iter()
                    .cloned()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    solve_part1_impl(&Data {
        map,
        movements: input.movements.clone(),
    })
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
impl Cell {
    /// Coordinates that this occupies where the current cell is at (0,0)
    fn unit_coords(&self, direction: &Direction) -> impl Iterator<Item = &Direction> {
        let left_right = direction.1 == 0;
        match (self, left_right) {
            // if we're moving up and down, consider both the left and right.
            // But if we're going left and right, they move as individual units
            (Cell::BoxLeft, false) => [(0, 0), (1, 0)].as_slice(),
            (Cell::BoxRight, false) => [(0, 0), (-1, 0)].as_slice(),
            _ => [(0, 0)].as_slice(),
        }
        .iter()
    }

    /// All coordinates that this occupies
    fn all_coords<'a>(
        &'a self,
        cur_xy: &'a Position,
        direction: &'a Direction,
    ) -> impl Iterator<Item = Position> + 'a {
        self.unit_coords(direction)
            .filter_map(move |d| add_xy(cur_xy, d))
    }

    /// All coordinates that this cell should occupy when moved in this direction.
    fn all_next_xy_in_direction<'a>(
        &'a self,
        cur_xy: &'a Position,
        direction: &'a Direction,
    ) -> impl Iterator<Item = Position> + 'a {
        self.all_coords(cur_xy, direction)
            .filter_map(move |p| add_xy(&p, direction))
    }
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

#[derive(Debug, Clone)]
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
impl Movement {
    fn direction(&self) -> Direction {
        match self {
            Movement::Up => (0, -1),
            Movement::Down => (0, 1),
            Movement::Left => (-1, 0),
            Movement::Right => (1, 0),
        }
    }
}

/// Problem input
#[derive(Debug)]
struct Data {
    map: Map,
    movements: Vec<Movement>,
}

fn parse_grid<T, E>(input: &str) -> Result<Vec<Vec<T>>, E>
where
    T: TryFrom<char, Error = E>,
{
    input
        .lines()
        .map(|line| line.chars().map(T::try_from).collect::<Result<Vec<_>, _>>())
        .collect::<Result<Vec<_>, _>>()
}

impl Data {
    fn parse(s: &str) -> Result<Self> {
        // split s into two things separated by a blank line
        let (mapcontent, movementscontent) = s
            .split_once("\n\n")
            .ok_or_else(|| anyhow::anyhow!("missing blank line"))?;

        let map = parse_grid(mapcontent)?;
        let movements = parse_grid(movementscontent)?;

        // // parse map
        // let map = mapcontent
        //     .lines()
        //     .map(|line| line.chars().map(Cell::try_from).collect::<Result<Vec<_>>>())
        //     .collect::<Result<Vec<_>>>()?;

        // // parse movements
        // let movements = movementscontent
        //     .lines()
        //     .flat_map(|line| line.chars().map(Movement::try_from))
        //     .collect::<Result<Vec<_>>>()?;

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
            9021 // XXX: Update this to the expected value for part 2 sample data.
        );
    }
}
