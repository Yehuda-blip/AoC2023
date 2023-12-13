use std::collections::HashSet;

use anyhow::{Result, Context};
use itertools::Itertools;

pub fn solve(input: &String) -> Result<String> {
    let res: u32 = itertools::process_results(input.lines().map(|line| {
        let (record_str, sequence_str) = line.split_once(' ').context("no space between record and sequences")?;
        let mut record = record_str.chars().collect();
        let sequences = sequence_str.split(',').map(|num| num.parse().with_context(|| "bad parse")).collect::<Result<Vec<usize>, anyhow::Error>>()?;
        let res = options(&mut record, &sequences, 0,  &mut HashSet::<String>::new());
        // println!("{}", res);
        return Ok::<u32, anyhow::Error>(res)
    }), |it| it.sum())?;
    Ok(res.to_string())
}

fn options(record: &mut Vec<char>, broken_seq: &Vec<usize>, current_index: usize, cache: &mut HashSet<String>) -> u32 {
    if current_index >= record.len() {
        let breaks = record.iter().dedup_with_count().filter(|(num, c)| **c == '#').map(|(num, c)| num).collect::<Vec<_>>();
        if breaks.len() != broken_seq.len() {
            return 0
        }
        if breaks.iter().zip(broken_seq.iter()).any(|(s1, s2)| s1 != s2) {
            return 0
        }
        return 1
    };
    let now = record[current_index];
    let count = match now {
        '?' => {
            record[current_index] = '.';
            let with_dot = match cache.contains(&String::from_iter(record[0..=current_index].iter())) {
                true => 0,
                false =>  {
                    let val = options(record, broken_seq, current_index + 1, cache);
                    cache.insert(String::from_iter(record[0..=current_index].iter()));
                    val
                }
            };
            record[current_index] = '#';
            let with_break = match cache.contains(&String::from_iter(record[0..=current_index].iter())) {
                true => 0,
                false =>  {
                    let val = options(record, broken_seq, current_index + 1, cache);
                    cache.insert(String::from_iter(record[0..=current_index].iter()));
                    val
                }
            };
            record[current_index] = '?';
            with_dot + with_break
        }
        _ => options(record, broken_seq, current_index + 1, cache)
    };
    return count;
}

mod tests {
    use super::*;

    fn runner(record: &mut Vec<char>, broken_seq: &Vec<usize>) -> u32 {
        options(record, broken_seq, 0, &mut HashSet::new())
    }

    #[test]
    fn test_options_1() {
        let mut test = (&mut vec!['?','?','?','?','?','?'], &vec![1,1,1]);
        let result = runner(&mut test.0, test.1);
        assert_eq!(result, 4)
    }

//     #[test]
//     fn test_options_2() {
//         let mut test = (&mut vec!['?','?','#','?','?','?'], &vec![1,1,1]);
//         let result = runner(&mut test.0, test.1);
//         assert_eq!(result, 2)
//     }

//     #[test]
//     fn test_options_3() {
//         let mut test = (&mut vec!['?','?','?','?','?','?'], &vec![1,2,1]);
//         let result = runner(&mut test.0, test.1);
//         assert_eq!(result, 1)
//     }

//     #[test]
//     fn test_options_4() {
//         let mut test = (&mut vec!['?','?','?','?','?','?'], &vec![2,2]);
//         let result = runner(&mut test.0, test.1);
//         assert_eq!(result, 3)
//     }

//     #[test]
//     fn test_options_5() {
//         let mut test = (&mut "..####.#?#.?????##?.".chars().collect(), &vec![4,1,1,6]);
//         let result = runner(&mut test.0, test.1);
//         assert_eq!(result, 2)
//     }

//     #[test]
//     fn test_options_6() {
//         let mut test = (&mut "?###????????".chars().collect(), &vec![3,2,1]);
//         let result = runner(&mut test.0, test.1);
//         assert_eq!(result, 10)
//     }
    
//     #[test]
//     fn test_options_7() {
//         let mut test = (&mut "?.??????.##??#.#?.#".chars().collect(), &vec![1,1,2,5,1,1]);
//         let result = runner(&mut test.0, test.1);
//         assert_eq!(result, 7)
//     }
    
//     #[test]
//     fn test_options_8() {
//         let mut test = (&mut "?#???##??##??#???#??".chars().collect(), &vec![8,2,1,3,1]);
//         let result = runner(&mut test.0, test.1);
//         assert_eq!(result, 1)
//     }
    
//    #[test]
//     fn test_options_9() {
//         let mut test = (&mut "????#?#?#?#?#??#??.".chars().collect(), &vec![9,8]);
//         let result = runner(&mut test.0, test.1);
//         assert_eq!(result, 1)
//     }
        
//    #[test]
//    fn test_options_10() {
//        let mut test = (&mut "#???#???????????".chars().collect(), &vec![1,2,1,6]);
//        let result = runner(&mut test.0, test.1);
//        assert_eq!(result, 9)
//    }
}
