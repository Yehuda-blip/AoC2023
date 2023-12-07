use std::{cmp::Ordering, str::FromStr};

use anyhow::Result;
use itertools::Itertools;

use super::camel_hand::{compare_cards, CamelHand, Rank};

const JOKER: char = 'J';

fn remap_with_joker(rank: Rank) -> Rank {
    match rank {
        Rank::HighCard => Rank::OnePair,
        Rank::OnePair => Rank::ThreeOfAKind,
        Rank::TwoPair => Rank::FullHouse,
        Rank::ThreeOfAKind => Rank::FourOfAKind,
        Rank::FullHouse => Rank::FourOfAKind,
        Rank::FourOfAKind => Rank::FiveOfAKind,
        Rank::FiveOfAKind => Rank::FiveOfAKind // these are 5 jokers
    }
}

fn remap_rank(camel_hand: &CamelHand) -> Rank {
    camel_hand
        .get_hand()
        .chars()
        .filter(|c| *c == JOKER)
        .fold(camel_hand.get_rank(), |rank, _| remap_with_joker(rank))
}

fn compare_cards_with_jokers(this: &char, other: &char) -> Ordering {
    match (this == &JOKER, other == &JOKER) {
        (true, true) => Ordering::Equal,
        (false, true) => Ordering::Greater,
        (true, false) => Ordering::Less,
        (false, false) => compare_cards(this, other),
    }
}

fn compare_hands_with_jokers(this: &CamelHand, other: &CamelHand) -> Ordering {
    match remap_rank(&this).cmp(&remap_rank(&other)) {
        Ordering::Equal => {
            match this
                .get_hand()
                .chars()
                .zip(other.get_hand().chars())
                .skip_while(|(this_card, other_card)| {
                    compare_cards_with_jokers(this_card, other_card) == Ordering::Equal
                })
                .next()
            {
                None => Ordering::Equal,
                Some((this_card, other_card)) => compare_cards_with_jokers(&this_card, &other_card),
            }
        }
        inequality => inequality,
    }
}

pub fn solve(input: &String) -> Result<String> {
    let res =
        itertools::process_results(input.lines().map(|line| CamelHand::from_str(line)), |it| {
            it.sorted_by(|this_hand, other_hand| compare_hands_with_jokers(this_hand, other_hand))
                .enumerate()
                .fold(0, |acc, (rank, hand)| {
                    acc + (rank as u32 + 1) * hand.get_bid()
                })
        })?;
    Ok(res.to_string())
}

#[cfg(test)]
pub mod tests {
    use std::{str::FromStr, cmp::Ordering};

    use anyhow::Result;
    use itertools::Itertools;

    use crate::day_7::{camel_hand::{CamelHand, Rank}, day_7b::remap_with_joker};

    use super::compare_hands_with_jokers;

    #[test]
    fn test_remaps() {
        let hand1 = CamelHand::from_str("2J456 0").expect("wut?");
        assert_eq!(remap_with_joker(hand1.get_rank()), Rank::OnePair);
        let hand1 = CamelHand::from_str("2JJ56 0").expect("wut?");
        assert_eq!(remap_with_joker(remap_with_joker(hand1.get_rank())), Rank::ThreeOfAKind);
        let hand1 = CamelHand::from_str("33J56 0").expect("wut?");
        assert_eq!(remap_with_joker(hand1.get_rank()), Rank::ThreeOfAKind);
        let hand1 = CamelHand::from_str("33J55 0").expect("wut?");
        assert_eq!(remap_with_joker(hand1.get_rank()), Rank::FullHouse);
        let hand1 = CamelHand::from_str("33JJ5 0").expect("wut?");
        assert_eq!(remap_with_joker(remap_with_joker(hand1.get_rank())), Rank::FourOfAKind);
        let hand1 = CamelHand::from_str("3JJJ5 0").expect("wut?");
        assert_eq!(remap_with_joker(remap_with_joker(remap_with_joker(hand1.get_rank()))), Rank::FourOfAKind);
        let hand1 = CamelHand::from_str("33J33 0").expect("wut?");
        assert_eq!(remap_with_joker(remap_with_joker(hand1.get_rank())), Rank::FiveOfAKind);
        let hand1 = CamelHand::from_str("JJJJJ 0").expect("wut?");
        assert_eq!(remap_with_joker(hand1.get_rank()), Rank::FiveOfAKind);
    }

    #[test]
    fn test_sort1() {    
        const TEST: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        let expected = vec!["32T3K", "KK677", "T55J5", "QQQJA", "KTJJT"];
        let hands: Vec<CamelHand> = TEST
            .lines()
            .map(|s| CamelHand::from_str(s))
            .collect::<Result<Vec<CamelHand>>>()
            .expect("could not map");
        let sorted: Vec<&String> = hands
            .iter()
            .sorted_by(|h1, h2| compare_hands_with_jokers(*h1, *h2))
            .map(|h| h.get_hand())
            .collect();
        assert_eq!(expected, sorted)
    }

    #[test]
    fn test_sort2() {
        const TEST: &str = "32T3K 765
KK677 28
KTJJT 220
22222 483
JJJJJ 684";
        let expected = vec!["32T3K", "KK677", "KTJJT", "JJJJJ", "22222"];
        let hands: Vec<CamelHand> = TEST
            .lines()
            .map(|s| CamelHand::from_str(s))
            .collect::<Result<Vec<CamelHand>>>()
            .expect("could not map");
        let sorted: Vec<&String> = hands
            .iter()
            .sorted_by(|h1, h2| compare_hands_with_jokers(*h1, *h2))
            .map(|h| h.get_hand())
            .collect();
        assert_eq!(expected, sorted)
    }

    #[test]
    fn test_cmp() {
        let hand1 = CamelHand::from_str("QQQJA 0").expect("wut?");
        let hand2 = CamelHand::from_str("KTJJT 0").expect("wut?");
        let order = compare_hands_with_jokers(&hand1, &hand2);
        assert_eq!(order, Ordering::Less);
    }

    #[test]
    fn test_cmp1() {
        let hand1 = CamelHand::from_str("QQQJA 0").expect("wut?");
        let hand2 = CamelHand::from_str("KTJJT 0").expect("wut?");
        let order = compare_hands_with_jokers(&hand1, &hand2);
        assert_eq!(order, Ordering::Less);
    }
    
    #[test]
    fn test_cmp2() {
        let hand1 = CamelHand::from_str("JJJ2J 0").expect("wut?");
        let hand2 = CamelHand::from_str("JJJJJ 0").expect("wut?");
        let order = compare_hands_with_jokers(&hand1, &hand2);
        assert_eq!(order, Ordering::Greater);
    }
    
    #[test]
    fn test_cmp3() {
        let hand1 = CamelHand::from_str("JJJ2J 0").expect("wut?");
        let hand2 = CamelHand::from_str("JJJJJ 0").expect("wut?");
        let order = compare_hands_with_jokers(&hand1, &hand2);
        assert_eq!(order, Ordering::Greater);
    }
}
