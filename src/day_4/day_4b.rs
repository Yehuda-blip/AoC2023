use anyhow::Result;
use std::collections::VecDeque;

use super::day_4a::{count_matches, parse_card};

pub fn solve(input: &String) -> Result<String> {
    let mut queue = VecDeque::<(u32, u32, u32)>::new();
    let cards = itertools::process_results(input.lines().map(parse_card), |parsed_lines| {
        parsed_lines
            .enumerate()
            .fold(0, |copy_counter, (i, (targets, scratches))| {
                let match_count = count_matches(targets, scratches);
                let copies = queue
                    .iter()
                    .fold(0, |counter, (_card_id, card_copies, _card_matchess)| {
                        counter + card_copies
                    })
                    + 1;
                queue.push_back((i as u32 + 1, copies, match_count));
                queue
                    .retain(|(card_id, _card_copies, card_matches)| {
                        card_id + card_matches > (i + 1) as u32
                    });
                copy_counter + copies as u64
            })
    })?;
    return Ok(cards.to_string());
}
