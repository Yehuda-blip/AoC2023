use std::{cmp::Ordering, str::FromStr};

use anyhow::{Context, anyhow};
use itertools::Itertools;

const ACES_AND_FACES: [char; 5] = ['T', 'J', 'Q', 'K', 'A'];
const CARDS_PER_HAND: usize = 5;

#[derive(Debug, Clone, Copy)]
pub(super) enum Rank {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl PartialEq for Rank {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Rank {}

impl PartialOrd for Rank {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Rank {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as i32).cmp(&(*other as i32))
    }
}

pub(super) struct CamelHand {
    hand: String,
    rank: Rank,
    bid: u32
}

pub(super) fn compare_cards(my_card: &char, other_card: &char) -> Ordering {
    match (
        ACES_AND_FACES.iter().position(|c| c == my_card),
        ACES_AND_FACES.iter().position(|c| c == other_card),
    ) {
        (Some(my_card_level), Some(other_card_level)) => my_card_level.cmp(&other_card_level),
        (Some(_), None) => Ordering::Greater,
        (None, Some(_)) => Ordering::Less,
        (None, None) => match (my_card.to_digit(10), other_card.to_digit(10)) {
            (Some(my_card_level), Some(other_card_level)) => my_card_level.cmp(&other_card_level),
            _ => panic!("comparing invalid cards '{}', '{}'", my_card, other_card),
        },
    }
}

fn compare_equal_ranked_hands(my_hand: &String, other_hand: &String) -> std::cmp::Ordering {
    match my_hand
        .chars()
        .zip(other_hand.chars())
        .skip_while(|(my_card, other_card)| my_card == other_card)
        .next()
    {
        None => Ordering::Equal,
        Some((my_card, other_card)) => compare_cards(&my_card, &other_card),
    }
}

impl PartialEq for CamelHand {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for CamelHand {}

impl PartialOrd for CamelHand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CamelHand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.rank.cmp(&other.rank) {
            Ordering::Equal => compare_equal_ranked_hands(&self.hand, &other.hand),
            inequality => inequality
        }
    }
}

impl FromStr for CamelHand {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (hand, bid) = s.split_once(' ').context("bad hand description, no space")?;
        if hand.len() != CARDS_PER_HAND || hand.chars().any(|c| (!c.is_digit(10) || ['0', '1'].contains(&c)) && !ACES_AND_FACES.contains(&c)) {
            return Err(anyhow!("bad Camel Cards hand"))
        }
        let bid = bid.parse().with_context(|| "could not parse bid")?;
        let mut card_count = hand.chars().sorted().dedup_with_count().sorted_by(|(count, _), (other_count, _)| count.cmp(other_count).reverse());
        let rank = match card_count.next() {
            Some((5, _)) => Rank::FiveOfAKind,
            Some((4, _)) => Rank::FourOfAKind,
            Some((3, _)) => match card_count.next() {
                Some((2, _)) => Rank::FullHouse,
                Some((1, _)) => Rank::ThreeOfAKind,
                _ => panic!("invalid hand passed validations, could not compute rank")         
            },
            Some((2, _)) => match card_count.next() {
                Some((2, _)) => Rank::TwoPair,
                Some((1, _)) => Rank::OnePair,
                _ => panic!("invalid hand passed validations, could not compute rank")   
            }
            Some((1, _)) => Rank::HighCard,
            _ => panic!("invalid hand passed validations, could not compute rank") 
        };
        Ok(CamelHand { hand: String::from(hand), rank, bid})
    }
}

impl CamelHand {
    pub(super) fn get_bid(&self) -> u32 {
        self.bid
    }

    pub(super) fn get_hand(&self) -> &String {
        &self.hand
    }

    pub(super) fn get_rank(&self) -> Rank{
        self.rank
    }
}