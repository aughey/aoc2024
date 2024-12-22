use crate::Result;
use anyhow::Context as _;
use aoc_runner_derive::aoc;
use glam::I8Vec2;
use std::{collections::HashMap, fmt::Display};

pub const DAY: u32 = 21;

type Keypad = HashMap<char, I8Vec2>;

// fn compute_path(
//     robot: &mut KeypadProgress,
//     cs: impl Iterator<Item = char> + Clone,
// ) -> Result<Vec<I8Vec2>> {
//     let mut ret = Vec::new();
//     for c in cs {
//         let seq = compute_char(robot, c)?;
//         println!(
//             "char {} {:?}",
//             c,
//             seq.iter().map(|&v| dir_to_key(&v)).collect::<String>()
//         );
//         ret.extend(seq);
//     }
//     Ok(ret)
// }

// Key is (destination, robot_states)
// Value is (commands, new_robot_states)
type Cache = HashMap<(char, Vec<char>), (Vec<char>, Vec<char>)>;

fn compute_code_dynamic(
    code: &str,
    robot_states: &[char],
    robot_keypads: &[&Keypad],
    cache: &mut Cache,
) -> Vec<char> {
    let mut robot_states = robot_states.to_vec();
    let mut ret = Vec::new();
    for c in code.chars() {
        let (commands, new_robot_states) =
            compute_char_dynamic(c, robot_states.as_slice(), robot_keypads, cache);
        println!("{} -> {:?}", c, commands);
        robot_states = new_robot_states;
        ret.extend(commands);
    }
    ret
}

/// Computes the shortest path from `from` to `to` using the robot states.
/// Returns the commands and the new robot state once moved there.
fn compute_char_dynamic(
    to: char,
    robot_states: &[char],
    robot_keypads: &[&Keypad],
    cache: &mut Cache,
) -> (Vec<char>, Vec<char>) {
    assert_eq!(robot_states.len(), robot_keypads.len());

    let key = (to, robot_states.to_vec());
    if let Some(v) = cache.get(&key) {
        return v.clone();
    }
    // base state, the robot can press the button directly
    if robot_states.is_empty() {
        return (vec![to], vec![]);
    }
    // If we're already there, we don't need to do anything
    if robot_states[0] == to {
        return (vec![], robot_states.to_vec());
    }

    let my_char = robot_states[0];
    let my_keypad = robot_keypads[0];

    let curpos = my_keypad.get(&my_char).expect("Invalid robot state");

    let target = my_keypad.get(&to).expect("Invalid target key");

    let diff = *target - *curpos;
    // Get all the possible ways we can make the very next move
    let possible_next_directions = [0, 1]
        .into_iter()
        .filter(|&i| diff[i] != 0)
        .map(|i| {
            // Make a direction vector
            let mut dir = I8Vec2::ZERO;
            dir[i] = diff[i].signum();
            dir
        })
        .filter_map(|dir| {
            let next_pos = curpos + dir;
            // Find the next position in the keypad
            let next_char = my_keypad.iter().find(|(_c, &v)| v == next_pos)?;
            Some((*next_char.0, dir))
        });

    let mut options = Vec::new();
    for (next_char, next_dir) in possible_next_directions {
        // compute what it will take to go to the next character
        let key_to_move_in_dir = dir_to_key(&next_dir);
        let (move_commands, new_robot_states) = compute_char_dynamic(
            key_to_move_in_dir,
            &robot_states[1..],
            &robot_keypads[1..],
            cache,
        );

        // back up to our level
        let mut new_robot_states = new_robot_states;
        new_robot_states.insert(0, next_char);

        // And now, once we're there, to get to the end
        let (end_commands, end_robot_states) =
            compute_char_dynamic(to, &new_robot_states, &robot_keypads, cache);

        // Now press 'A'
        let (a_commands, a_robot_states) =
            compute_char_dynamic('A', &end_robot_states, &robot_keypads, cache);

        // Combine all the commands
        let mut commands = move_commands;
        commands.extend_from_slice(&end_commands);
        commands.extend_from_slice(&a_commands);
        options.push((commands, a_robot_states));
    }
    // Find the shortest path
    let (commands, new_robot_states) = options
        .into_iter()
        .min_by_key(|(commands, _)| commands.len())
        .expect("No path found");

    // This is our answer
    cache.insert(key, (commands.clone(), new_robot_states.clone()));

    (commands, new_robot_states)
}

