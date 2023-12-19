use std::collections::HashSet;

use anyhow::{Result, anyhow, Ok};
use itertools::Itertools;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub(super) enum Direction {
    North,
    East,
    South,
    West
}

// static mut count: i64 = 0;

pub(super) fn step(orientation: Direction, location: (usize, usize), marked: &mut HashSet<(Direction, (usize, usize))>, platform: &Vec<Vec<char>>) {
    if marked.contains(&(orientation, location)) {
        return
    }
    marked.insert((orientation, location));
    // unsafe {
    //     count += 1;
    //     println!("{}", count);
    // }
    // let view = platform.iter().enumerate().map(|(i, row)| {
    //     row.iter().enumerate().map(|(j, c)| {
    //         if marked.iter().any(|(dir, mloc)| {
    //             (i, j) == *mloc
    //         }) {
    //             return '#'
    //         }
    //         return *c;
    //     }).collect_vec()
    // }).collect_vec();
    // view.iter().for_each(|row| {
    //     println!("{:?}", row);
    // });
    // println!("\n");

    match platform[location.0][location.1] {
        '\\' => match orientation {
            Direction::North if location.1 > 0 => {
                step(Direction::West, (location.0, location.1 - 1), marked, platform)
            },
            Direction::East if location.0 < platform.len() - 1 => {
                step(Direction::South, (location.0 + 1, location.1), marked, platform)
            },
            Direction::South if location.1 < platform[0].len() - 1 => {
                step(Direction::East, (location.0, location.1 + 1), marked, platform)
            },
            Direction::West if location.0 > 0 => {
                step(Direction::North, (location.0 - 1, location.1), marked, platform)
            },
            _ => {return;}
        },
        '/' => match orientation {
            Direction::North if location.1 < platform[0].len() - 1 => {
                step(Direction::East, (location.0, location.1 + 1), marked, platform)
            },
            Direction::East if location.0 > 0 => {
                step(Direction::North, (location.0 - 1, location.1), marked, platform)
            },
            Direction::South if location.1 > 0 => {
                step(Direction::West, (location.0, location.1 - 1), marked, platform)
            },
            Direction::West if location.0 < platform.len() - 1 => {
                step(Direction::South, (location.0 + 1, location.1), marked, platform)
            },
            _ => {return;}
        },
        '-' => match orientation {
            Direction::North | Direction::South => {
                if location.1 > 0 {
                    step(Direction::West, (location.0, location.1 - 1), marked, platform)
                }
                if location.1 < platform[0].len() - 1 {
                    step(Direction::East, (location.0, location.1 + 1), marked, platform)
                }
            },
            Direction::East if location.1 < platform[0].len() - 1 => {
                step(Direction::East, (location.0, location.1 + 1), marked, platform)
            }
            Direction::West if location.1 > 0 => {
                step(Direction::West, (location.0, location.1 - 1), marked, platform)
            }
            _ => {return;}
        },
        '|' => match orientation {
            Direction::East | Direction::West => {
                if location.0 > 0 {
                    step(Direction::North, (location.0 - 1, location.1), marked, platform)
                }
                if location.0 < platform.len() - 1 {
                    step(Direction::South, (location.0 + 1, location.1), marked, platform)
                }
            },
            Direction::North if location.0 > 0 => {
                step(Direction::North, (location.0 - 1, location.1), marked, platform)
            }
            Direction::South if location.0 < platform.len() - 1 => {
                step(Direction::South, (location.0 + 1, location.1), marked, platform)
            }
            _ => {return;}
        }
        '.' => {
            let next_loc = match orientation {
                Direction::North if location.0 > 0 => {
                    (location.0 - 1, location.1)
                },
                Direction::East if location.1 < platform[0].len() - 1 => {
                    (location.0, location.1 + 1)
                },
                Direction::South if location.0 < platform.len() - 1 => {
                    (location.0 + 1, location.1)
                },
                Direction::West if location.1 > 0 => {
                    (location.0, location.1 - 1)
                },
                _ => {return;}
            };
            step(orientation, next_loc, marked, platform)
        },
        _ => panic!("unidentified value in platform")
    }
}

fn sanitize(platform: &Vec<Vec<char>>) -> Result<()> {
    if platform.len() < 1 {
        return Err(anyhow!("not enough rows"));
    }
    if platform[0].len() < 1 {
        return Err(anyhow!("not enough rows"));
    }
    match platform.iter().map(|row| {
        if row.len() != platform[0].len() {
            return Err(anyhow!("mismatched row lengths"))
        }
        if row.iter().any(|c| {
            match c {
                '|' | '-' | '\\' | '/' | '.' => false,
                _ => true
            }
        }) {
            return Err(anyhow!("bad char"))
        }
        return Ok(());
        
    }).find(|validation| validation.is_err()) {
        None => Ok(()),
        Some(err) => err
    }
}

pub fn solve(input: &String) -> Result<String> {
    let platform = input.lines().map(|line| {
        line.chars().collect_vec()
    }).collect_vec();

    sanitize(&platform)?;

    let mut marks = HashSet::<(Direction, (usize, usize))>::new();
    step(Direction::East, (0,0), &mut marks, &platform);

    let energized = marks.into_iter().map(|(_, loc)| loc).collect::<HashSet::<(usize, usize)>>();
    return Ok(energized.len().to_string())
}
