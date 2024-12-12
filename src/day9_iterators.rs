use tracing::info;

use crate::Result;
use std::fmt::Display;

pub const DAY: u32 = 9;

/// Enforces the invariant that the string is a string of char digits between 0 and 9
pub struct DigitString<'a>(&'a str);
impl<'a> DigitString<'a> {
    pub fn new(s: &'a str) -> Option<Self> {
        s.chars()
            .all(|c| c.is_ascii_digit())
            .then_some(DigitString(s))
    }
}
impl<'a> IntoIterator for DigitString<'a> {
    type Item = u8;
    type IntoIter = std::iter::Map<std::str::Chars<'a>, fn(char) -> u8>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.chars().map(|c| c.to_digit(10).unwrap() as u8)
    }
}

fn string_to_digits_validated(
    s: &str,
) -> Option<impl DoubleEndedIterator<Item = u8> + Clone + Clone + '_> {
    s.chars()
        .all(|c| c.is_ascii_digit())
        .then_some(s.chars().map(|c| c.to_digit(10).unwrap() as u8))
}

pub fn part1_generator(s: &str) -> Result<impl Iterator<Item = Block> + '_> {
    let digits = string_to_digits_validated(s)
        .ok_or_else(|| anyhow::anyhow!("Not all characters in string are digits"))?;

    let forward = disk_map_to_blocks(forward_disk_generator(digits.clone())).enumerate();

    let last_id = (s.len() + 1) / 2 - 1;
    let block_len: usize = digits.clone().map(usize::from).sum();

    let backward_ids = {
        let mut numbers = (0..=last_id).rev();
        move || numbers.next().unwrap()
    };

    let mut backward = disk_map_to_blocks(disk_generator(
        digits.rev(),
        if s.len() % 2 == 0 {
            DiskMap::Empty(Default::default())
        } else {
            DiskMap::Data(Default::default())
        },
        backward_ids,
    ))
    .enumerate()
    // ignore empty blocks
    .filter(|(_, b)| *b != Block::Empty)
    // invert the index
    .map(move |(i, b)| (block_len - i - 1, b));
    info!(
        "Forward looks like: {:?}",
        forward.clone().collect::<Vec<_>>()
    );
    info!(
        "Backward looks like: {:?}",
        backward.clone().collect::<Vec<_>>()
    );
    info!("first backward block is {:?}", backward.clone().next());

    let mut next_backward = backward.next();

    Ok(forward.map(
        move |(forward_i, forward_block)| match (&forward_block, &next_backward) {
            (_, None) => Block::Empty,
            (Block::Data(_), Some((back_i, _))) => {
                if forward_i <= *back_i {
                    forward_block
                } else {
                    Block::Empty
                }
            }
            (Block::Empty, Some((backward_i, backward_block))) => {
                if *backward_i > forward_i {
                    let ret = backward_block.clone();
                    next_backward = backward.next();
                    ret
                } else {
                    Block::Empty
                }
            }
        },
    ))
}

/// Solution to part 1
fn solve_part1(input: &str) -> Result<u64> {
    Ok(part1_generator(input)?
        .enumerate()
        .map(|(i, b)| match b {
            Block::Data(id) => id * i as u64,
            Block::Empty => 0,
        })
        .sum())
}

/// Solution to part 2
fn solve_part2(_input: &str) -> Result<u64> {
    Ok(0)
}

/// codspeed compatible function
pub fn part1(input: &str) -> impl Display {
    solve_part1(input).unwrap()
}

/// codspeed compatible function
pub fn part2(input: &str) -> impl Display {
    solve_part2(input).unwrap()
}

#[derive(Debug, Clone, PartialEq)]
pub enum Block {
    Empty,
    Data(u64),
}
impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Block::Empty => write!(f, "."),
            Block::Data(x) => write!(f, "{}", x),
        }
    }
}

pub enum DiskMap {
    Empty(u8),
    Data((usize, u8)),
}

pub fn forward_disk_generator(
    digits: impl Iterator<Item = u8> + Clone,
) -> impl Iterator<Item = DiskMap> + Clone {
    let mut numbers = 0..;
    let id_generator = move || numbers.next().unwrap();
    disk_generator(digits, DiskMap::Data(Default::default()), id_generator)
}

pub fn disk_generator(
    digits: impl Iterator<Item = u8> + Clone,
    first_digit_is: DiskMap,
    mut id_generator: impl FnMut() -> usize + Clone,
) -> impl Iterator<Item = DiskMap> + Clone {
    // Depending if the first digit is a data or empty block, we start with a different modulus
    let modulus = if matches!(first_digit_is, DiskMap::Data(_)) {
        0
    } else {
        1
    };
    digits
        .enumerate()
        .map(move |(i, c)| match (i % 2 == modulus, c) {
            (true, count) => DiskMap::Data((id_generator(), count)),
            (false, count) => DiskMap::Empty(count),
        })
}

pub fn disk_map_to_blocks(
    map: impl Iterator<Item = DiskMap> + Clone,
) -> impl Iterator<Item = Block> + Clone {
    map.flat_map(|m| {
        let (count, block) = match m {
            DiskMap::Empty(count) => (count, Block::Empty),
            DiskMap::Data((index, count)) => (count, Block::Data(index as u64)),
        };
        (0..count).map(move |_| block.clone())
    })
}

/// Render the blocks to a string
pub fn blocks_to_string(blocks: impl Iterator<Item = Block>) -> String {
    blocks
        .map(|b| b.to_string())
        .collect::<Vec<String>>()
        .join("")
}

#[cfg(test)]
mod tests {
    use test_log::test;

    use crate::test_data;

    use super::*;

    const TEST_DATA: &str = "2333133121414131402";
    const TEST_RES: &str = "00...111...2...333.44.5555.6666.777.888899";

    #[test]
    fn test_disk_generator() {
        let input = DigitString::new(TEST_DATA).unwrap();
        let disk = forward_disk_generator(input.into_iter());
        let blocks = disk_map_to_blocks(disk);
        assert_eq!(blocks_to_string(blocks), TEST_RES);
    }

    #[test]
    fn test_part1_generator() {
        let blocks = part1_generator("22")
            .unwrap()
            .map(|b| b.to_string())
            .collect::<Vec<_>>();
        assert_eq!(blocks, vec!["0", "0", ".", "."]);

        let blocks = part1_generator("222")
            .unwrap()
            .map(|b| b.to_string())
            .collect::<Vec<_>>();
        assert_eq!(blocks, vec!["0", "0", "1", "1", ".", "."]);

        let blocks = blocks_to_string(part1_generator(TEST_DATA).unwrap());
        assert_eq!(blocks, "0099811188827773336446555566..............");
    }

    #[test]
    fn part1_example() {
        assert_eq!(solve_part1(&test_data(super::DAY).unwrap()).unwrap(), 1928);
    }

    // #[test]
    // fn part2_example() {
    //     assert_eq!(solve_part2(&test_data(super::DAY).unwrap()).unwrap(), 2858);
    // }
}
