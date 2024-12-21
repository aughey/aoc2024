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
fn maze_moves(o: &Orientation, maze: &Maze) -> impl Iterator<Item = (Orientation, usize)> {
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
    ]
    .into_iter();

    let _step_forward = (|| {
        let xy = add_xy(&o.position, &o.direction)?;
        let cell = maze.get(xy.1)?.get(xy.0)?;
        (cell != &Cell::Wall).then_some((
            Orientation {
                position: xy,
                direction: o.direction,
            },
            1,
        ))
    })();

    let _step_forward = (|| {
        let next_xy = add_xy(&o.position, &o.direction)?;
        let forward_cell = maze.get(next_xy.1)?.get(next_xy.0);
        let non_wall = forward_cell.filter(|cell| cell != &&Cell::Wall);
        non_wall.map(|_| {
            (
                Orientation {
                    position: add_xy(&o.position, &o.direction).unwrap(),
                    direction: o.direction,
                },
                1,
            )
        })
    })();

    let _step_forward = (|| {
        let next_xy = add_xy(&o.position, &o.direction)?;
        let forward_cell = maze.get(next_xy.1)?.get(next_xy.0)?;
        let non_wall = forward_cell != &Cell::Wall;
        non_wall.then_some((
            Orientation {
                position: add_xy(&o.position, &o.direction).unwrap(),
                direction: o.direction,
            },
            1,
        ))
    })();

    let _step_forward = add_xy(&o.position, &o.direction).and_then(|next_xy| {
        let forward_cell = maze.get(next_xy.1)?.get(next_xy.0)?;
        let non_wall = forward_cell != &Cell::Wall;
        non_wall.then_some((
            Orientation {
                position: add_xy(&o.position, &o.direction).unwrap(),
                direction: o.direction,
            },
            1,
        ))
    });

    // Compute xy, get the maze cell, check if it's not a wall, and return the new orientation
    let _step_forward = add_xy(&o.position, &o.direction)
        .and_then(|xy| maze.get(xy.1)?.get(xy.0))
        .filter(|cell| cell != &&Cell::Wall)
        .map(|_| {
            (
                Orientation {
                    position: add_xy(&o.position, &o.direction).unwrap(),
                    direction: o.direction,
                },
                1,
            )
        });

    let _step_forward = add_xy(&o.position, &o.direction)
        .and_then(|forward_xy| maze.get(forward_xy.1)?.get(forward_xy.0))
        .map(|forward_cell| forward_cell != &Cell::Wall)
        .filter(|not_wall| *not_wall)
        .map(|_| {
            (
                Orientation {
                    position: add_xy(&o.position, &o.direction).unwrap(),
                    direction: o.direction,
                },
                1,
            )
        });

    let step_forward = if let Some(xy) = add_xy(&o.position, &o.direction) {
        if let Some(row) = maze.get(xy.1) {
            if let Some(cell) = row.get(xy.0) {
                if cell != &Cell::Wall {
                    Some((
                        Orientation {
                            position: xy,
                            direction: o.direction,
                        },
                        1,
                    ))
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    };

    turns.chain(step_forward)
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

// Playing around with implementing my own search algorithm.
#[cfg(test)]
#[allow(dead_code)]
mod mydi {
    use std::collections::{BTreeSet, HashMap, HashSet};

    use crate::{
        add_xy,
        day16::{start_pos, Cell},
        parse_grid, test_data, Position,
    };

    use super::Maze;
    struct Visit<C, N>
    where
        C: pathfinding::num_traits::Zero + Ord + Copy,
        N: Eq + std::hash::Hash + Clone,
    {
        total_cost: C,
        node: N,
    }
    impl<C, N> PartialEq for Visit<C, N>
    where
        C: pathfinding::num_traits::Zero + Ord + Copy,
        N: Eq + std::hash::Hash + Clone,
    {
        fn eq(&self, other: &Self) -> bool {
            self.total_cost == other.total_cost
        }
    }
    impl<C, N> PartialOrd for Visit<C, N>
    where
        C: pathfinding::num_traits::Zero + Ord + Copy,
        N: Eq + std::hash::Hash + Clone,
    {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.total_cost.cmp(&other.total_cost))
        }
    }
    impl<C, N> Eq for Visit<C, N>
    where
        C: pathfinding::num_traits::Zero + Ord + Copy,
        N: Eq + std::hash::Hash + Clone,
    {
    }
    impl<C, N> Ord for Visit<C, N>
    where
        C: pathfinding::num_traits::Zero + Ord + Copy,
        N: Eq + std::hash::Hash + Clone,
    {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.total_cost.cmp(&other.total_cost)
        }
    }

    struct Marking<N, C>
    where
        N: Eq + std::hash::Hash + Clone,
        C: pathfinding::num_traits::Zero + Ord + Copy,
    {
        cost: C,
        prev_node: N,
    }

    fn mydijkstra<N, C, FN, IN, FS>(
        start: &N,
        mut successors: FN,
        mut success: FS,
    ) -> Option<(Vec<N>, C)>
    where
        N: Eq + std::hash::Hash + Clone,
        C: pathfinding::num_traits::Zero + Ord + Copy,
        FN: FnMut(&N) -> IN,
        IN: IntoIterator<Item = (N, C)>,
        FS: FnMut(&N) -> bool,
    {
        let mut visit_queue = Vec::new();
        visit_queue.push(start.clone());

        let mut markings: HashMap<N, Marking<N, C>> = HashMap::new();
        markings.insert(
            start.clone(),
            Marking {
                cost: C::zero(),
                prev_node: start.clone(),
            },
        );

        let mut end_node = None;
        let mut visited = HashSet::new();

        while let Some(to_visit) = visit_queue.pop() {
            if visited.contains(&to_visit) {
                continue;
            }
            visited.insert(to_visit.clone());

            let mut child_nodes = BTreeSet::new();
            let my_cost = markings
                .get(&to_visit)
                .expect("Must have already seen")
                .cost
                .clone();

            let next_nodes = successors(&to_visit).into_iter();
            for (n, cost) in next_nodes {
                let this_cost = my_cost + cost;

                let already_visited = if let Some(marking) = markings.get_mut(&n) {
                    if success(&n) {
                        end_node = Some(n.clone());
                    }
                    if this_cost < marking.cost {
                        marking.cost = this_cost;
                        marking.prev_node = to_visit.clone();
                    }
                    true
                } else {
                    markings.insert(
                        n.clone(),
                        Marking {
                            cost: this_cost,
                            prev_node: to_visit.clone(),
                        },
                    );
                    false
                };
                if !already_visited {
                    child_nodes.insert(Visit {
                        total_cost: this_cost,
                        node: n.clone(),
                    });
                }
            }
            visit_queue.extend(child_nodes.into_iter().map(|n| n.node));
        }

        // Go backwards to find the path
        let mut path = Vec::new();
        let mut cur_node = end_node.as_ref()?;
        let mut total_cost = C::zero();
        while cur_node != start {
            path.push(cur_node.clone());
            let cur = markings.get(cur_node)?;
            total_cost = total_cost + cur.cost;
            cur_node = &cur.prev_node;
        }
        Some((path, total_cost))
    }

    #[test]
    fn test_btree_ordering() {
        let mut btree = BTreeSet::new();
        btree.insert(Visit {
            total_cost: 1,
            node: 1,
        });
        btree.insert(Visit {
            total_cost: 3,
            node: 3,
        });
        btree.insert(Visit {
            total_cost: 2,
            node: 2,
        });
        let nodes = btree.into_iter().map(|v| v.node).collect::<Vec<_>>();
        assert_eq!(nodes, vec![1, 2, 3]);
    }
    #[test]
    fn test_my_di() -> anyhow::Result<()> {
        let maze: Vec<Vec<Cell>> = parse_grid(&test_data(super::DAY)?)?;

        let maze = &maze;

        let start_pos = start_pos(maze)?.position;

        let directions = &[(-1, 0), (0, -1), (1, 0), (0, 1)];

        let shortest = pathfinding::directed::dijkstra::dijkstra(
            &start_pos,
            |xy| {
                directions
                    .iter()
                    .filter_map(|dir| {
                        let xy = add_xy(&xy, dir)?;
                        let cell = maze.get(xy.1)?.get(xy.0)?;
                        (cell != &Cell::Wall).then_some((xy, 1))
                    })
                    .collect::<Vec<_>>()
                    .into_iter()
            },
            |xy| maze[xy.1][xy.0] == Cell::End,
        )
        .ok_or_else(|| anyhow::anyhow!("No path found"))?;

        let shortest_length = shortest.0.len();
        //      print_maze2(maze, shortest.0.as_slice());
        assert_eq!(shortest_length, 41);

        let _shortest = mydijkstra(
            &start_pos,
            |xy| {
                directions
                    .iter()
                    .filter_map(|dir| {
                        let xy = add_xy(&xy, dir)?;
                        let cell = maze.get(xy.1)?.get(xy.0)?;
                        (cell != &Cell::Wall).then_some((xy, 1))
                    })
                    .collect::<Vec<_>>()
                    .into_iter()
            },
            |xy| maze[xy.1][xy.0] == Cell::End,
        )
        .ok_or_else(|| anyhow::anyhow!("No path found"))?;
        //       print_maze2(maze, &shortest.0);
        //        assert_eq!(shortest.0.len(), 41);

        Ok(())
    }

    #[allow(dead_code)]
    fn print_maze2(maze: &Maze, path_points: &[Position]) {
        for (y, row) in maze.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                let c = if path_points.contains(&(x, y)) {
                    'O'
                } else {
                    Into::<char>::into(cell)
                };
                print!("{}", c);
            }
            println!();
        }
    }
}
