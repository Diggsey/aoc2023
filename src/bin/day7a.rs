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

static CARDS: &str = "23456789TJQKA";

fn main() {
    let sum: i64 = INPUT
        .lines()
        .map(|line| {
            let (hand, bid) = line.split_once(' ').unwrap();
            let bid: i64 = bid.parse().unwrap();
            let hand: Vec<usize> = hand.chars().map(|c| CARDS.find(c).unwrap()).collect();
            let counts: Vec<_> = hand
                .iter()
                .copied()
                .counts()
                .into_values()
                .sorted()
                .collect();
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
