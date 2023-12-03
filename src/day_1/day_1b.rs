use anyhow::{Result, anyhow};

const S_TO_N: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

#[derive(PartialEq)]
pub enum Direction {
    Front,
    Back,
}

pub fn solve(input: &String) -> Result<String> {
    let sum = input
        .split('\n')
        .try_fold::<i32, fn(i32, &str)->Result<i32>, Result<i32>>(0, |sum, line| {
            Ok(sum + find(line.as_bytes(), Direction::Front)? * 10 + find(line.as_bytes(), Direction::Back)?)
        })?;
    return Ok(sum.to_string())
}

pub fn find(line: &[u8], direction: Direction) -> Result<i32> {
    let indices: Box<dyn Iterator<Item = usize>> = match direction {
        Direction::Front => Box::new(0..line.len()),
        Direction::Back => Box::new((0..line.len()).rev()),
    };
    indices.into_iter()
        .find_map(|i| {
            if line[i].is_ascii_digit() {
                return Some((line[i] - ('0' as u8)) as i32);
            }
            (0..10).find_map(|j| {
                let num = S_TO_N[j].as_bytes();
                let (start, end);
                if direction == Direction::Front && i + num.len() <= line.len() {
                    (start, end) = (i, (i + num.len()))
                }
                else if direction == Direction::Back && i as i32 - num.len() as i32 + 1 >= 0 {
                    (start, end) = (i + 1 - num.len(), i + 1)
                }
                else {
                    return None;
                }
                if &line[start..end] == num
                {
                    return Some(j as i32);
                }
                return None;
            })
        }).ok_or(anyhow!("could not parse a digit in '{:?}'", line))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find() {
        let test = "three1aastwoned";
        let result = find(test.as_bytes(), Direction::Back).expect("error in test parse");
        assert_eq!(result, 1)
    }
}