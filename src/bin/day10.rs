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

fn is_inside(p: Pos, map: MapRef, edge_map: &[Vec<bool>]) -> bool {
    let mut intersections = 0;
    let mut left = 0;
    let mut right = 0;
    let y = p.1;
    for x in p.0..map[y].len() {
        if !edge_map[y][x] {
            continue;
        }
        match map[y][x] {
            'J' => left -= 1,
            'L' => left += 1,
            'F' => right += 1,
            '7' => right -= 1,
            '|' => intersections += 1,
            _ => {}
        }
        if right != 0 && right == -left {
            intersections += 1;
            right = 0;
            left = 0;
        }
    }
    intersections % 2 == 1
}

fn part2<I>(things: I) -> usize
where
    I: Iterator<Item = ParsedItem>,
{
    let mut map: Vec<_> = things.collect();
    let start = map
        .iter()
        .enumerate()
        .find_map(|(y, l)| {
            l.iter()
                .enumerate()
                .find_map(|(x, c)| if *c == 'S' { Some((x, y)) } else { None })
        })
        .expect("S");
    map[start.1][start.0] = 'F'; // TODO: remove hardcoded replacement
    let mut edge_map = vec![vec![false; map[0].len()]; map.len()];
    let mut prev = start;
    let mut current = (start.0 + 1, start.1); // hardcoded next //next_add(start, next[&map[start.1][start.0]][0]);
    loop {
        println!(
            "cur: {current:?}, size: {} {}",
            edge_map[0].len(),
            edge_map.len()
        );
        println!("{prev:?} -> {current:?} : {}", map[current.1][current.0]);
        edge_map[current.1][current.0] = true;
        if current == start {
            break;
        }
        let new = next_from(&map, prev, current);
        prev = current;
        current = new;
    }
    map.iter()
        .enumerate()
        .flat_map(|(y, l)| l.iter().enumerate().map(move |(x, c)| (*c, (x, y))))
        .filter(|(_, (x, y))| !edge_map[*y][*x])
        .inspect(|(c, p)| println!("{p:?}: {c} is {} inside", is_inside(*p, &map, &edge_map)))
        .filter(|(_, p)| is_inside(*p, &map, &edge_map))
        .count()
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
    for (i, t) in [
        (
            ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...",
            8,
        ),
        (
            "FF7S7F7F7F7F7F7F---7
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
        ),
    ]
    .iter()
    .enumerate()
    {
        let things = parse(t.0);
        let res = part2(things);
        assert_eq!(res, t.1, "test {i}");
        println!("=======================================");
        println!("=======================================");
        println!("=======================================");
        println!("=======================================");
        println!("=======================================");
        println!("=======================================");
        println!("test {i} is OK");
    }
}
