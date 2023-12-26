use itertools::Itertools;

const INPUT: &str = include_str!("../../inputs/day7.txt");

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum HandKind {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

static CARDS: &str = "J23456789TQKA";

fn main() {
    let sum: i64 = INPUT
        .lines()
        .map(|line| {
            let (hand, bid) = line.split_once(' ').unwrap();
            let bid: i64 = bid.parse().unwrap();
            let hand: Vec<usize> = hand.chars().map(|c| CARDS.find(c).unwrap()).collect();
            let mut count_map = hand.iter().copied().counts();
            let joker_count = count_map.remove(&0).unwrap_or_default();
            let mut counts: Vec<_> = count_map.into_values().sorted().collect();
            if counts.is_empty() {
                counts.push(0);
            }
            *counts.last_mut().unwrap() += joker_count;
            let hand_kind = match counts.as_slice() {
                [1, 1, 1, 1, 1] => HandKind::HighCard,
                [1, 1, 1, 2] => HandKind::OnePair,
                [1, 2, 2] => HandKind::TwoPair,
                [1, 1, 3] => HandKind::ThreeOfAKind,
                [2, 3] => HandKind::FullHouse,
                [1, 4] => HandKind::FourOfAKind,
                [5] => HandKind::FiveOfAKind,
                _ => unreachable!(),
            };
            (hand_kind, hand, bid)
        })
        .sorted()
        .enumerate()
        .map(|(rank, (_, _, bid))| bid * (rank as i64 + 1))
        .sum();

    println!("{}", sum);
}
