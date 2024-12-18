use crate::{add_xy, Direction, Position, Result};
use anyhow::Context as _;
use aoc_runner_derive::aoc;
use std::{collections::HashSet, fmt::Display};

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
        |xy| maze_moves(xy, maze),
        |xy| maze[xy.position.1][xy.position.0] == Cell::End,
    )
    .ok_or_else(|| anyhow::anyhow!("No path found"))?;

    Ok(shortest)
}

// maze_moves returns a generated Iterator rather than a static
// vec so that we're not allocating.
fn maze_moves<'a>(
    o: &'a Orientation,
    maze: &'a Maze,
) -> impl Iterator<Item = (Orientation, usize)> {
    // We can always turn
    let turns = [
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

    let make_step_forward = || {
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
    let step_forward = make_step_forward();

    turns.into_iter().chain(step_forward.into_iter())
}

fn solve_part2_impl(input: &Data) -> Result<usize> {
    let maze = &input.maze;
    // find the start
    let start_pos = start_pos(maze)?;

    // use astar this time
    let astar = pathfinding::directed::astar::astar_bag(
        &start_pos,
        |xy| maze_moves(xy, maze),
        |_| 0,
        |xy| maze[xy.position.1][xy.position.0] == Cell::End,
    )
    .ok_or_else(|| anyhow::anyhow!("Could not construct astar solver"))?;

    let astar = astar.0;
    let found = astar
        .into_iter()
        .flatten()
        .map(|o| o.position)
        .collect::<HashSet<_>>();

    Ok(found.len())
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
impl From<&Cell> for char {
    fn from(val: &Cell) -> Self {
        match val {
            Cell::Wall => '#',
            Cell::Empty => '.',
            Cell::Start => 'S',
            Cell::End => 'E',
        }
    }
}

#[allow(dead_code)]
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
