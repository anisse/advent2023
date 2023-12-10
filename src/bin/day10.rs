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
//clockwise
fn right_vector(v: (i8, i8)) -> (i8, i8) {
    (v.1, -v.0)
}
//counter-clockwise
fn left_vector(v: (i8, i8)) -> (i8, i8) {
    (-v.1, v.0)
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
        //println!("{prev:?} -> {current:?} : {}", map[current.1][current.0]);
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

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Flood {
    Unknown,
    CountedDot,
    Edge,
    Ignored,
}

fn flood_from(map: MapRef, flood_map: &mut [Vec<Flood>], start: Pos) -> usize {
    let (x, y) = (start.0, start.1);
    if !matches!(flood_map[y][x], Flood::Unknown) {
        return 0;
    }
    let count;
    println!(
        "flooding at : {start:?}: {} {:?}",
        map[y][x], flood_map[y][x],
    );
    if map[start.1][start.0] == '.' {
        count = 1;
        flood_map[y][x] = Flood::CountedDot;
    } else {
        count = 0;
        flood_map[y][x] = Flood::Ignored;
    }
    count
        + [(-1, 0), (0, 1), (0, -1), (1, 0)]
            .iter()
            .filter(|(_, ydiff)| !(*ydiff == -1 && y == 0))
            .filter(|(xdiff, _)| !(*xdiff == -1 && x == 0))
            .filter(|(xdiff, _)| !(*xdiff == 1 && x == map[0].len() - 1))
            .filter(|(_, ydiff)| !(*ydiff == 1 && y == map.len() - 1))
            .map(|(xdiff, ydiff)| {
                (
                    (x as isize + *xdiff as isize) as usize,
                    (y as isize + *ydiff as isize) as usize,
                )
            })
            .map(|(x, y)| flood_from(map, flood_map, (x, y)))
            .sum::<usize>()
}

fn part2<I>(things: I) -> usize
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
    let mut counted_map = vec![vec![Flood::Unknown; map[0].len()]; map.len()];
    let mut prev = start;
    let mut current = (start.0 + 1, start.1); // hardcoded next //next_add(start, next[&map[start.1][start.0]][0]);
    loop {
        counted_map[current.1][current.0] = Flood::Edge;
        //println!("{prev:?} -> {current:?} : {}", map[current.1][current.0]);
        if current == start {
            break;
        }
        let new = next_from(&map, prev, current);
        prev = current;
        current = new;
    }
    let mut prev = start;
    let mut current = (start.0 + 1, start.1); // hardcoded next //next_add(start, next[&map[start.1][start.0]][0]);
    let mut count = 0;
    loop {
        println!(
            "Flooding from {prev:?} -> {current:?} : {}",
            map[current.1][current.0]
        );
        let xdiff = (prev.0 as isize - current.0 as isize) as i8;
        let ydiff = (prev.1 as isize - current.1 as isize) as i8;
        let right = right_vector((xdiff, ydiff));
        if ((current.0 > 0 && right.0 < 0)
            || (current.0 < map[0].len() - 1 && right.0 > 0)
            || right.0 == 0)
            && ((current.1 > 0 && right.1 < 0)
                || (current.1 < map.len() - 1 && right.1 > 0)
                || right.1 == 0)
        {
            println!("Starting flood at {current:?} + {right:?}");
            count += flood_from(
                &map,
                &mut counted_map,
                (
                    (current.0 as isize + right.0 as isize) as usize,
                    (current.1 as isize + right.1 as isize) as usize,
                ),
            );
        }
        if current == start {
            break;
        }
        let new = next_from(&map, prev, current);
        prev = current;
        current = new;
    }
    count
}

#[test]
fn test() {
    let things = parse(sample!());
    //part 1
    //let res = part1(things.clone());
    //assert_eq!(res, 42);
    //part 2
    let res = part2(things);
    assert_eq!(res, 4);
    for t in [(
        "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L",
        10,
    )]
    .iter()
    {
        let things = parse(t.0);
        let res = part2(things);
        assert_eq!(res, t.1);
    }
}
