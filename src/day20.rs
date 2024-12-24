use crate::add_xy;
use crate::GetCell;
use crate::Position;
use crate::{parse_grid, Result};
use anyhow::Context as _;
use aoc_runner_derive::aoc;
use itertools::Itertools;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator as _;
use std::collections::HashMap;
use std::fmt::Display;

pub const DAY: u32 = 20;

fn solve_part1_impl(input: &Data) -> Result<usize> {
    let map = input.map.as_slice();

    let start = map
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter().enumerate().filter_map(move |(x, &cell)| {
                if cell == Cell::Start {
                    Some((x, y))
                } else {
                    None
                }
            })
        })
        .next()
        .context("no start cell found")?;

    let shortest_path = pathfinding::directed::dijkstra::dijkstra(
        &XYMeta::new(start, 0),
        |p| successors(p, map, None).map(|xy| (xy, 1)),
        |p| map.get_cell(&p.pos).unwrap() == &Cell::End,
    )
    .ok_or_else(|| anyhow::anyhow!("no path found"))?;

    let shortest_path_len = shortest_path.0.len() - 1;
    println!("Shortest path length: {}", shortest_path_len);

    let shortest_path_with_cheat = pathfinding::directed::dijkstra::dijkstra(
        &XYMeta::new(start, 1),
        |p| successors(p, map, None).map(|xy| (xy, 1)),
        |p| map.get_cell(&p.pos).unwrap() == &Cell::End,
    )
    .ok_or_else(|| anyhow::anyhow!("no path found"))?;

    println!(
        "Shortest path length with cheats: {}",
        shortest_path_with_cheat.0.len() - 1
    );

    let wall_xy_positions = map
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter().enumerate().filter_map(move |(x, &cell)| {
                if cell == Cell::Wall {
                    Some((x, y))
                } else {
                    None
                }
            })
        })
        .collect::<Vec<_>>();

    let max_path_len = shortest_path_len - if cfg!(test) { 2 } else { 100 };

    let count: usize = wall_xy_positions
        .into_par_iter()
        .filter_map(|(x, y)| {
            let mut map = input.map.clone();
            let map = map.as_mut_slice();
            map[y][x] = Cell::Space;
            println!("Checking wall at ({}, {})", x, y);

            let shortest_path = pathfinding::directed::dijkstra::dijkstra(
                &start,
                |p| successors_simple3(p, map).map(|xy| (xy, 1)),
                |p| map.get_cell(p).unwrap() == &Cell::End,
            )
            .ok_or_else(|| anyhow::anyhow!("no path found"))
            .unwrap();
            if shortest_path.0.len() - 1 > max_path_len {
                return None;
            }

            let path_count = pathfinding::directed::count_paths::count_paths(
                (start, vec![]),
                |p| successors_simple(p, map, max_path_len),
                |(p, visited)| {
                    map.get_cell(&p).unwrap() == &Cell::End && visited.len() <= max_path_len
                },
            );
            println!("{},{} Shortest path count: {}", x, y, path_count);
            Some(path_count)
        })
        .sum();

    println!("Shortest path count with cheats: {}", count);

    // let count = pathfinding::directed::count_paths::count_paths(
    //     XYMeta {
    //         pos: start,
    //         cheat_count: 1,
    //     },
    //     |p| successors(p, map),
    //     |p| map.get_cell(p.pos).unwrap() == &Cell::End,
    // );

    Ok(count)
}

#[derive(Debug, Eq, Clone)]
struct XYMeta {
    cheat_active: bool,
    pos: Position,
    cheat_count: usize,
    depth: usize,
    cheat_path: Vec<Position>,
}
impl XYMeta {
    fn new(pos: Position, cheat_count: usize) -> Self {
        Self {
            pos,
            cheat_count,
            cheat_active: false,
            depth: 0,
            cheat_path: vec![],
        }
    }
}
impl AsRef<Position> for XYMeta {
    fn as_ref(&self) -> &Position {
        &self.pos
    }
}
impl PartialEq for XYMeta {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos && self.cheat_path == other.cheat_path
        // && self.cheat_active == other.cheat_active
        // && self.cheat_count == other.cheat_count
    }
}
impl std::hash::Hash for XYMeta {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.pos.hash(state);
        self.cheat_path.hash(state);
        // self.cheat_active.hash(state);
        // self.cheat_count.hash(state);
    }
}

