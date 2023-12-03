use std::{collections::VecDeque, io::{self, Write}};
use anyhow::Result;

pub fn solve(input: &String) -> Result<String> {
    return Ok(sum_part_numbers(input).to_string());
}

#[derive(Debug)]
pub(crate) enum InterestPoint {
    PartNumber{ number: i32, start_pos: usize, len: usize },
    Symbol{ pos: usize },
    Gear{ pos: usize }
}

fn sum_part_numbers(input: &String) -> i32 {
    let mut line_queue = VecDeque::<Vec::<InterestPoint>>::new();
    line_queue.push_back(Vec::<InterestPoint>::with_capacity(0));
    line_queue.push_back(Vec::<InterestPoint>::with_capacity(0));
    input.lines().chain("\n".lines()).flat_map(|line| {
        line_queue.push_back(parse_line(line));
        let extracted_part_numbers = extract_previous_line_parts(&line_queue);
        line_queue.pop_front();
        return extracted_part_numbers;
    }).sum()
}

fn extract_previous_line_parts(line_queue: &VecDeque::<Vec::<InterestPoint>>) -> Vec<i32> {
    let pos = |p: &InterestPoint| {
        match p {
            InterestPoint::PartNumber { number: _, start_pos, len: _ } => start_pos.to_owned(),
            InterestPoint::Symbol { pos } => pos.to_owned(),
            InterestPoint::Gear { pos } => pos.to_owned()
        }
    };
    let mut part_numbers: Vec<i32> = vec![];
    let two_lines_ago = line_queue.get(0).unwrap();
    let mut two_lines_ago_i = 0;
    let middle_points = line_queue.get(1).unwrap();
    let just_inserted = line_queue.get(2).unwrap();
    let mut just_inserted_i = 0;
    let capture_by_symbols = |part_number: &InterestPoint, adjacent_vec: &Vec<InterestPoint>, adjacent_vec_i: &mut usize| {
        let mut number_captured = false;
        match part_number {
            InterestPoint::Symbol { pos: _ } => {panic!("FUCKKKK")},
            InterestPoint::Gear { pos: _ } => {panic!("FUCKKKK")},
            InterestPoint::PartNumber { number: _, start_pos, len } => {
                while *start_pos != 0 && *adjacent_vec_i < adjacent_vec.len() && (pos(&adjacent_vec[*adjacent_vec_i])) < (start_pos - 1) {
                    *adjacent_vec_i += 1;
                }
                let mut increased_index_near_target = false;
                while !number_captured && *adjacent_vec_i < adjacent_vec.len() && (pos(&adjacent_vec[*adjacent_vec_i])) <= (start_pos + len) {
                    match adjacent_vec[*adjacent_vec_i] {
                        InterestPoint::PartNumber { .. } => {},
                        InterestPoint::Symbol { .. } if number_captured == false => {number_captured = true;}
                        InterestPoint::Gear { .. } if number_captured == false => {number_captured = true;}
                        _ => {}
                    }
                    *adjacent_vec_i += 1;
                    increased_index_near_target = true;
                }
                if increased_index_near_target {
                    *adjacent_vec_i -= 1;
                }
            }
        }
        return number_captured;
    };
    for (i, part_point) in middle_points.iter().enumerate() {
        match part_point {
            InterestPoint::PartNumber { number, start_pos, len } => {
                let mut captured = false;
                if i > 0 {
                    match middle_points[i-1] {
                        InterestPoint::PartNumber { .. } => {},
                        InterestPoint::Symbol { pos } if pos == start_pos - 1 =>  {
                            captured = true;
                        },
                        InterestPoint::Gear { pos } if pos == start_pos - 1 =>  {
                            captured = true;
                        }
                        _ => {}
                    }
                }
                if i + 1 < middle_points.len() {
                    match middle_points[i+1] {
                        InterestPoint::PartNumber { .. } => {},
                        InterestPoint::Symbol { pos } if pos == start_pos + len =>  {
                            captured = true;
                        },
                        InterestPoint::Gear { pos } if pos == start_pos + len =>  {
                            captured = true;
                        }
                        _ => {}
                    }
                }
                if !captured {
                    captured = capture_by_symbols(part_point, two_lines_ago, &mut two_lines_ago_i);
                }
                if !captured {
                    captured = capture_by_symbols(part_point, just_inserted, &mut just_inserted_i);
                }
                if captured {
                    part_numbers.push(*number)
                }
            },
            _ => {}
        }
    };
    return part_numbers
}

pub(crate) fn parse_line(line: &str) -> Vec<InterestPoint> {
    let mut points = Vec::<InterestPoint>::new();
    let mut parsing_num = 0;
    let mut parsing_len = 0;
    let mut parse_char = |c: char, i: usize| {
        match c {
            '.' => {
                if parsing_len > 0 {
                    points.push(InterestPoint::PartNumber { number: parsing_num, start_pos: i - parsing_len, len: parsing_len });
                    parsing_len = 0;
                    parsing_num = 0;
                }
            }
            _ if c.is_ascii_digit() => {
                parsing_len += 1;
                parsing_num *= 10;
                parsing_num += (c as u32 - '0' as u32) as i32;
            }
            sym => {
                if parsing_len > 0 {
                    points.push(InterestPoint::PartNumber { number: parsing_num, start_pos: i - parsing_len, len: parsing_len });
                    parsing_len = 0;
                    parsing_num = 0;
                }
                match sym {
                    '*' => points.push(InterestPoint::Gear { pos: i }),
                    _ => points.push(InterestPoint::Symbol { pos: i })
                }
                
            }
        }
    };
    line.chars().enumerate().for_each(|(i, c)| parse_char(c,i));
    return points;
}