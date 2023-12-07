advent_of_code::solution!(7);

use itertools::Itertools;
use std::cmp::Ordering;

fn score_card(card: char) -> usize {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        '9' => 9,
        '8' => 8,
        '7' => 7,
        '6' => 6,
        '5' => 5,
        '4' => 4,
        '3' => 3,
        '2' => 2,
        'j' => 0,
        _ => unreachable!(),
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

fn hand_type(cards: &str) -> HandType {
    let card_hist = cards.chars().counts();
    let counts = card_hist
        .iter()
        .filter_map(|(&k, &v)| if k == 'j' { None } else { Some(v) })
        .collect::<Vec<_>>();
    let max_count = *counts.iter().max().unwrap_or(&0);
    let pairs = counts.iter().filter(|&&v| v == 2).count();
    let jokers = *card_hist.get(&'j').unwrap_or(&0);
    match (max_count, pairs, jokers) {
        (c, _, j) if c + j == 5 => HandType::FiveOfAKind,
        (c, _, j) if c + j == 4 => HandType::FourOfAKind,
        (3, p, 0) if p > 0 => HandType::FullHouse,
        (2, 2, 1) => HandType::FullHouse,
        (3, _, 0) => HandType::ThreeOfAKind,
        (2, 1, 1) => HandType::ThreeOfAKind,
        (1, 0, 2) => HandType::ThreeOfAKind,
        (2, 2, 0) => HandType::TwoPair,
        (2, 1, 0) => HandType::OnePair,
        (1, 0, 1) => HandType::OnePair,
        _ => HandType::HighCard,
    }
}

fn comp_hands(h1: &str, h2: &str) -> Ordering {
    match hand_type(h1).cmp(&hand_type(h2)) {
        Ordering::Equal => h1
            .chars()
            .zip(h2.chars())
            .find_map(|(c1, c2)| match score_card(c1).cmp(&score_card(c2)) {
                Ordering::Equal => None,
                ord => Some(ord),
            })
            .unwrap(),
        ord => ord,
    }
}

fn get_cards(input: &str, joker: bool) -> Vec<(String, usize)> {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let (cards, bid) = l.split_once(' ').unwrap();
            if joker {
                (cards.replace("J", "j"), bid.parse().unwrap())
            } else {
                (cards.to_string(), bid.parse().unwrap())
            }
        })
        .collect()
}

fn winnings(cards: Vec<(String, usize)>) -> u32 {
    cards
        .iter()
        .enumerate()
        .map(|(rank, (_, bid))| ((rank + 1) * bid) as u32)
        .sum()
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut cards = get_cards(input, false);
    cards.sort_by(|h1, h2| comp_hands(h1.0.as_str(), h2.0.as_str()));
    Some(winnings(cards))
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut cards = get_cards(input, true);
    cards.sort_by(|h1, h2| comp_hands(h1.0.as_str(), h2.0.as_str()));
    Some(winnings(cards))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