fn successors_simple3(p: &Position, map: MapRef) -> impl Iterator<Item = Position> {
    const DIRECTIONS: [(isize, isize); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

    let next_pos = DIRECTIONS.iter().filter_map(|dir| add_xy(p, dir));

    let mut next_spaces = next_pos.filter_map(|xy| {
        let cell = map.get_cell(&xy)?;
        if cell == &Cell::Wall {
            None
        } else {
            Some(xy)
        }
    });

    let dirs = [
        next_spaces.next(),
        next_spaces.next(),
        next_spaces.next(),
        next_spaces.next(),
    ];
    dirs.into_iter().flatten()
}

fn successors_simple(
    p: &(Position, Vec<Position>),
    map: MapRef,
    max_count: usize,
) -> impl Iterator<Item = (Position, Vec<Position>)> {
    let mut visited = p.1.clone();
    visited.push(p.0.clone());

    let next_pos = successors_simple3(&p.0, map).filter(|xy| !visited.contains(xy));

    let next_spaces = next_pos.map(|xy| (xy, visited.clone()));

    let over_count = visited.len() > max_count;
    let mut next_spaces = next_spaces.filter(|_| !over_count);
    //let mut next_spaces = next_spaces;

    let dirs = [
        next_spaces.next(),
        next_spaces.next(),
        next_spaces.next(),
        next_spaces.next(),
    ];
    dirs.into_iter().flatten()
}

fn successors(
    in_pos: &XYMeta,
    map: MapRef,
    max_path_len: Option<usize>,
) -> impl Iterator<Item = XYMeta> {
    const DIRECTIONS: [(isize, isize); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

    let next_pos = DIRECTIONS.iter().filter_map(|dir| add_xy(&in_pos.pos, dir));

    let have_available_cheats = in_pos.cheat_count > 0;

    let next_spaces = next_pos.filter_map(move |xy| {
        let cell = map.get_cell(&xy)?;
        match (cell, have_available_cheats) {
            (Cell::Wall, true) => Some((xy, true)),
            (Cell::Wall, false) => None,
            (_, _) => Some((xy, false)),
        }
    });
    let meta = next_spaces.map(|(p, is_cheat)| XYMeta {
        pos: p,
        depth: in_pos.depth + 1,
        cheat_count: match (is_cheat, in_pos.cheat_active) {
            (true, _) => in_pos.cheat_count - 1,
            (false, true) => 0,
            (false, false) => in_pos.cheat_count,
        },
        cheat_active: is_cheat,
        cheat_path: if is_cheat {
            let mut cp = in_pos.cheat_path.clone();
            cp.push(p.clone());
            cp
        } else {
            in_pos.cheat_path.clone()
        },
    });

    let over_max = max_path_len.map(|max| in_pos.depth > max).unwrap_or(false);
    let mut meta = meta.filter(|_| !over_max);

    let dirs = [meta.next(), meta.next(), meta.next(), meta.next()];
    dirs.into_iter().flatten()
}

#[allow(dead_code)]
fn print_map(map: MapRef, path: &[impl AsRef<Position>]) {
    for (y, row) in map.into_iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if path.iter().any(|p| p.as_ref() == &(x, y)) {
                print!("o");
            } else {
                print!(
                    "{}",
                    match cell {
                        Cell::Wall => '#',
                        Cell::Space => '.',
                        Cell::Start => 'S',
                        Cell::End => 'E',
                    }
                );
            }
        }
        println!();
    }
}

