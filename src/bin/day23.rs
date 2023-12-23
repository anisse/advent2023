use std::{
    collections::{HashMap, HashSet},
    thread,
};

use advent2023::*;
fn main() {
    let map = parse(input!());
    //part 1
    let res = part1(&map);
    println!("Part 1: {}", res);
    //part 2
    let res = part2(&map);
    println!("Part 2: {}", res);
}
type Map = Vec<Vec<u8>>;
type MapRef<'a> = &'a [Vec<u8>];

type Pos = (usize, usize);

fn parse(input: &str) -> Map {
    input.lines().map(|x| x.as_bytes().to_vec()).collect()
}
fn part1(map: MapRef) -> usize {
    let start_x = map[0].iter().position(|c| *c == b'.').expect("start pos");
    let end_x = map[map.len() - 1]
        .iter()
        .position(|c| *c == b'.')
        .expect("end pos");
    longest_path(
        map,
        (start_x, 0),
        (end_x, map.len() - 1),
        &mut HashSet::new(),
    )
}

fn longest_path(map: MapRef, pos: Pos, end: Pos, current_path: &mut HashSet<Pos>) -> usize {
    if pos == end {
        return current_path.len();
    }
    let ipos = (pos.0 as isize, pos.1 as isize);
    let mut max = 0;
    for d in [(0, 1), (0, -1), (1, 0), (-1, 0)].into_iter() {
        let inext = (ipos.0 + d.0, ipos.1 + d.1);
        let next = (inext.0 as usize, inext.1 as usize);
        if inext.0 < 0 || next.0 >= map[0].len() || inext.1 < 0 || next.1 >= map.len() {
            continue;
        }
        match (map[next.1][next.0], d) {
            (b'.', _) => {}
            (b'v', (0, 1)) => {}
            (b'>', (1, 0)) => {}
            (b'^', (0, -1)) => {}
            (b'<', (-1, 0)) => {}
            _ => continue,
        }
        if current_path.contains(&next) {
            continue;
        }
        current_path.insert(next);
        let len = longest_path(map, next, end, current_path);
        current_path.remove(&next);
        if len > max {
            max = len;
        }
    }
    max
}

fn part2(map: MapRef) -> usize {
    let start_x = map[0].iter().position(|c| *c == b'.').expect("start pos");
    let end_x = map[map.len() - 1]
        .iter()
        .position(|c| *c == b'.')
        .expect("end pos");

    let map2 = map.to_vec();
    // Needs a bigger stack
    let child = thread::Builder::new()
        .stack_size(32 * 1024 * 1024)
        .spawn(move || {
            longest_path2(
                &map2,
                (start_x, 0),
                (end_x, map2.len() - 1),
                &mut HashSet::new(),
                &mut HashMap::new(),
            )
        })
        .unwrap();

    child.join().unwrap()
}

fn longest_path2(
    map: MapRef,
    pos: Pos,
    end: Pos,
    current_path: &mut HashSet<Pos>,
    maxes: &mut HashMap<Pos, usize>,
) -> usize {
    if pos == end {
        return current_path.len();
    }
    let ipos = (pos.0 as isize, pos.1 as isize);
    let mut max_current = 0;
    for d in [(0, 1), (0, -1), (1, 0), (-1, 0)].into_iter() {
        let inext = (ipos.0 + d.0, ipos.1 + d.1);
        let next = (inext.0 as usize, inext.1 as usize);
        if inext.0 < 0 || next.0 >= map[0].len() || inext.1 < 0 || next.1 >= map.len() {
            continue;
        }
        if map[next.1][next.0] == b'#' {
            continue;
        }
        if current_path.contains(&next) {
            continue;
        }
        let max = *maxes.entry(next).or_insert(0);
        if max > current_path.len() {
            continue;
        }
        current_path.insert(next);
        let len = longest_path2(map, next, end, current_path, maxes);
        current_path.remove(&next);
        if len > max {
            maxes.insert(next, len);
        }
        if len > max_current {
            max_current = len;
        }
    }
    if *maxes.entry(pos).or_insert(0) < max_current {
        maxes.insert(pos, max_current);
    }
    maxes[&pos]
}

#[test]
fn test() {
    let map = parse(sample!());
    //part 1
    let res = part1(&map);
    assert_eq!(res, 94);
    //part 2
    let res = part2(&map);
    assert_eq!(res, 154);
    let map = parse(input!());
    let res = part2(&map);
    assert_eq!(res, 6258);
}
