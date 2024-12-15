use crate::{Direction, Position, Result};
use anyhow::Context as _;
use aoc_runner_derive::aoc;
use std::fmt::Display;
use tracing::debug;

pub const DAY: u32 = 15;

type Map = Vec<Vec<Cell>>;

trait Get {
    fn get_cell(&self, xy: &Position) -> Option<&Cell>;
    fn get_cell_mut(&mut self, xy: &Position) -> Option<&mut Cell>;
    fn get_cell_result(&self, xy: &Position) -> Result<&Cell> {
        self.get_cell(xy)
            .ok_or_else(|| anyhow::anyhow!("no cell at {:?}", xy))
    }
    fn get_cell_mut_result(&mut self, xy: &Position) -> Result<&mut Cell> {
        self.get_cell_mut(xy)
            .ok_or_else(|| anyhow::anyhow!("no cell at {:?}", xy))
    }
}
impl Get for Map {
    fn get_cell(&self, xy: &Position) -> Option<&Cell> {
        self.get(xy.1)?.get(xy.0)
    }
    fn get_cell_mut(&mut self, xy: &Position) -> Option<&mut Cell> {
        self.get_mut(xy.1)?.get_mut(xy.0)
    }
}

fn playerxy(map: &Map) -> Option<Position> {
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
        //print_map(&map);
        let direction = m.direction();
        if can_move_in_direction(&map, &player_xy, &direction)? {
            move_cell(&mut map, &player_xy, &direction)?;
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

fn print_map(map: &Map) {
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

fn add_xy_result(cur_cell: &Position, direction: &Direction) -> Result<Position> {
    Ok((
        cur_cell
            .0
            .checked_add_signed(direction.0)
            .ok_or_else(|| anyhow::anyhow!("invalid movement"))?,
        cur_cell
            .1
            .checked_add_signed(direction.1)
            .ok_or_else(|| anyhow::anyhow!("invalid movement"))?,
    ))
}

fn can_move_in_direction(map: &Map, cur_xy: &Position, direction: &Direction) -> Result<bool> {
    debug!(
        "Checking if {:?} can move from {:?} in direction {:?}",
        map.get_cell_result(cur_xy)?,
        cur_xy,
        direction
    );
    match map
        .get_cell(cur_xy)
        .ok_or_else(|| anyhow::anyhow!("Cannot get current cell"))?
    {
        Cell::Wall => Ok(false),
        Cell::Empty => Ok(true),
        cell => {
            for newxy in cell.all_next_xy_in_direction(cur_xy, direction) {
                if !can_move_in_direction(map, &newxy, direction)? {
                    return Ok(false);
                }
            }
            Ok(true)
        }
    }
}

fn move_cell(map: &mut Map, from: &Position, direction: &Direction) -> Result<()> {
    //    assert!(can_move_in_direction(map, from, direction)?);

    let my_cell = match map.get_cell_result(from)? {
        Cell::Empty => return Ok(()),
        Cell::Wall => anyhow::bail!("cannot move a wall cell"),
        c => c.clone(),
    };

    // First move all next coordinates out of the way
    for next_xy in my_cell.all_next_xy_in_direction(from, direction) {
        move_cell(map, &next_xy, direction)?;
    }

    debug!(
        "Moving {my_cell:?} from {:?} in direction {:?}",
        from, direction
    );

    // Now move mine in place
    for my_xy in my_cell.all_coords_from(from, direction) {
        let my_cell = map.get_cell_result(&my_xy)?.clone();

        let next_xy = add_xy_result(&my_xy, direction)?;

        let next_cell = map.get_cell_mut_result(&next_xy)?;
        assert!(matches!(*next_cell, Cell::Empty));
        *next_cell = my_cell.clone();

        let this_cell = map.get_cell_mut_result(&my_xy)?;
        *this_cell = Cell::Empty;
    }
    Ok(())
}

fn add_xy(xy: &Position, direction: &Direction) -> Option<Position> {
    Some((
        xy.0.checked_add_signed(direction.0)?,
        xy.1.checked_add_signed(direction.1)?,
    ))
}

fn solve_part2_impl(input: &Data) -> Result<usize> {
    // Rewrite the map
    let map = input
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
            (Cell::BoxLeft, false) => [(0, 0), (1, 0)].as_slice(),
            (Cell::BoxRight, false) => [(0, 0), (-1, 0)].as_slice(),
            _ => [(0, 0)].as_slice(),
        }
        .iter()
    }
    fn all_coords_from<'a>(
        &'a self,
        cur_xy: &'a Position,
        direction: &'a Direction,
    ) -> impl Iterator<Item = Position> + 'a {
        self.unit_coords(direction)
            .filter_map(move |d| add_xy(cur_xy, d))
    }
    fn all_next_xy_in_direction<'a>(
        &'a self,
        cur_xy: &'a Position,
        direction: &'a Direction,
    ) -> impl Iterator<Item = Position> + 'a {
        self.all_coords_from(cur_xy, direction)
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
