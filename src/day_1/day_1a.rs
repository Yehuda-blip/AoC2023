use anyhow::Error;

pub fn solve(input: &String) -> Result<String, Error> {
    Ok(input
        .split('\n')
        .fold(0, |sum, line| {
            sum + (line.chars().find(|c| c.is_digit(10)).unwrap_or('\0') as i32 - '0' as i32) * 10
                + (line.chars().rev().find(|c| c.is_digit(10)).unwrap_or('\0') as i32 - '0' as i32)
        })
        .to_string())
}
