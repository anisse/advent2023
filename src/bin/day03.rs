use std::collections::HashMap;

use advent2023::*;
fn main() {
    let things = parse(input!());
    //part 1
    let res = operation(things.clone());
    println!("Part 1: {}", res);
    //part 2
    let res = operation2(things);
    println!("Part 2: {}", res);
}
type ParsedItem = Vec<char>;
fn parse(input: &str) -> impl Iterator<Item = ParsedItem> + Clone + '_ {
    input.lines().map(|x| x.chars().collect())
}

fn operation<I>(things: I) -> usize
where
    I: Iterator<Item = ParsedItem>,
{
    let mut sum = 0;
    let map: Vec<_> = things.collect();
    for (y, line) in map.iter().enumerate() {
        let mut x = 0;
        while x < line.len() {
            if let Some((i, l)) = integer(&line[x..]) {
                if is_adjacent_int(&map, x, y, l) {
                    sum += i as usize;
                }
                x += l;
            }
            x += 1;
        }
    }
    sum
}

fn is_adjacent_int(map: &[Vec<char>], x: usize, y: usize, lx: usize) -> bool {
    let mut yy: isize;
    let mut xx: isize;
    for yy in (y as isize - 1)..=(y as isize + 1) {
        for xx in (x as isize - 1)..=((x + lx) as isize) {
            // out of bounds
            if yy < 0 || xx < 0 || yy as usize >= map.len() || xx as usize >= map[yy as usize].len()
            {
                continue;
            }
            // skip number
            if yy == y as isize && ((x as isize)..((x + lx) as isize)).contains(&xx) {
                continue;
            }
            if map[yy as usize][xx as usize] != '.' {
                println!(
                    "part {} found at coord ({xx}, {yy}) next to int",
                    map[yy as usize][xx as usize],
                );
                return true;
            }
        }
    }
    false
}

fn integer(s: &[char]) -> Option<(u16, usize)> {
    let mut i = 0;
    while i < s.len() {
        if s[i].is_ascii_digit() {
            i += 1
        } else {
            break;
        }
    }
    if i == 0 {
        return None;
    }
    //dbg!(&s[..i]);
    return Some((
        s[..i].iter().collect::<String>().parse().expect("not int"),
        i,
    ));
}

fn operation2<I>(things: I) -> usize
where
    I: Iterator<Item = ParsedItem>,
{
    let mut sum = 0;
    let map: Vec<_> = things.collect();
    for (y, line) in map.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c == '*' {
                if let Some(p) = is_adjacent_gear(&map, x, y) {
                    sum += p;
                }
            }
        }
    }
    sum
}

fn is_adjacent_gear(map: &[Vec<char>], x: usize, y: usize) -> Option<usize> {
    let mut integers: HashMap<(isize, isize), usize> = HashMap::new();
    for yy in (y as isize - 1)..=(y as isize + 1) {
        for xx in (x as isize - 1)..=(x as isize + 1) {
            // out of bounds
            if yy < 0 || xx < 0 || yy as usize >= map.len() || xx as usize >= map[yy as usize].len()
            {
                continue;
            }
            // skip self
            if yy == y as isize && yy == x as isize {
                continue;
            }
            if let Some((i, _)) = integer(&map[yy as usize][(xx as usize)..]) {
                let mut intcoord = xx - 1;
                let mut int = i as usize;
                if intcoord >= 0 {
                    while let Some((i, _)) = integer(&map[yy as usize][(intcoord as usize)..]) {
                        intcoord -= 1;
                        int = i as usize;
                        if intcoord < 0 {
                            break;
                        }
                    }
                }

                integers.insert((intcoord, yy), int);
                println!(
                    "part number {int} found at coord ({intcoord}, {yy}) next to gear({x}, {y})",
                );
            }
        }
    }
    if integers.len() == 2 {
        /*
        println!(
            "gear at ({x}, {y}) has two adjacent integers {} and {}",
            integers[0], integers[1],
        );
        */
        return Some(integers.values().product());
    }
    None
}

#[test]
fn test() {
    let things = parse(sample!());
    //part 1
    let res = operation(things.clone());
    assert_eq!(res, 42);
    //part 2
    let res = operation2(things);
    assert_eq!(res, 42);
}
