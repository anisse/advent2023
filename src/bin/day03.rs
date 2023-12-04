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
            if let Some((i, l)) = integer(&line[x..], 0) {
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
                /*
                println!(
                    "part {} found at coord ({xx}, {yy}) next to int",
                    map[yy as usize][xx as usize],
                );
                */
                return true;
            }
        }
    }
    false
}

fn integer(s: &[char], i: usize) -> Option<(u16, usize)> {
    let mut start = i;
    if !s[i].is_ascii_digit() {
        return None;
    }
    while start > 0 && s[start - 1].is_ascii_digit() {
        start -= 1;
    }
    let mut end = i;
    while end < s.len() && s[end].is_ascii_digit() {
        end += 1
    }
    if start == end {
        return None;
    }
    //dbg!(&s[start..end]);
    return Some((
        s[start..end]
            .iter()
            .collect::<String>()
            .parse()
            .expect("not int"),
        end - i,
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
            //println!("At ({x}, {y}) char = {c}");
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
    let mut integers = vec![];
    for yy in (y as isize - 1)..=(y as isize + 1) {
        let mut xx = x as isize - 1;
        while xx <= x as isize + 1 {
            //println!("Start searching at {xx}, {yy} for gear ({x}, {y})");
            // out of bounds
            if yy < 0 || xx < 0 || yy as usize >= map.len() || xx as usize >= map[yy as usize].len()
            {
                xx += 1;
                continue;
            }
            // skip self
            if yy == y as isize && xx == x as isize {
                xx += 1;
                continue;
            }
            //println!("Searching at {xx}, {yy} for gear ({x}, {y})");
            if let Some((int, skip)) = integer(&map[yy as usize], xx as usize) {
                //println!("Found {int} at {xx}, {yy}; skipping {skip}");
                integers.push(int as usize);
                xx += skip as isize;
                //println!("part number {int} found at coord ({xx}, {yy}) next to gear({x}, {y})",);
            } else {
                xx += 1;
            }
        }
    }
    //dbg!(&integers);
    if integers.len() == 2 {
        /*
        println!(
            "gear at ({x}, {y}) has two adjacent integers {} and {}; product is {}",
            integers[0],
            integers[1],
            integers.iter().product::<usize>(),
        );
        */
        return Some(integers.iter().product());
    }
    None
}

#[test]
fn test() {
    let things = parse(sample!());
    //part 1
    let res = operation(things.clone());
    assert_eq!(res, 4361);
    //part 2
    let res = operation2(things);
    assert_eq!(res, 467835);
}
