use crate::Result;
use anyhow::Context as _;
use aoc_runner_derive::aoc;
use std::{collections::HashSet, fmt::Display};
use tracing::info;

pub const DAY: u32 = 14;

fn solve_part1_impl(input: &Data) -> Result<usize> {
    let robots = &input.robots;
    let board_size = (101, 103);

    // Step all robots 100 seconds
    let robots = robots.iter().map(|robot| robot.step(&board_size, 100));

    let robots = robots.collect::<Result<Vec<_>>>()?;

    let quad_size = (board_size.0 / 2, board_size.1 / 2);
    let second_quad_start = (board_size.0 - quad_size.0, board_size.1 - quad_size.1);
    let quads = [
        (0..quad_size.0, 0..quad_size.1),
        (0..quad_size.0, second_quad_start.1..board_size.1),
        (second_quad_start.0..board_size.0, 0..quad_size.1),
        (
            second_quad_start.0..board_size.0,
            second_quad_start.1..board_size.1,
        ),
    ];

    let robots_in_quads = quads.iter().map(|(xrange, yrange)| {
        robots
            .iter()
            .filter(|r| xrange.contains(&r.pos.0) && yrange.contains(&r.pos.1))
            .count()
    });

    Ok(robots_in_quads
        .inspect(|count| println!("{}", count))
        .product())
}

fn print_board<'a>(robots: impl Iterator<Item = &'a Robot> + Clone, board_size: &Point) {
    for y in 0..board_size.1 {
        for x in 0..board_size.0 {
            let robot_count = robots.clone().filter(|r| r.pos == (x, y)).count();
            if robot_count > 0 {
                print!("{}", robot_count);
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn solve_part2_impl(input: &Data) -> Result<usize> {
    let robots = &input.robots;
    let board_size = (101, 103);

    let mut robots = robots.clone();
    for s in 1..50000 {
        robots = robots
            .iter()
            .map(|r| r.step(&board_size, 1))
            .collect::<Result<Vec<_>>>()?;
        // Look for more than  robots in a row
        let mut found = false;
        for y in 0..board_size.1 {
            let x_locations = robots
                .iter()
                .filter(|r| r.pos.1 == y)
                .map(|r| r.pos.0)
                .collect::<HashSet<_>>();

            for x in 0..board_size.0 {
                if (0..10).all(|i| x_locations.contains(&(x + i))) {
                    found = true;
                }
            }
        }
        if found {
            println!("Step {}", s);
            print_board(robots.iter(), &board_size);
        }
    }
    Ok(1)
}

/// Solution to part 1
#[aoc(day14, part1)]
fn solve_part1(input: &str) -> Result<usize> {
    let input = Data::parse(input).context("input parsing")?;
    solve_part1_impl(&input)
}

/// Solution to part 2
#[aoc(day14, part2)]
fn solve_part2(input: &str) -> Result<usize> {
    let input = Data::parse(input).context("input parsing")?;
    solve_part2_impl(&input)
}

type Point = (usize, usize);
type Direction = (isize, isize);

#[derive(Clone, Debug)]
struct Robot {
    pos: Point,
    vel: Direction,
}
impl Robot {
    fn step(&self, board_size: &Point, t: usize) -> Result<Robot> {
        let pos = (
            move_value(self.pos.0, self.vel.0, board_size.0, t).context("x")?,
            move_value(self.pos.1, self.vel.1, board_size.1, t).context("y")?,
        );
        Ok(Robot { pos, vel: self.vel })
    }
}

pub fn move_value(value: usize, direction: isize, limit: usize, steps: usize) -> Result<usize> {
    // Move value will take value, add direction * steps, and wrap it around limit
    // It can go negative, so the wrapping needs to be done properly
    let value = value as isize;
    let direction = direction as isize;
    let limit = limit as isize;
    let steps = steps as isize;

    Ok(((value + direction * steps) % limit + limit) as usize % limit as usize)
}

/// Problem input
#[derive(Debug)]
struct Data {
    // XXX: Change this to the actual data structure
    robots: Vec<Robot>,
}
impl Data {
    fn parse(s: &str) -> Result<Self> {
        // line looks like
        // p=7,6 v=-1,-3
        let linere = regex::Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();

        let s = s.lines();
        let robots = s
            .map(|line| {
                let caps = linere
                    .captures(line)
                    .ok_or_else(|| anyhow::anyhow!("bad line"))?;
                Ok(Robot {
                    pos: (caps[1].parse()?, caps[2].parse()?),
                    vel: (caps[3].parse()?, caps[4].parse()?),
                })
            })
            .collect::<Result<_>>();

        Ok(Data { robots: robots? })
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
            21 // XXX: Update this to the expected value for part 1 sample data.
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(solve_part2(&test_data(super::DAY).unwrap()).unwrap(), 0);
    }

    #[test]
    fn test_wrap() {
        assert_eq!(move_value(1, -3, 7, 1).unwrap(), 5);
    }
}
