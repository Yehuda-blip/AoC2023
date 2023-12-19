use anyhow::Result;

pub(super) fn my_hash(s: &str) -> usize {
    s.as_bytes().iter().fold(0, |hash, ascii| {
        ((hash + (*ascii as usize)) * 17) % 256
    })
}

pub fn solve(input: &String) -> Result<String> {
    Ok(input.split(',').map(|s| my_hash(s)).sum::<usize>().to_string())
}
