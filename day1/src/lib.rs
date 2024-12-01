use std::{error::Error, io::BufRead as _};

use anyhow::Result;

// Read a two-column file and return the values as two vectors
fn read_data<T: std::str::FromStr<Err = E>, E: Error + Send + Sync + 'static>(
    filename: &str,
) -> Result<(Vec<T>, Vec<T>)> {
    let mut data = std::io::BufReader::new(std::fs::File::open(filename)?).lines();

    return data.try_fold((vec![], vec![]), |(mut left, mut right), line| {
        let line = line?;
        let mut d = line.split_whitespace().map(|v| v.parse::<T>());

        left.push(
            d.next()
                .ok_or_else(|| anyhow::anyhow!("No value found for left"))??,
        );
        right.push(
            d.next()
                .ok_or_else(|| anyhow::anyhow!("No value found for right"))??,
        );

        Ok((left, right))
    });

    // let mut left = vec![];
    // let mut right = vec![];

    // for d in data {
    //     let mut d = d.split_whitespace().map(|v| v.parse::<T>());

    //     left.push(
    //         d.next()
    //             .ok_or_else(|| anyhow::anyhow!("No value found for left"))??,
    //     );
    //     right.push(
    //         d.next()
    //             .ok_or_else(|| anyhow::anyhow!("No value found for right"))??,
    //     );
    // }

    // Ok((left, right))
}

/// Counts how many times a value appears in a slice
fn count_value<T: PartialEq>(data: &[T], value: T) -> usize {
    data.iter().filter(|&v| v == &value).count()
}

pub fn compute_answer(filename: &str) -> Result<usize> {
    let (mut left, mut right) = read_data::<usize, _>(filename)?;

    left.sort();
    right.sort();

    // let mut sum = 0;
    // for (l, r) in left.into_iter().zip(right.into_iter()) {
    //     sum += l.max(r) - l.min(r);
    // }

    // Ok(sum)
    Ok(left
        .into_iter()
        .zip(right.into_iter())
        .map(|(l, r)| l.max(r) - l.min(r))
        .sum())
}

pub fn compute_answer2(filename: &str) -> Result<usize> {
    let (left, right) = read_data::<usize, _>(filename)?;

    // let mut sum = 0;
    // for value in left {
    //     sum += count_value(&right, value) * value;
    // }

    // Ok(sum)
    Ok(left.into_iter().map(|v| count_value(&right, v) * v).sum())
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
