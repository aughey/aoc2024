use anyhow::Result;
use std::{collections::HashMap, error::Error, io::BufRead as _};

// Read a two-column file and return the values as two vectors
fn read_data<T: std::str::FromStr<Err = E>, E: Error + Send + Sync + 'static>(
    filename: &str,
) -> Result<(Vec<T>, Vec<T>)> {
    // Get a line iterator of the file.
    let data = std::io::BufReader::new(std::fs::File::open(filename)?).lines();

    // try_fold could be used here, but the complexity of reading and overrules
    // the user-friendliness of a for loop accumulated into stack vecs.
    let mut left = vec![];
    let mut right = vec![];
    for line in data {
        // lines can fail to read.
        let line = line?;
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

/// The answer for the first part is defined as the sum of the differences between the two columns
/// when sorted.  Technically, it is the least value from each columns, take the difference of each (abs)
/// and sum them.
pub fn compute_answer(filename: &str) -> Result<usize> {
    let (mut left, mut right) = read_data::<usize, _>(filename)?;

    left.sort();
    right.sort();

    Ok(left
        .into_iter()
        .zip(right)
        .map(|(l, r)| l.max(r) - l.min(r))
        .sum())
}

/// The second part takes the left column and multiplies it by the count of the right column values that are equal to the
/// left column value.  The sum of these values is the answer.
pub fn compute_answer2(filename: &str) -> Result<usize> {
    let (left, right) = read_data::<usize, _>(filename)?;

    let mut cache = HashMap::new();
    let mut counts = move |v| *cache.entry(v).or_insert_with(|| count_value(&right, v));

    Ok(left.into_iter().map(|v| counts(v) * v).sum())
}

#[cfg(test)]
mod tests {
    use crate::compute_answer;

    #[test]
    fn test_sample() {
        assert_eq!(compute_answer("test.txt").unwrap(), 11);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(crate::compute_answer2("test.txt").unwrap(), 31);
    }
}