// fn compute_char(robot: &mut KeypadProgress, c: char) -> Result<Vec<I8Vec2>> {
//         .get(&c)
//         .ok_or_else(|| anyhow::anyhow!("Invalid key: {}", c))?;
//     let child = if let Some(child) = robot.child.as_ref() {
//         child
//     } else {
//         robot.pos = *target;
//         return Ok(vec![key_to_dir(c)]);
//     };

//     let path_digit = pathfinding::directed::dijkstra::dijkstra(
//         &(robot.pos, child.clone(), vec![]),
//         |(pos, child, _)| {
//             const DIRECTIONS: [I8Vec2; 4] = [
//                 I8Vec2::new(0, 1),
//                 I8Vec2::new(0, -1),
//                 I8Vec2::new(1, 0),
//                 I8Vec2::new(-1, 0),
//             ];
//             let poss = DIRECTIONS.iter().filter_map(|&dir| {
//                 let next_pos = *pos + dir;
//                 _ = robot.keypad.iter().find(|(_, &v)| v == next_pos)?;
//                 Some((dir, next_pos))
//             });
//             let costs = poss.map(|(next_dir, next_pos)| {
//                 let dirc = dir_to_key(&next_dir);
//                 let mut child = child.clone();
//                 let subroute = compute_char(&mut child, dirc).expect("ans");

//                 ((next_pos, child, subroute.clone()), subroute.len())
//             });
//             costs.collect::<Vec<_>>()
//         },
//         |(pos, _, _)| *pos == *target,
//     )
//     .ok_or_else(|| anyhow::anyhow!("No path to target"))?;

//     let path = path_digit.0;

//     //    println!("path {:?}", path);

//     let last = path.last().unwrap();

//     let mut last_child = last.1.clone();
//     let press_a = compute_char(&mut last_child, 'A')?;

//     let mut ret = path
//         .into_iter()
//         .map(|(_, _, moves)| moves)
//         .flatten()
//         .collect::<Vec<_>>();
//     // .into_iter()
//     // .flat_map(|(_, _, moves)| moves)
//     // .collect::<Vec<_>>();

//     ret.extend(press_a);

//     robot.pos = *target;
//     robot.child = Some(last_child);

//     Ok(ret)
// }

#[derive(Clone, Eq)]
struct KeypadProgress<'a> {
    keypad: &'a Keypad,
    pos: I8Vec2,
    child: Option<Box<KeypadProgress<'a>>>,
}
impl std::fmt::Debug for KeypadProgress<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("KeypadProgress")
            //   .field("pos", &self.pos)
            //  .field("child", &self.child)
            .finish()
    }
}
impl PartialEq for KeypadProgress<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos && self.child == other.child
    }
}
impl std::hash::Hash for KeypadProgress<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.pos.hash(state);
        self.child.hash(state);
    }
}
impl<'a> KeypadProgress<'a> {
    fn new(keypad: &'a Keypad, child: Option<Box<KeypadProgress<'a>>>) -> Result<Self> {
        let pos = *keypad
            .get(&'A')
            .ok_or_else(|| anyhow::anyhow!("Invalid start position"))?;

        Ok(Self { keypad, pos, child })
    }
}

fn compute_sequence(
    keypad: &HashMap<char, I8Vec2>,
    input: impl IntoIterator<Item = char>,
) -> Result<Vec<I8Vec2>> {
    let mut cur_pos = *keypad
        .get(&'A')
        .ok_or_else(|| anyhow::anyhow!("Invalid start position"))?;

    let mut sequence = Vec::new();
    for key in input {
        let next_pos = keypad
            .get(&key)
            .ok_or_else(|| anyhow::anyhow!("Invalid key: {}", key))?;

        while cur_pos != *next_pos {
            let diff = *next_pos - cur_pos;

            let mut possible = [0, 1]
                .into_iter()
                .filter(|&i| diff[i] != 0)
                .map(|i| {
                    // Make a direction vector
                    let mut dir = I8Vec2::ZERO;
                    dir[i] = diff[i].signum();
                    dir
                })
                .filter(|&dir| {
                    let next_pos = cur_pos + dir;
                    // Find the next position in the keypad
                    let next_pos_valid = keypad.iter().any(|(_, &v)| v == next_pos);
                    next_pos_valid
                });
            let next_dir = possible.next().ok_or_else(|| {
                anyhow::anyhow!(
                    "Invalid move from {:?} to {:?} (diff: {:?})",
                    cur_pos,
                    next_pos,
                    diff
                )
            })?;
            sequence.push(next_dir);
            cur_pos += next_dir;
        }
        sequence.push(I8Vec2::ZERO);
    }

    Ok(sequence)
}

