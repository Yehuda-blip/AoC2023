use std::{cmp::Ordering, str::FromStr};

use anyhow::Result;
use itertools::Itertools;

use super::camel_hand::{compare_cards, CamelHand, Rank};

const JOKER: char = 'J';

fn remap_with_joker(rank: Rank, jokers: usize) -> Rank {
    match jokers {
        0 => rank,
        1 => match rank {
            Rank::HighCard => Rank::OnePair,
            Rank::OnePair => Rank::ThreeOfAKind,
            Rank::TwoPair => Rank::FullHouse,
            Rank::ThreeOfAKind => Rank::FourOfAKind,
            Rank::FullHouse => Rank::FourOfAKind,
            Rank::FourOfAKind => Rank::FiveOfAKind,
            Rank::FiveOfAKind => Rank::FiveOfAKind
        },
        2 => match rank {
            Rank::OnePair => Rank::ThreeOfAKind,
            Rank::TwoPair => Rank::FourOfAKind,
            Rank::FullHouse => Rank::FiveOfAKind,
            _ => panic!("bad rank computation")
        },
        3 => match rank {
            Rank::ThreeOfAKind => Rank::FourOfAKind,
            Rank::FullHouse => Rank::FiveOfAKind,
            _ => panic!("bad rank computation")
        },
        4 => match rank {
            Rank::FourOfAKind => Rank::FiveOfAKind,
            _ => panic!("bad rank computation")
        }
        5 => match rank {
            Rank::FiveOfAKind => Rank::FiveOfAKind,
            _ => panic!("bad rank computation")
        },
        _ => panic!("too many jokers")
    }
}

fn remap_rank(camel_hand: &CamelHand) -> Rank {
    let jokers = camel_hand.get_hand().chars().filter(|card| *card == JOKER).count();
    remap_with_joker(camel_hand.get_rank(), jokers)
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

    use crate::day_7::camel_hand::CamelHand;

    use super::compare_hands_with_jokers;

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
