use anyhow::{Context, Result};

pub fn solve(input: &String) -> Result<String> {
    let histories = input.lines().map(compute_next_history).collect::<Result::<Vec<i32>>>()?;
    print!("{:?}", histories);
    return Ok(histories.iter().sum::<i32>().to_string());
}

fn compute_next_history(line: &str) -> Result<i32> {
    let numbers: Vec<i32> = line.split_whitespace().map(|s| s.parse().context("bad number")).collect::<Result::<Vec<i32>>>()?;
    let len = numbers.len();
    let mut levels = vec![numbers];
    for i in 1..len {
        let compute_from = levels.last().unwrap();
        let mut next = vec![];
        for j in 1..compute_from.len() {
            next.push(compute_from[j] - compute_from[j-1]);
        }
        if !next.iter().any(|val| *val != 0) {
            levels.push(next);
            break;
        }
        levels.push(next);
    }
    let mut acc = 0;
    for i in 0..levels.len() {
        let now = &levels[levels.len() - 1 - i];
        acc += (now[now.len() - 1] - acc) - (now[now.len() - 2] - acc);
    }
    let numbers_again = &levels[0];
    return Ok(numbers_again[numbers_again.len() - 1] + acc);
}