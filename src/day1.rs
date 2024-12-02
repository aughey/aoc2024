use anyhow::Result;
use aoc_runner_derive::{aoc, aoc_generator};
use std::{collections::BTreeMap, error::Error};

pub const DAY: u32 = 1;

#[aoc_generator(day1)]
pub fn read_data(input: &str) -> Result<(Vec<i32>, Vec<i32>)> {
    read_data_generic(input)
}

// Read a two-column file and return the values as two vectors
fn read_data_generic<T: std::str::FromStr<Err = E>, E: Error + Send + Sync + 'static>(
    input: &str,
) -> Result<(Vec<T>, Vec<T>)> {
    // Get a line iterator of the file.
    let data = input.lines();

    // try_fold could be used here, but the complexity of reading and overrules
    // the user-friendliness of a for loop accumulated into stack vecs.
    let mut left = vec![];
    let mut right = vec![];
    for line in data {
        // Split the line into words and parse each word into a T.
        let mut values = line.split_whitespace().map(|v| v.parse::<T>());
        // Define a closure that will return the next value or an error if there are no more values.
        let mut next_value = move || {
            values
                .next()
                .ok_or_else(|| anyhow::anyhow!("No value found"))
        };

        // Push the next two values into the left and right vectors.
        // next_value has two nested Result types for the parsing and the iterator.
        left.push(next_value()??);
        right.push(next_value()??);
    }

    Ok((left, right))
}

/// Counts how many times a value appears in a slice
fn count_value<T: PartialEq>(data: &[T], value: T) -> usize {
    data.iter().filter(|&v| v == &value).count()
}

pub fn part1(input: &str) -> Result<i32> {
    let (left, right) = read_data(input)?;
    Ok(solve_part1(&(left, right)))
}

/// The answer for the first part is defined as the sum of the differences between the two columns
/// when sorted.  Technically, it is the least value from each columns, take the difference of each (abs)
/// and sum them.
#[aoc(day1, part1)]
pub fn solve_part1((left, right): &(Vec<i32>, Vec<i32>)) -> i32 {
    let mut left = left.clone();
    let mut right = right.clone();

    left.sort();
    right.sort();

    left.into_iter()
        .zip(right)
        .map(|(l, r)| (l - r).abs())
        .sum()
}

pub fn part2(input: &str) -> Result<usize> {
    let (left, right) = read_data(input)?;
    Ok(solve_part2(&(left, right)))
}

/// The second part takes the left column and multiplies it by the count of the right column values that are equal to the
/// left column value.  The sum of these values is the answer.
#[aoc(day1, part2)]
pub fn solve_part2((left, right): &(Vec<i32>, Vec<i32>)) -> usize {
    let mut cache = BTreeMap::new();
    let mut counts = move |v| *cache.entry(v).or_insert_with(|| count_value(right, v));

    left.iter().map(|&v| counts(v) * v as usize).sum()
}

#[cfg(test)]
mod tests {
    use crate::{
        day1::{read_data, solve_part1, solve_part2},
        test_data,
    };

    #[test]
    fn test_sample() {
        assert_eq!(
            solve_part1(&read_data(&test_data(super::DAY).unwrap()).unwrap()),
            11
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            solve_part2(&read_data(&test_data(super::DAY).unwrap()).unwrap()),
            31
        );
    }
}
