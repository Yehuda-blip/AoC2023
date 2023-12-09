use std::collections::HashMap;

use anyhow::{anyhow, Context, Result};
use itertools::Itertools;

use crate::utils::DOUBLE_LINE_ENDING;

use super::day_8a::{construct_map, Node, User, UNMARKED};

pub fn solve(input: &String) -> Result<String> {
    let (directions, locations_list) = input
        .split_once(DOUBLE_LINE_ENDING)
        .context("bad instructions")?;

    let (start_locations, mut desert_map) = construct_map(locations_list, User::Ghost)?;
    let path_length_factored_matrix = start_locations
        .iter()
        .enumerate()
        .take(10000000)
        .map(|(designated_marker, location_id)| {
            compute_length_of_every_path_from(
                location_id,
                &mut desert_map,
                directions,
                designated_marker as u32 + UNMARKED + 1,
            )
        })
        .collect::<Result<Vec<Vec<Vec<(usize, usize)>>>>>()?;
    let steps = search_for_shortest_collective_path(&path_length_factored_matrix, 0, vec![]);
    return Ok(steps.to_string());
}

fn compute_length_of_every_path_from(
    start_location: &str,
    map: &mut HashMap<&str, Node>,
    directions_strip: &str,
    designated_marker: u32,
) -> Result<Vec<Vec<(usize, usize)>>> {
    let mut curret_location = map
        .get_mut(start_location)
        .context("location isn't on the map")?;
    let mut step_counter: usize = 0;
    let mut path_lengths = vec![];
    let mut directions = directions_strip.chars().cycle();
    while step_counter < 1000 {//curret_location.marker != designated_marker {
        curret_location.marker = designated_marker;
        let next_location = match directions.next() {
            Some('R') => {
                curret_location.right
            }
            Some('L') => {
                curret_location.left
            }
            _ => Err(anyhow!("bad direction in directions"))?,
        };
        curret_location = map.get_mut(next_location).context("directions lead to unmapped location")?;
        step_counter += 1;
        print!("{:?}", curret_location.id);
        if curret_location
            .id
            .chars()
            .last()
            .context("bad id - it's empty")?
            == 'Z'
        {
            path_lengths.push(step_counter)
        }
    }
    let path_lengths_factored = path_lengths
        .into_iter()
        .map(hardcore_factor_like_a_MF)
        .collect();
    Ok(path_lengths_factored)
}

fn hardcore_factor_like_a_MF(mut factoring: usize) -> Vec<(usize, usize)> {
    let primes_below = sieve_method(factoring);
    let mut factors = vec![];
    while factoring > 1 {
        let next_factor = *primes_below.iter().find(|p| factoring % *p == 0).unwrap();
        factoring /= next_factor;
        factors.push(next_factor);
    }
    factors
        .into_iter()
        .sorted()
        .dedup_with_count()
        .collect_vec()
}

fn sieve_method(factoring: usize) -> Vec<usize> {
    let mut sieve = vec![true; factoring + 1];
    for i in 2..=factoring {
        if sieve[i] {
            let mut composite_marker = i + i;
            while composite_marker < sieve.len() {
                sieve[composite_marker] = false;
                composite_marker += i;
            }
        }
    }
    sieve[2..]
        .into_iter()
        .enumerate()
        .filter(|(_i, is_prime)| **is_prime)
        .map(|(i, _is_prime)| i + 2)
        .collect_vec()
}

fn search_for_shortest_collective_path(
    factored_path_matrix: &Vec<Vec<Vec<(usize, usize)>>>,
    i: usize,
    factors_vec: Vec<(usize, usize)>,
) -> u64 {
    if i >= factored_path_matrix.len() {
        let length = factors_vec.iter().fold(1, |acc, (repeats, factor)| {
            acc * (*factor as u64).pow(*repeats as u32)
        });
        return length;
    }

    let mut min = u64::MAX;
    let len = factored_path_matrix[i].len();
    for j in 0..len {
        let factor_union = factor_union(&factors_vec, &factored_path_matrix[i][j]);
        let length = search_for_shortest_collective_path(factored_path_matrix, i + 1, factor_union);
        min = min.min(length);
    }
    return min;
}

fn factor_union(
    factors1: &Vec<(usize, usize)>,
    factors2: &Vec<(usize, usize)>,
) -> Vec<(usize, usize)> {
    let mut i2 = 0;
    let mut factor_union = Vec::<(usize, usize)>::new();
    for i1 in 0..factors1.len() {
        let (repeat1, f1) = factors1[i1];
        let (mut repeat2, mut f2) = factors2[i2];
        while (f2 < f1) && i2 < factors2.len() {
            factor_union.push((repeat2, f2));
            i2 += 1;
            (repeat2, f2) = factors2[i2];
        }
        if f2 == f1 {
            factor_union.push((repeat2.max(repeat1), f2));
            i2 += 1;
        } else {
            factor_union.push((repeat1, f1));
        }
    }
    return factor_union;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sieve() {
        let primes = sieve_method(21);
        assert_eq!(primes, vec![2, 3, 5, 7, 11, 13, 17, 19])
    }

    #[test]
    fn test_factoring() {
        let factors100 = hardcore_factor_like_a_MF(100);
        assert_eq!(factors100, vec![(2,2), (2,5)]);
        let factors118591749 = hardcore_factor_like_a_MF(118591749);
        assert_eq!(factors118591749, vec![(3,3), (2,19), (3,23)]);
    }

    #[test]
    fn test_union() {
        let factors1 = vec![(5, 3), (4, 7), (4, 11), (12, 13)];
        let factors2 = vec![(4, 3), (2, 5), (5, 7), (12, 13)];
        let union = factor_union(&factors1, &factors2);
        assert_eq!(union, vec![(5,3), (2,5), (5,7), (4,11), (12,13)])
    }
}

