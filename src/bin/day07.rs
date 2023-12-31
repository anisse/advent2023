use advent2023::*;
use itertools::Itertools;
use std::{cmp::Ordering, collections::HashMap};

fn main() {
    let things = parse(input!());
    //part 1
    let res = part1(things.clone());
    println!("Part 1: {}", res);
    //part 2
    let res = part2(things);
    println!("Part 2: {}", res);
}
type ParsedItem = (String, u16);

fn parse(input: &str) -> impl Iterator<Item = ParsedItem> + Clone + '_ {
    input.lines().map(|x| {
        let mut comps = x.split_ascii_whitespace();
        (
            comps.next().expect("first el").to_string(),
            comps
                .next()
                .expect("next el")
                .parse::<u16>()
                .expect("not int"),
        )
    })
}

fn part1<I>(hands: I) -> usize
where
    I: Iterator<Item = ParsedItem>,
{
    hands
        .map(|(a, score)| {
            let k = card_kind(&a);
            ((a, k), score)
        })
        .sorted_by(|(a, _), (b, _)| compare_hands(CARD_ORDER, b, a))
        //.inspect(|a| println!("{a:?}"))
        .enumerate()
        .map(|(i, (_, score))| score as usize * (i + 1))
        .sum()
}

const CARD_ORDER: &str = "23456789TJQKA";
const CARD_ORDER_J: &str = "J23456789TQKA";
fn order_map(order: &str) -> HashMap<char, usize> {
    order.chars().enumerate().map(|(i, c)| (c, i)).collect()
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(usize)]
enum Kind {
    Five,
    Four,
    Full,
    Three,
    TwoPair,
    OnePair,
    High,
}
impl From<&Kind> for usize {
    fn from(val: &Kind) -> Self {
        *val as usize
    }
}

fn compare_hands(order: &str, a: &(String, Kind), b: &(String, Kind)) -> std::cmp::Ordering {
    if a.1 != b.1 {
        return a.1.cmp(&b.1);
    }
    /*
    println!(
        "{a:?} and {b:?} are equal, second ordering is {:?}",
        second_order(&order, &b.0, &a.0)
    );
    */
    second_order(&order_map(order), &b.0, &a.0)
}

fn second_order(order: &HashMap<char, usize>, a: &str, b: &str) -> std::cmp::Ordering {
    for (a, b) in a.chars().zip(b.chars()) {
        if order[&a] != order[&b] {
            return order[&a].cmp(&order[&b]);
        }
    }
    Ordering::Equal
}

fn card_kind(s: &str) -> Kind {
    let cards = s.chars().counts();
    let mut current_count = 0;
    // TODO. use reduce instead
    for (i, card_count) in cards
        .values()
        .sorted_unstable_by(|c1, c2| c2.cmp(c1)) //reverse order
        .enumerate()
    {
        if i == 0 {
            current_count = *card_count;
            continue;
        }
        if current_count == 3 && *card_count == 2 {
            return Kind::Full;
        }
        if *card_count < current_count {
            break;
        }
        if current_count == 2 {
            return Kind::TwoPair;
        }
        current_count = *card_count;
    }
    match current_count {
        5 => Kind::Five,
        4 => Kind::Four,
        3 => Kind::Three,
        2 => Kind::OnePair,
        1 => Kind::High,
        _ => unreachable!(),
    }
}

fn highest_kind(s: &str) -> Kind {
    CARD_ORDER
        .chars()
        .map(|j| card_kind(&s.replace('J', &j.to_string())))
        .min()
        .expect("a max")
}

fn part2<I>(hands: I) -> usize
where
    I: Iterator<Item = ParsedItem>,
{
    hands
        .map(|(a, score)| {
            let k = highest_kind(&a);
            ((a, k), score)
        })
        .sorted_by(|(a, _), (b, _)| compare_hands(CARD_ORDER_J, b, a))
        //.inspect(|a| println!("{a:?}"))
        .enumerate()
        .map(|(i, (_, score))| score as usize * (i + 1))
        .sum()
}

#[test]
fn test() {
    let things = parse(sample!());
    //part 1
    let res = part1(things.clone());
    assert_eq!(res, 6440);
    //part 2
    let res = part2(things);
    assert_eq!(res, 5905);
}
