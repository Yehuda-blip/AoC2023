use anyhow::{Context, Result};

fn opt(prev: &Vec<u64>, record: &Vec<char>, broken_seq: &Vec<usize>, record_i: usize, seq_i: usize) -> u64 {
    if seq_i == 0 {
        if let Some((first_forced_broken, _)) = record.iter().enumerate().filter(|(_i, c)| **c == '#').next() {
            if record_i > first_forced_broken {
                return 0
            }
        }
    }
    
    let this_seq = broken_seq[seq_i];
    if record_i + this_seq > record.len() {
        return 0
    }
    if record[record_i..record_i+this_seq].contains(&'.') {
        return 0
    }
    if record_i > 0 && record[record_i - 1] == '#' {
        return 0
    }
    if record_i + this_seq < record.len() && record[record_i + this_seq] == '#' {
        return 0
    }
    if prev.len() == 0 {
        return 1
    }
    let previous = broken_seq[seq_i-1];
    if previous + 1 > record_i {
        return 0;
    }
    let last_broken_index = record[0..record_i].iter().enumerate().filter(|(_i, c)| **c == '#').last();
    match last_broken_index {
        None => {
            let end = 0.max(record_i as isize - previous as isize) as usize;
            prev[0..end].iter().sum()
        }
        Some((must_cover, _c)) => {
            let end = 0.max(record_i as isize - previous as isize) as usize;
            let start = 0.max(must_cover as isize + 1 - previous as isize) as usize;
            prev[start..end].iter().sum()
        }
    }
}

pub fn options(record: &mut Vec<char>, broken_seq: &mut Vec<usize>) -> u64 {
    let mut prev: Vec<u64> = vec![];
    let mut now: Vec<u64> = vec![];

    record.push('.');
    record.push('?');
    broken_seq.push(1);
    for seq in 0..broken_seq.len() {
        for place in 0..record.len() {
            now.push(opt(&prev, record, broken_seq, place, seq));
        }
        prev = now;
        now = vec![];
    }
    prev[prev.len() - 1]
}

pub fn solve(input: &String) -> Result<String> {
    let res: u64 = itertools::process_results(input.lines().map(|line| {
        let (record_str, sequence_str) = line.split_once(' ').context("no space between record and sequences")?;
        let mut record = record_str.chars().collect();
        let mut sequences = sequence_str.split(',').map(|num| num.parse().with_context(|| "bad parse")).collect::<Result<Vec<usize>, anyhow::Error>>()?;
        let res = options(&mut record, &mut sequences);
        // println!("{}", res);
        return Ok::<u64, anyhow::Error>(res)
    }), |it| it.sum())?;
    Ok(res.to_string())
}

// mod tests {
//     use super::*;

//     #[test]
//     fn test_options_1() {
//         let test = (&mut vec!['?','?','?','?','?','?'], &mut vec![1,1,1]);
//         let result = options(test.0, test.1);
//         assert_eq!(result, 4)
//     }

//     #[test]
//     fn test_options_2() {
//         let test = (&mut vec!['?','?','#','?','?','?'], &mut vec![1,1,1]);
//         let result = options(test.0, test.1);
//         assert_eq!(result, 2)
//     }

//     #[test]
//     fn test_options_3() {
//         let test = (&mut vec!['?','?','?','?','?','?'], &mut vec![1,2,1]);
//         let result = options(test.0, test.1);
//         assert_eq!(result, 1)
//     }

//     #[test]
//     fn test_options_4() {
//         let test = (&mut vec!['?','?','?','?','?','?'], &mut vec![2,2]);
//         let result = options(test.0, test.1);
//         assert_eq!(result, 3)
//     }

//     #[test]
//     fn test_options_5() {
//         let test = (&mut "..####.#?#.?????##?.".chars().collect(), &mut vec![4,1,1,6]);
//         let result = options(test.0, test.1);
//         assert_eq!(result, 2)
//     }

//     #[test]
//     fn test_options_6() {
//         let test = (&mut "?###????????".chars().collect(), &mut vec![3,2,1]);
//         let result = options(test.0, test.1);
//         assert_eq!(result, 10)
//     }
    
//     #[test]
//     fn test_options_7() {
//         let test = (&mut "?.??????.##??#.#?.#".chars().collect(), &mut vec![1,1,2,5,1,1]);
//         let result = options(test.0, test.1);
//         assert_eq!(result, 7)
//     }
    
//     #[test]
//     fn test_options_8() {
//         let test = (&mut "?#???##??##??#???#??".chars().collect(), &mut vec![8,2,1,3,1]);
//         let result = options(test.0, test.1);
//         assert_eq!(result, 1)
//     }
    
//    #[test]
//     fn test_options_9() {
//         let test = (&mut "????#?#?#?#?#??#??.".chars().collect(), &mut vec![9,8]);
//         let result = options(test.0, test.1);
//         assert_eq!(result, 1)
//     }
        
//    #[test]
//    fn test_options_10() {
//        let test = (&mut "#???#???????????".chars().collect(), &mut vec![1,2,1,6]);
//        let result = options(test.0, test.1);
//        assert_eq!(result, 9)
//    }

//    #[test]
//    fn test_options_11() {
//        let mut test = (&mut "??#?.??.#..".chars().collect(), &mut vec![3,1]);
//        let result = options(test.0, test.1);
//        assert_eq!(result, 2)
//    }
// }
