use advent2023::*;
use itertools::Itertools;
fn main() {
    let things = parse(input!());
    //part 1
    let res = part1(things.clone());
    println!("Part 1: {}", res);
    //part 2
    let res = part2(things);
    println!("Part 2: {}", res);
}
type ParsedItem = Vec<i64>;
fn parse(input: &str) -> impl Iterator<Item = ParsedItem> + Clone + '_ {
    input.lines().map(|l| ints(l).collect())
}
fn part1<I>(things: I) -> i64
where
    I: Iterator<Item = ParsedItem>,
{
    things
        /*
        .inspect(|x| {
            dbg!(&x);
        })
        */
        .map(|h| {
            let mut v = vec![];
            v.push(h);
            let mut x = &v[0];
            loop {
                let d = differences(x);
                if d.iter().all(|x| *x == 0) {
                    break;
                }
                v.push(d);
                x = &v[v.len() - 1];
            }
            v
        })
        /*
        .inspect(|x| {
            dbg!(&x);
        })
        */
        .map(|histories| {
            histories
                .iter()
                .rev()
                .map(|h| *h.iter().last().expect("last element"))
                //.inspect(|last| println!("{last}, "))
                .sum::<i64>()
        })
        //.inspect(|tot: i64| println!("total: {tot}"))
        .sum()
}

fn differences(v: &[i64]) -> Vec<i64> {
    v.iter().tuple_windows().map(|(a, b)| b - a).collect()
}

fn part2<I>(things: I) -> i64
where
    I: Iterator<Item = ParsedItem>,
{
    things
        /*
        .inspect(|x| {
            dbg!(&x);
        })
        */
        .map(|h| {
            let mut v = vec![];
            v.push(h);
            let mut x = &v[0];
            loop {
                let d = differences(x);
                if d.iter().all(|x| *x == 0) {
                    break;
                }
                v.push(d);
                x = &v[v.len() - 1];
            }
            v
        })
        /*
        .inspect(|x| {
            dbg!(&x);
        })
        */
        .map(|histories| {
            histories
                .iter()
                .rev()
                .map(|h| *h.iter().next().expect("first lement"))
                //.inspect(|first| println!("{first}, "))
                .fold(0, |prev, x| x - prev)
        })
        //.inspect(|tot| println!("total: {tot}"))
        .sum()
}

#[test]
fn test() {
    let things = parse(sample!());
    //part 1
    let res = part1(things.clone());
    assert_eq!(res, 114);
    //part 2
    let res = part2(things);
    assert_eq!(res, 2);
    let things = parse("0 3 6 9 12 15");
    let res = part2(things);
    assert_eq!(res, -3);
}
