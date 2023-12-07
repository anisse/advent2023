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
        .sorted_by(|(a, _), (b, _)| compare_hands(b, a))
        .inspect(|a| println!("{a:?}"))
        .enumerate()
        .map(|(i, (_, score))| score as usize * (i + 1))
        .sum()
}

//const CARD_ORDER: &str = "AKQJT98765432";
const CARD_ORDER: &str = "23456789TJQKA";
const CARD_ORDER2: &str = "J23456789TQKA";
fn order_dict() -> HashMap<char, usize> {
    CARD_ORDER
        .chars()
        .enumerate()
        .map(|(i, c)| (c, i))
        .collect()
}
fn order_dict2() -> HashMap<char, usize> {
    CARD_ORDER2
        .chars()
        .enumerate()
        .map(|(i, c)| (c, i))
        .collect()
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Kind {
    Five(char),
    Four(char),
    Full(char, char),
    Three(char),
    TwoPair(char, char),
    OnePair(char),
    High(char),
}
impl From<&Kind> for usize {
    fn from(val: &Kind) -> Self {
        match val {
            Kind::Five(_) => 0,
            Kind::Four(_) => 1,
            Kind::Full(_, _) => 2,
            Kind::Three(_) => 3,
            Kind::TwoPair(_, _) => 4,
            Kind::OnePair(_) => 5,
            Kind::High(_) => 6,
        }
    }
}
impl Ord for Kind {
    fn cmp(&self, other: &Self) -> Ordering {
        let s: usize = self.into();
        //if s != other.into() {
        return s.cmp(&other.into());
        //}
        /*
        let order = order_dict();
        match (self, other) {
            (Kind::Five(c1), Kind::Five(c2))
            | (Kind::Four(c1), Kind::Four(c2))
            | (Kind::Three(c1), Kind::Three(c2))
            | (Kind::OnePair(c1), Kind::OnePair(c2))
            | (Kind::High(c1), Kind::High(c2)) => order[c1].cmp(&order[c2]),
            (Kind::Full(s1, s2), Kind::Full(o1, o2))
            | (Kind::TwoPair(s1, s2), Kind::TwoPair(o1, o2)) => {
                println!("Comparing ({s1}, {s2}) with ({o1}, {o2})");
                if s1 != o1 {
                    return order[s1].cmp(&order[o1]);
                }
                if s2 != o2 {
                    return order[s2].cmp(&order[o2]);
                }
                Ordering::Equal
            }
            _ => unreachable!(),
        }
        */
    }
}

/*
#[test]
fn ord_correct() {
    assert_eq!(
        card_kind("KK677").cmp(&card_kind("KTJJT")),
        Ordering::Greater
    )
}
*/

impl PartialOrd for Kind {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn compare_hands(a: &String, b: &String) -> std::cmp::Ordering {
    let ka = card_kind(a);
    let kb = card_kind(b);
    let cmp = ka.cmp(&kb);
    let order = order_dict();
    if Ordering::Equal == cmp {
        println!(
            "{a} vs {b}: {ka:?} and {kb:?} are equal, second ordering is {:?}",
            second_order(&order, b, a)
        );
        return second_order(&order, b, a);
    }
    cmp
}

fn compare_hands2(a: &(String, Kind), b: &(String, Kind)) -> std::cmp::Ordering {
    let order = order_dict2();
    let cmp = a.1.cmp(&b.1);
    if Ordering::Equal == cmp {
        println!(
            "{a:?} and {b:?} are equal, second ordering is {:?}",
            second_order(&order, &b.0, &a.0)
        );
        return second_order(&order, &b.0, &a.0);
    }
    cmp
}

fn max(order: &HashMap<char, usize>, c1: char, c2: char) -> char {
    if order[&c1] < order[&c2] {
        return c1;
    }
    c2
}
fn min(order: &HashMap<char, usize>, c1: char, c2: char) -> char {
    if order[&c1] > order[&c2] {
        return c1;
    }
    c2
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
    let mut current_card = '0';
    let mut current_count = 0;
    let order = order_dict();
    for (i, (card, card_count)) in cards
        .iter()
        .sorted_by(|(_, c1), (_, c2)| c2.cmp(c1))
        .enumerate()
    {
        if i == 0 {
            current_card = *card;
            current_count = *card_count;
            continue;
        }
        if current_count == 3 && *card_count == 2 {
            return Kind::Full(current_card, *card);
        }
        if *card_count < current_count {
            break;
        }
        if current_count == 2 {
            return Kind::TwoPair(
                min(&order, current_card, *card),
                max(&order, current_card, *card),
            );
        }
        if order[&current_card] < order[card] && current_count != 2 {
            continue;
        }
        current_card = *card;
        current_count = *card_count;
    }
    match current_count {
        5 => Kind::Five(current_card),
        4 => Kind::Four(current_card),
        3 => Kind::Three(current_card),
        2 => Kind::OnePair(current_card),
        1 => Kind::High(current_card),
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
        .sorted_by(|(a, _), (b, _)| compare_hands2(b, a))
        .inspect(|a| println!("{a:?}"))
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