fn solve_part2_impl(input: &Data) -> Result<usize> {
    let map = input.map.as_slice();

    let start = map
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter().enumerate().filter_map(move |(x, &cell)| {
                if cell == Cell::Start {
                    Some((x, y))
                } else {
                    None
                }
            })
        })
        .next()
        .context("no start cell found")?;

    println!("Start: {:?}", start);

    let shortest_path = pathfinding::directed::dijkstra::dijkstra(
        &XYMeta::new(start, 0),
        |p| successors(p, map, None).map(|xy| (xy, 1)),
        |p| map.get_cell(&p.pos).unwrap() == &Cell::End,
    )
    .ok_or_else(|| anyhow::anyhow!("no path found"))?;

    let shortest_path_len = shortest_path.0.len();
    println!("Shortest path length: {}", shortest_path.0.len() - 1);

    let shortest_path = shortest_path
        .0
        .into_iter()
        .map(|p| p.pos)
        .collect::<Vec<_>>();

    const CHEAT_DISTANCE: usize = 20;

    let nodes_within_cheat_distance = |xy: Position| {
        shortest_path
            .iter()
            .enumerate()
            // Make the enumeration index the distance to the end
            .map(|(i, p)| (shortest_path_len - i - 1, p))
            .filter_map(move |(dist_to_end, p)| {
                let xydist = xy_distance(p, &xy);
                if xydist <= CHEAT_DISTANCE {
                    Some((dist_to_end, p, xydist))
                } else {
                    None
                }
            })
    };

    const SAVE_DISTANCE: usize = if cfg!(test) { 50 } else { 100 };

    let mut save_count = HashMap::<usize, usize>::new();

    for (cur_dist, n) in shortest_path.iter().enumerate() {
        for (cheat_dist_to_end, _cheat_node, cheat_dist) in nodes_within_cheat_distance(*n) {
            let total_dist = cur_dist + cheat_dist_to_end + cheat_dist;
            // println!(
            //     "From: {:?} to: {:?} total_dist: {}",
            //     n, _cheat_node, total_dist
            // );
            if total_dist > shortest_path_len - 1 {
                continue;
            }
            let save_dist = shortest_path_len - 1 - total_dist;
            if save_dist >= SAVE_DISTANCE {
                *save_count.entry(save_dist).or_insert(0) += 1;
            }
        }
    }
    let sorted = save_count
        .iter()
        .sorted_by_key(|(k, _v)| *k)
        .collect::<Vec<_>>();
    println!("Save count: {:?}", sorted);

    let total = save_count.values().sum::<usize>();

    Ok(total)
}

fn xy_distance(a: &Position, b: &Position) -> usize {
    let xdist = a.0.abs_diff(b.0);
    let ydist = a.1.abs_diff(b.1);
    xdist + ydist
}

/// Solution to part 1
#[aoc(day20, part1)]
fn solve_part1(input: &str) -> Result<usize> {
    let input = Data::parse(input).context("input parsing")?;
    solve_part1_impl(&input)
}

/// Solution to part 2
#[aoc(day20, part2)]
fn solve_part2(input: &str) -> Result<usize> {
    let input = Data::parse(input).context("input parsing")?;
    solve_part2_impl(&input)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Wall,
    Space,
    Start,
    End,
}
impl TryFrom<char> for Cell {
    type Error = anyhow::Error;

    fn try_from(value: char) -> std::result::Result<Self, Self::Error> {
        match value {
            '#' => Ok(Cell::Wall),
            '.' => Ok(Cell::Space),
            'S' => Ok(Cell::Start),
            'E' => Ok(Cell::End),
            _ => Err(anyhow::anyhow!("Invalid cell: {}", value)),
        }
    }
}

type Map = Vec<Vec<Cell>>;
type MapRef<'a> = &'a [Vec<Cell>];

/// Problem input
#[derive(Debug)]
struct Data {
    // XXX: Change this to the actual data structure
    map: Map,
}
impl Data {
    fn parse(s: &str) -> Result<Self> {
        let map = parse_grid(s)?;
        Ok(Data { map })
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
        assert_eq!(solve_part1(&test_data(super::DAY).unwrap()).unwrap(), 44);
    }

    #[test]
    fn part2_example() {
        assert_eq!(solve_part2(&test_data(super::DAY).unwrap()).unwrap(), 285);
    }
}
