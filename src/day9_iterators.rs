use crate::Result;
use std::fmt::Display;

pub const DAY: u32 = 9;

/// Enforces the invariant that the string is a string of char digits between 0 and 9
pub struct DigitString<'a>(&'a str);
impl<'a> DigitString<'a> {
    pub fn new(s: &'a str) -> Option<Self> {
        s.chars().all(|c| c.is_digit(10)).then_some(DigitString(s))
    }
}
impl<'a> IntoIterator for DigitString<'a> {
    type Item = u8;
    type IntoIter = std::iter::Map<std::str::Chars<'a>, fn(char) -> u8>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.chars().map(|c| c.to_digit(10).unwrap() as u8)
    }
}

fn part1_generator<'a, I>(digits: I) -> impl Iterator<Item = Block> + 'a
where
    I: Iterator<Item = u8> + Clone + DoubleEndedIterator + 'a,
{
    let forward = disk_map_to_blocks(disk_generator(digits.clone())).enumerate();
    // Do a run to get the length of the blocks
    let len = forward.clone().count();
    let mut backward = disk_map_to_blocks(disk_generator(digits.rev()))
        .enumerate()
        // ignore empty blocks
        .filter(|(_, b)| *b != Block::Empty)
        // invert the index
        .map(move |(i, b)| (len - i - 1, b));

    forward.map(move |(forward_i, forward_block)| match &forward_block {
        Block::Data(_) => forward_block,
        Block::Empty => {
            if let Some((backward_i, backward_block)) = backward.next() {
                if backward_i > forward_i {
                    return backward_block;
                }
            }
            forward_block
        }
    })
}

/// Solution to part 1
fn solve_part1(input: &str) -> Result<u64> {
    //    part1_generator(DigitString::new(input)?)

    Ok(0)
}

/// Solution to part 2
fn solve_part2(input: &str) -> Result<u64> {
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

#[derive(Clone, PartialEq)]
pub enum Block {
    Empty,
    Data(u64),
}
impl ToString for Block {
    fn to_string(&self) -> String {
        match self {
            Block::Empty => '.'.to_string(),
            Block::Data(x) => x.to_string(),
        }
    }
}

pub enum DiskMap {
    Empty(u8),
    Data((usize, u8)),
}

pub fn disk_generator(
    digits: impl Iterator<Item = u8> + Clone,
) -> impl Iterator<Item = DiskMap> + Clone {
    digits.enumerate().map(|(i, c)| match (i % 2 == 0, c) {
        (true, count) => DiskMap::Data((i / 2, count)),
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
        let disk = disk_generator(input.into_iter());
        let blocks = disk_map_to_blocks(disk);
        assert_eq!(blocks_to_string(blocks), TEST_RES);
    }

    #[test]
    fn test_part1_generator() {
        let input = DigitString::new("22").unwrap();
        let blocks = part1_generator(input.into_iter())
            .map(|b| b.to_string())
            .collect::<Vec<_>>();
        assert_eq!(blocks, vec!["0", "0", ".", "."]);

        let input = DigitString::new("222").unwrap();
        let blocks = part1_generator(input.into_iter())
            .map(|b| b.to_string())
            .collect::<Vec<_>>();
        assert_eq!(blocks, vec!["0", "0", "1", "1", ".", "."]);
    }

    #[test]
    fn part1_example() {
        //        assert_eq!(solve_part1(&test_data(super::DAY).unwrap()).unwrap(), 1928);
    }

    // #[test]
    // fn part2_example() {
    //     assert_eq!(solve_part2(&test_data(super::DAY).unwrap()).unwrap(), 2858);
    // }
}
