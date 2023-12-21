use anyhow::{Context, Result};

use super::day_10a::{find_largest_loop, longtitude_latitude_map};

pub fn solve(input: &String) -> Result<String> {
    let mut map = longtitude_latitude_map(input);
    return Ok(calc_inside(&mut map)?.to_string())
}

pub fn calc_inside(map: &mut Vec<Vec<char>>) -> Result<u32> {
    let start_pos = (0..map.len())
        .flat_map(|i| (0..map[i].len()).map(move |j| (i, j)))
        .find(|(row, col)| map[*row][*col] == 'S')
        .context("no start position found")?;
    let (largest_loop, start_pipe) = find_largest_loop(start_pos, &map)?;
    map[start_pos.0][start_pos.1] = start_pipe;
    let inside: u32 = map.iter().enumerate().map(|(i, row)| {
        let mut inside = false;
        let mut count = 0;
        let mut last_turn = None;
        row.iter().enumerate().for_each(|(j, _pipe)| {
            if largest_loop.contains(&(i, j)) {
                if ['7', 'J'].contains(&map[i][j]) {
                    match last_turn {
                        None => {
                            panic!("your algorithm is problematic");
                        }
                        Some('L') if map[i][j] == 'J' => {
                            last_turn = None;
                        }
                        Some('L') if map[i][j] == '7' => {
                            last_turn = None;
                            inside = !inside;
                        }
                        Some('F') if map[i][j] == '7' => {
                            last_turn = None;
                        }
                        Some('F') if map[i][j] == 'J' => {
                            last_turn = None;
                            inside = !inside;
                        }
                        _ => panic!("your algorithm is problematic 2")
                    }
                }
                else if ['F', 'L'].contains(&map[i][j]) {
                    match last_turn {
                        Some(_) => panic!("your algorithm is problematic 3"),
                        None => last_turn = Some(map[i][j])
                    }
                }
                else if map[i][j] == '|' {
                    inside = !inside;
                }
            } else {
                if inside {
                    count += 1;
                }
            }
        });
        return count;
    }).sum();
    Ok(inside)
}