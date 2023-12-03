use std::collections::VecDeque;

use super::day_3a::{InterestPoint, parse_line};
use anyhow::Result;

pub fn solve(input: &String) -> Result<String> {
    return Ok(sum_gear_products(input).to_string());
}

fn sum_gear_products(input: &String) -> i32 {
    let mut line_queue = VecDeque::<Vec::<InterestPoint>>::new();
    line_queue.push_back(Vec::<InterestPoint>::with_capacity(0));
    line_queue.push_back(Vec::<InterestPoint>::with_capacity(0));
    input.lines().chain("\n".lines()).flat_map(|line| {
        line_queue.push_back(parse_line(line));
        let extracted_part_numbers = extract_previous_line_gear_products(&line_queue);
        line_queue.pop_front();
        return extracted_part_numbers;
    }).sum()
}

fn extract_previous_line_gear_products(line_queue: &VecDeque::<Vec::<InterestPoint>>) -> Vec<i32> {
    let range = |p: &InterestPoint| {
        match p {
            InterestPoint::PartNumber { number: _, start_pos, len } => start_pos.clone()..(start_pos+len).clone(),
            InterestPoint::Symbol { pos } => pos.clone()..(pos + 1).clone(),
            InterestPoint::Gear { pos } => pos.clone()..(pos + 1).clone()
        }
    };
    let mut gear_products: Vec<i32> = vec![];
    let two_lines_ago = line_queue.get(0).unwrap();
    let mut two_lines_ago_i = 0;
    let middle_points = line_queue.get(1).unwrap();
    let just_inserted = line_queue.get(2).unwrap();
    let mut just_inserted_i = 0;
    let capture_numbers = |part_number: &InterestPoint, adjacent_vec: &Vec<InterestPoint>, adjacent_vec_i: &mut usize| {
        let mut captured_parts_num = vec![];
        match part_number {
            InterestPoint::Symbol { pos: _ } => {panic!("FUCKKKK")},
            InterestPoint::PartNumber { .. } => {panic!("FUCKKKK")},
            InterestPoint::Gear { pos } => {
                while pos != &0 && *adjacent_vec_i < adjacent_vec.len() && range(&adjacent_vec[*adjacent_vec_i]).end < *pos {
                    *adjacent_vec_i += 1;
                }
                let mut increased_index_near_target = false;
                while captured_parts_num.len() <= 2 && *adjacent_vec_i < adjacent_vec.len() && (range(&adjacent_vec[*adjacent_vec_i])).start <= (pos + 1) {
                    match adjacent_vec[*adjacent_vec_i] {
                        InterestPoint::PartNumber { number, start_pos, len } => {
                            captured_parts_num.push(number);
                        },
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
        return captured_parts_num;
    };
    for (i, part_point) in middle_points.iter().enumerate() {
        match part_point {
            InterestPoint::Gear { pos } => {
                let mut captured = vec![];
                if i > 0 {
                    match middle_points[i-1] {
                        InterestPoint::PartNumber { number, start_pos, len  } => {
                            if range(&middle_points[i-1]).end == *pos {
                                captured.push(number)
                            }
                        },
                        _ => {}
                    };
                }
                if i + 1 < middle_points.len() {
                    match middle_points[i+1] {
                        InterestPoint::PartNumber { number, start_pos, len  } => {
                            if range(&middle_points[i+1]).start == (pos + 1) {
                                captured.push(number)
                            }
                        },
                        _ => {}
                    }
                }
                if captured.len() <= 2 {
                    captured.append(&mut capture_numbers(part_point, two_lines_ago, &mut two_lines_ago_i));
                }
                if captured.len() <= 2 {
                    captured.append(&mut capture_numbers(part_point, just_inserted, &mut just_inserted_i));
                }
                if captured.len() == 2 {
                    gear_products.push(captured[0] * captured[1]);
                }
            },
            _ => {}
        }
    };
    return gear_products
}