fn key_to_dir(c: char) -> I8Vec2 {
    match c {
        '>' => I8Vec2::new(1, 0),
        '<' => I8Vec2::new(-1, 0),
        '^' => I8Vec2::new(0, 1),
        'v' => I8Vec2::new(0, -1),
        'A' => I8Vec2::ZERO,
        _ => panic!("Invalid key: {}", c),
    }
}

fn dir_to_key(dir: &I8Vec2) -> char {
    match dir {
        I8Vec2 { x: 1, y: 0 } => '>',
        I8Vec2 { x: -1, y: 0 } => '<',
        I8Vec2 { x: 0, y: 1 } => '^',
        I8Vec2 { x: 0, y: -1 } => 'v',
        I8Vec2 { x: 0, y: 0 } => 'A',
        _ => panic!("Invalid direction: {:?}", dir),
    }
}

fn keypad_to_xy<INNER>(keypad: &[INNER]) -> HashMap<char, I8Vec2>
where
    INNER: AsRef<[char]>,
{
    keypad
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            let y = keypad.len() - y - 1;
            row.as_ref()
                .iter()
                .enumerate()
                .filter_map(move |(x, &key)| {
                    if key == ' ' {
                        None
                    } else {
                        Some((key, I8Vec2::new(x as i8, y as i8)))
                    }
                })
        })
        .collect()
}

fn dir_to_keypad(dirs: impl IntoIterator<Item = I8Vec2>) -> impl Iterator<Item = char> {
    dirs.into_iter().map(|dir| dir_to_key(&dir))
}

fn solve_part1_impl(input: &Data) -> Result<usize> {
    let numeric_keypad = [
        ['7', '8', '9'],
        ['4', '5', '6'],
        ['1', '2', '3'],
        [' ', '0', 'A'],
    ];
    let numeric_keypad = keypad_to_xy(numeric_keypad.as_slice());

    let directional_keypad = [[' ', '^', 'A'], ['<', 'v', '>']];
    let directional_keypad = keypad_to_xy(directional_keypad.as_slice());

    let mut sum = 0;

    let keypads = [&numeric_keypad]; // &directional_keypad, &directional_keypad];
    let mut cache = Cache::new();
    for &code in &input.codes {
        let robot_states = ['A']; //, 'A', 'A'];
        println!("code {}", code);
        {
            let sequence = compute_code_dynamic(code, &robot_states, &keypads, &mut cache);
            let sequence = sequence.into_iter().collect::<String>();
            println!("compute_path path {}", sequence);
            let num = code[..code.len() - 1].parse::<usize>()?;
            sum += num * sequence.len();
            println!("{} * {}", num, sequence.len());
        }
    }
    //     continue;

    //     let robot0 = compute_sequence(&numeric_keypad, code.chars())?;
    //     let robot1 = compute_sequence(&directional_keypad, dir_to_keypad(robot0))?;
    //     println!(
    //         "robot1 sequence {}",
    //         dir_to_keypad(robot1.clone()).collect::<String>()
    //     );
    //     let robot2 = compute_sequence(&directional_keypad, dir_to_keypad(robot1))?;

    //     let sequence = dir_to_keypad(robot2).collect::<String>();

    //     println!("{}: {} {}", code, sequence, sequence.len());

    Ok(sum)
}

fn solve_part2_impl(_input: &Data) -> Result<usize> {
    // XXX: Solving logic for part 2
    Ok(0)
}

/// Solution to part 1
#[aoc(day21, part1)]
fn solve_part1(input: &str) -> Result<usize> {
    let input = Data::parse(input).context("input parsing")?;
    solve_part1_impl(&input)
}

/// Solution to part 2
#[aoc(day21, part2)]
fn solve_part2(input: &str) -> Result<usize> {
    let input = Data::parse(input).context("input parsing")?;
    solve_part2_impl(&input)
}

/// Problem input
#[derive(Debug)]
struct Data<'a> {
    // XXX: Change this to the actual data structure
    codes: Vec<&'a str>,
}
impl<'a> Data<'a> {
    fn parse(s: &'a str) -> Result<Self> {
        let s = s.lines();
        let codes = s.collect();

        Ok(Data { codes })
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
            126384
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            solve_part2(&test_data(super::DAY).unwrap()).unwrap(),
            0 // XXX: Update this to the expected value for part 2 sample data.
        );
    }
}
