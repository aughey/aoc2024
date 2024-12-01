use anyhow::Result;

fn read_data(filename: &str) -> Result<(Vec<i32>, Vec<i32>)> {
    let data = std::fs::read_to_string(filename)?;
    let data = data.split('\n');
    // split on new line

    let mut left = vec![];
    let mut right = vec![];

    for d in data {
        let mut d = d.split_whitespace().map(|v| v.parse::<i32>().unwrap());

        left.push(d.next().ok_or_else(|| anyhow::anyhow!("No value found"))?);
        right.push(d.next().ok_or_else(|| anyhow::anyhow!("No value found"))?);
    }

    Ok((left, right))
}

fn count_value(data: &[i32], value: i32) -> usize {
    data.iter().filter(|&&v| v == value).count()
}

fn remove_least(data: &mut Vec<i32>) -> Option<i32> {
    if data.is_empty() {
        return None;
    }
    let mut min = data[0];
    let mut min_index = 0;

    for (i, &v) in data.iter().enumerate() {
        if v < min {
            min = v;
            min_index = i;
        }
    }

    Some(data.remove(min_index))
}

pub fn compute_answer(filename: &str) -> Result<i32> {
    let (mut left, mut right) = read_data(filename)?;

    let mut sum = 0;
    while left.len() > 0 {
        let l = remove_least(&mut left).unwrap();
        let r = remove_least(&mut right).unwrap();
        sum += l.max(r) - l.min(r);
    }

    Ok(sum)
}

pub fn compute_answer2(filename: &str) -> Result<usize> {
    let (mut left, mut right) = read_data(filename)?;

    let mut sum = 0;
    for value in left {
        let count = count_value(&right, value);
        sum += count * value as usize;
    }

    Ok(sum)
}

#[cfg(test)]
mod tests {
    use crate::{compute_answer, read_data};

    #[test]
    fn test_sample() {
        assert_eq!(compute_answer("test.txt").unwrap(), 11);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(crate::compute_answer2("test.txt").unwrap(), 31);
    }
}
