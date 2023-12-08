use std::collections::HashMap;
use std::iter::zip;

use once_cell::sync::Lazy;
use regex::Regex;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug, Hash)]
enum CardType {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug, Hash)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Clone, Copy, Debug, Eq, Ord)]
struct Hand {
    cards: [CardType; 5],
    bet: u32,
    hand_type: HandType,
}
impl Hand {
    fn new(hand: &str, bet: &str, use_joker: bool) -> Self {
        assert!(hand.len() == 5);
        let cards: Vec<CardType> = hand
            .chars()
            .map(|c| match c {
                'A' => CardType::Ace,
                'K' => CardType::King,
                'Q' => CardType::Queen,
                'J' => match use_joker {
                    true => CardType::Joker,
                    false => CardType::Jack,
                },
                'T' => CardType::Ten,
                '9' => CardType::Nine,
                '8' => CardType::Eight,
                '7' => CardType::Seven,
                '6' => CardType::Six,
                '5' => CardType::Five,
                '4' => CardType::Four,
                '3' => CardType::Three,
                '2' => CardType::Two,
                _ => panic!(),
            })
            .collect();

        let mut card_array: [CardType; 5] = [CardType::Two; 5];
        card_array.copy_from_slice(&cards);

        Hand {
            cards: card_array,
            bet: bet.parse().unwrap(),
            hand_type: Hand::get_hand_type(card_array),
        }
    }

    fn new_hands(file: &str, use_joker: bool) -> Vec<Hand> {
        const REGEX: &str = r"^(?P<hand>\w{5}) (?P<bet>\d+)$";
        static RE: Lazy<Regex> = Lazy::new(|| Regex::new(REGEX).unwrap());
        file.lines()
            .map(|line| {
                let x = RE.captures(line).unwrap();
                let (_, [hand, bet]) = x.extract();
                Hand::new(hand, bet, use_joker)
            })
            .collect()
    }

    fn get_hand_type(cards: [CardType; 5]) -> HandType {
        let mut counts: HashMap<CardType, u8> = HashMap::new();
        cards.iter().for_each(|&card| {
            let count = counts.entry(card).or_default();
            *count += 1;
        });

        let joker_count = *counts.get(&CardType::Joker).unwrap_or(&0);

        match counts.values().max().unwrap() {
            5 => HandType::FiveOfAKind,
            4 => match joker_count {
                4 | 1 => HandType::FiveOfAKind,
                0 => HandType::FourOfAKind,
                _ => panic!(),
            },
            3 => match joker_count {
                3 => match counts.len() {
                    3 => HandType::FourOfAKind,
                    2 => HandType::FiveOfAKind,
                    _ => panic!(),
                },
                2 => HandType::FiveOfAKind,
                1 => HandType::FourOfAKind,
                0 => match counts.len() {
                    3 => HandType::ThreeOfAKind,
                    2 => HandType::FullHouse,
                    _ => panic!(),
                },
                _ => panic!(),
            },
            2 => match joker_count {
                2 => match counts.len() {
                    4 => HandType::ThreeOfAKind,
                    3 => HandType::FourOfAKind,
                    _ => panic!(),
                },
                1 => match counts.len() {
                    4 => HandType::ThreeOfAKind,
                    3 => HandType::FullHouse,
                    _ => panic!(),
                },
                0 => match counts.len() {
                    4 => HandType::OnePair,
                    3 => HandType::TwoPair,
                    _ => panic!(),
                },
                _ => panic!(),
            },
            1 => match joker_count {
                1 => HandType::OnePair,
                0 => HandType::HighCard,
                _ => panic!(),
            },
            _ => panic!(),
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        if self.hand_type != other.hand_type {
            return false;
        }
        zip(self.cards, other.cards).all(|(a, b)| a == b)
    }
}
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.hand_type.cmp(&other.hand_type) {
            std::cmp::Ordering::Less => Some(std::cmp::Ordering::Less),
            std::cmp::Ordering::Greater => Some(std::cmp::Ordering::Greater),
            std::cmp::Ordering::Equal => {
                for (a, b) in zip(self.cards, other.cards) {
                    let comp = a.cmp(&b);
                    if comp != std::cmp::Ordering::Equal {
                        return Some(comp);
                    }
                }
                Some(std::cmp::Ordering::Equal)
            }
        }
    }
}

pub fn solve_part_1(file: &str) -> Option<u32> {
    let mut hands = Hand::new_hands(file, false);
    hands.sort();
    Some(
        hands
            .iter()
            .enumerate()
            .fold(0, |z, (i, hand)| z + (hand.bet * (i as u32 + 1))),
    )
}

pub fn solve_part_2(file: &str) -> Option<u32> {
    let mut hands = Hand::new_hands(file, true);
    hands.sort();
    Some(
        hands
            .iter()
            .enumerate()
            .fold(0, |z, (i, hand)| z + (hand.bet * (i as u32 + 1))),
    )
}

const DAY: u8 = 7;

pub fn main(file: &String) {
    println!("Solving Day {}", DAY);
    println!("  part 1: {:?}", solve_part_1(&file));
    println!("  part 2: {:?}", solve_part_2(&file));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::inputs::{self, InputType};

    #[test]
    fn solves_first_problem() {
        let content = inputs::get_file(DAY, InputType::Sample);
        assert_eq!(solve_part_1(&content), Some(6440))
    }

    #[test]
    fn solves_second_problem() {
        let content = inputs::get_file(DAY, InputType::Sample);
        assert_eq!(solve_part_2(&content), Some(5905))
    }
}
