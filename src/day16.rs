use crate::{add_xy, Direction, Position, Result};
use anyhow::Context as _;
use aoc_runner_derive::aoc;
use std::{collections::HashSet, fmt::Display};
use tracing::info;

pub const DAY: u32 = 16;

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
struct Orientation {
    position: Position,
    direction: Direction,
}

fn start_pos(maze: &Maze) -> Result<Orientation> {
    let start_pos = maze
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter().enumerate().find_map(|(x, cell)| {
                if *cell == Cell::Start {
                    Some((x, y))
                } else {
                    None
                }
            })
        })
        .ok_or_else(|| anyhow::anyhow!("No start cell found"))?;

    Ok(Orientation {
        position: start_pos,
        direction: (1, 0),
    })
}

fn solve_part1_impl(input: &Data) -> Result<(Vec<Orientation>, usize)> {
    let maze = &input.maze;
    // find the start
    let start_pos = start_pos(maze)?;

    let shortest = pathfinding::directed::dijkstra::dijkstra(
        &start_pos,
        |xy| maze_moves(xy, maze).collect::<Vec<_>>(),
        |xy| maze[xy.position.1][xy.position.0] == Cell::End,
    )
    .ok_or_else(|| anyhow::anyhow!("No path found"))?;

    Ok(shortest)
}

fn maze_moves<'a>(
    o: &'a Orientation,
    maze: &'a Maze,
) -> impl Iterator<Item = (Orientation, usize)> + 'a {
    // We can always turn
    let moves = [
        (
            Orientation {
                position: o.position,
                direction: (o.direction.1, -o.direction.0),
            },
            1000,
        ),
        (
            Orientation {
                position: o.position,
                direction: (-o.direction.1, o.direction.0),
            },
            1000,
        ),
    ];

    let make_cell = || {
        let xy = add_xy(&o.position, &o.direction)?;
        let cell = maze.get(xy.1)?.get(xy.0)?;
        if cell == &Cell::Wall {
            None
        } else {
            Some((
                Orientation {
                    position: xy,
                    direction: o.direction,
                },
                1,
            ))
        }
    };

    moves
        .into_iter()
        .chain([make_cell()].into_iter().filter_map(|x| x))
}

fn solve_part2_impl(input: &Data) -> Result<usize> {
    let maze = &input.maze;
    // find the start
    let start_pos = start_pos(maze)?;

    let mut found = None;
    for max in (100..).step_by(10) {
        println!("Trying max {}", max);
        let all = pathfinding::directed::yen::yen(
            &start_pos,
            |xy| maze_moves(xy, maze).collect::<Vec<_>>(),
            |xy| maze[xy.position.1][xy.position.0] == Cell::End,
            max,
        );
        let mut s = all.iter();
        let first = s
            .next()
            .map(|p| p.1)
            .ok_or_else(|| anyhow::anyhow!("No path found"))?;
        if s.any(|p| p.1 != first) {
            let unique = all
                .into_iter()
                .filter(|p| p.1 == first)
                .flat_map(|p| p.0.into_iter().map(|p| p.position))
                .collect::<HashSet<_>>();
            found = Some(unique);
            break;
        }
    }

    Ok(found
        .ok_or_else(|| anyhow::anyhow!("No unique path found"))?
        .len())
}

#[allow(dead_code)]
fn print_path(maze: &Maze, path: &HashSet<Position>) {
    for (y, row) in maze.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if path.contains(&(x, y)) {
                print!("O");
            } else {
                print!("{}", Into::<char>::into(cell));
            }
        }
        println!();
    }
}

/// Solution to part 1
#[aoc(day16, part1)]
fn solve_part1(input: &str) -> Result<usize> {
    let input = Data::parse(input).context("input parsing")?;
    Ok(solve_part1_impl(&input)?.1)
}

/// Solution to part 2
#[aoc(day16, part2)]
fn solve_part2(input: &str) -> Result<usize> {
    let input = Data::parse(input).context("input parsing")?;
    solve_part2_impl(&input)
}

#[derive(Debug, Clone, PartialEq)]
enum Cell {
    Wall,
    Empty,
    Start,
    End,
}
impl TryFrom<char> for Cell {
    type Error = anyhow::Error;

    fn try_from(value: char) -> std::result::Result<Self, Self::Error> {
        Ok(match value {
            '#' => Cell::Wall,
            '.' => Cell::Empty,
            'S' => Cell::Start,
            'E' => Cell::End,
            _ => anyhow::bail!("Invalid maze char {value}"),
        })
    }
}
impl Into<char> for &Cell {
    fn into(self) -> char {
        match self {
            Cell::Wall => '#',
            Cell::Empty => '.',
            Cell::Start => 'S',
            Cell::End => 'E',
        }
    }
}

fn print_maze(maze: &Maze, o: &Orientation) {
    let my_pos = &o.position;
    for (y, row) in maze.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if my_pos == &(x, y) {
                match o.direction {
                    (0, -1) => print!("^"),
                    (0, 1) => print!("v"),
                    (-1, 0) => print!("<"),
                    (1, 0) => print!(">"),
                    _ => print!("?"),
                }
            } else {
                print!("{}", Into::<char>::into(cell));
            }
        }
        println!();
    }
}

type Maze = Vec<Vec<Cell>>;

/// Problem input
#[derive(Debug)]
struct Data {
    maze: Maze,
}
impl Data {
    fn parse(s: &str) -> Result<Self> {
        let s = s.lines();
        let maze = s
            .map(|row| row.chars().map(Cell::try_from).collect::<Result<_>>())
            .collect::<Result<_>>()?;

        Ok(Data { maze })
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
        assert_eq!(solve_part1(&test_data(super::DAY).unwrap()).unwrap(), 11048);
    }

    #[test]
    fn part2_example() {
        assert_eq!(solve_part2(&test_data(super::DAY).unwrap()).unwrap(), 64);
    }
}
