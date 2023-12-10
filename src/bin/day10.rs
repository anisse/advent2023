use std::collections::HashMap;

use advent2023::*;
fn main() {
    let things = parse(input!());
    //part 1
    let res = part1(things.clone());
    println!("Part 1: {}", res);
    //part 2
    let res = part2(things);
    println!("Part 2: {}", res);
}

type ParsedItem = Vec<char>;
type Map = Vec<ParsedItem>;
type MapRef<'a> = &'a [ParsedItem];
type Pos = (usize, usize);

fn parse(input: &str) -> impl Iterator<Item = ParsedItem> + Clone + '_ {
    input.lines().map(|x| x.chars().collect())
}

fn next_map() -> HashMap<char, [(i8, i8); 2]> {
    HashMap::from([
        ('F', [(1, 0), (0, 1)]),
        ('J', [(0, -1), (-1, 0)]),
        ('|', [(0, -1), (0, 1)]),
        ('7', [(-1, 0), (0, 1)]),
        ('L', [(0, -1), (1, 0)]),
        ('-', [(-1, 0), (1, 0)]),
    ])
}
fn next_from(map: MapRef, prev: Pos, start: Pos) -> Pos {
    let xdiff = (prev.0 as isize - start.0 as isize) as i8;
    let ydiff = (prev.1 as isize - start.1 as isize) as i8;
    let next = next_map()[&map[start.1][start.0]];
    for n in next.iter() {
        if *n != (xdiff, ydiff) {
            return next_add(start, *n);
        }
    }
    unreachable!();
}
fn next_add(start: Pos, diff: (i8, i8)) -> Pos {
    (
        (start.0 as isize + diff.0 as isize) as usize,
        (start.1 as isize + diff.1 as isize) as usize,
    )
}
fn part1<I>(things: I) -> usize
where
    I: Iterator<Item = ParsedItem>,
{
    let map: Vec<_> = things.collect();
    let start = map
        .iter()
        .enumerate()
        .find_map(|(y, l)| {
            l.iter()
                .enumerate()
                .find_map(|(x, c)| if *c == 'S' { Some((x, y)) } else { None })
        })
        .expect("S");
    let mut prev = start;
    let mut current = (start.0 + 1, start.1); // hardcoded next //next_add(start, next[&map[start.1][start.0]][0]);
    let mut i = 1;
    loop {
        println!("{prev:?} -> {current:?} : {}", map[current.1][current.0]);
        if current == start {
            break;
        }
        let new = next_from(&map, prev, current);
        prev = current;
        current = new;
        i += 1;
    }
    i / 2
}

fn part2<I>(things: I) -> usize
where
    I: Iterator<Item = ParsedItem>,
{
    for _ in things {
        todo!()
    }
    42
}

#[test]
fn test() {
    let things = parse(sample!());
    //part 1
    let res = part1(things.clone());
    assert_eq!(res, 42);
    //part 2
    let res = part2(things);
    assert_eq!(res, 42);
}
