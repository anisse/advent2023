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
fn parse(input: &str) -> impl Iterator<Item = ParsedItem> + Clone + '_ {
    input.lines().map(|l| l.chars().collect())
}

fn transpose(map: &[Vec<char>]) -> Vec<Vec<char>> {
    let rows = map.len();
    let cols = map[0].len();
    (0..cols)
        .map(|c| (0..rows).map(|r| map[r][c]).collect())
        .collect()
}
fn flip_horizontal_in_place(map: &mut [Vec<char>]) {
    let rows = map.len();
    let cols = map[0].len();
    (0..rows).for_each(|r| {
        (0..cols / 2)
            .for_each(|c| (map[r][c], map[r][cols - c - 1]) = (map[r][cols - c - 1], map[r][c]))
    })
}
fn sort_split_in_place(map: &mut [Vec<char>]) {
    map.iter_mut().for_each(|l| {
        l.split_mut(|c| *c == '#')
            .for_each(|s| s.sort_by(|a, b| b.cmp(a)));
    });
}

fn part1<I>(things: I) -> usize
where
    I: Iterator<Item = ParsedItem>,
{
    let map: Vec<_> = things.collect();
    let mut map = transpose(&map);
    sort_split_in_place(&mut map);
    let map = transpose(&map);

    account_map(&map)
}

fn part2<I>(things: I) -> usize
where
    I: Iterator<Item = ParsedItem>,
{
    let mut map: Vec<_> = things.collect();
    let mut seen = HashMap::new();
    let mut history: Vec<Vec<Vec<char>>> = vec![];
    let mut i = 0;
    loop {
        map = one_cycle(map);
        if let Some(start) = seen.get(&map) {
            //_print_map(&map);
            let period = i - start;
            //println!("Cycle detected: starts at {start} period of {period} i={i}",);
            return account_map(&history[*start..][(1_000_000_000 - start - 1) % period]);
        }
        seen.insert(map.clone(), i);
        history.push(map.clone());
        i += 1;
    }
}
fn one_cycle(mut map: Vec<Vec<char>>) -> Vec<Vec<char>> {
    for _ in 0..4 {
        map = transpose(&map);
        sort_split_in_place(&mut map);
        //println!("Before flip:");
        flip_horizontal_in_place(&mut map);
        //println!("After flip:");
    }
    map
}
fn account_map(map: &[Vec<char>]) -> usize {
    let rows = map.len();
    map.iter()
        .enumerate()
        .map(|(r, l)| {
            l.iter()
                .filter(|c| **c == 'O')
                .map(|_| rows - r)
                .sum::<usize>()
        })
        .sum()
}

#[test]
fn test_accounting() {
    let map: Vec<_> = parse(
        "OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#....",
    )
    .collect();
    assert_eq!(account_map(&map), 136);
    let map: Vec<_> = parse("OOOO").collect();
    assert_eq!(account_map(&map), 4);
    let map: Vec<_> = parse(
        "OOOO
....",
    )
    .collect();
    assert_eq!(account_map(&map), 8);
}

#[test]
fn test_one_cycle() {
    let expected = [
        ".....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#....",
        ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#..OO###..
#.OOO#...O",
        ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O",
    ];
    let mut map: Vec<_> = parse(
        "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....",
    )
    .collect();
    for result in expected.iter() {
        let map_result: Vec<_> = parse(result).collect();
        map = one_cycle(map);
        println!("Expected:");
        _print_map(&map_result);
        println!("Got:");
        _print_map(&map);
        assert_eq!(map_result, map);
    }
}

fn _print_map(map: &[Vec<char>]) {
    map.iter().for_each(|l| {
        l.iter().for_each(|c| print!("{c}"));
        println!();
    })
}

#[test]
fn test() {
    let things = parse(sample!());
    //part 1
    let res = part1(things.clone());
    assert_eq!(res, 136);
    //part 2
    let res = part2(things);
    assert_eq!(res, 64);
}
