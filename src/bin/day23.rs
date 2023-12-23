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

/*
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
*/
fn part2(map: MapRef) -> usize {
    let start_x = map[0].iter().position(|c| *c == b'.').expect("start pos");
    let end_x = map[map.len() - 1]
        .iter()
        .position(|c| *c == b'.')
        .expect("end pos");

    let map = map.to_vec();
    // Needs a bigger stack
    let child = thread::Builder::new()
        .stack_size(8 * 1024 * 1024) // this is mostly for debug mode with full input
        .spawn(move || {
            longest_path2_bruteforce(
                &map,
                (start_x, 0),
                (end_x, map.len() - 1),
                &mut vec![vec![false; map[0].len()]; map.len()],
                0,
            )
        })
        .unwrap();

    child.join().unwrap()
}

fn longest_path2_bruteforce(
    map: MapRef,
    pos: Pos,
    end: Pos,
    seen: &mut [Vec<bool>],
    cur: usize,
) -> usize {
    if pos == end {
        return cur;
    }
    let ipos = (pos.0 as isize, pos.1 as isize);
    let mut max = 0;
    for d in [(0, 1), (0, -1), (1, 0), (-1, 0)].into_iter() {
        let inext = (ipos.0 + d.0, ipos.1 + d.1);
        let next = (inext.0 as usize, inext.1 as usize);
        if inext.0 < 0 || next.0 >= map[0].len() || inext.1 < 0 || next.1 >= map.len() {
            continue;
        }
        if map[next.1][next.0] == b'#' {
            continue;
        }
        if seen[next.1][next.0] {
            continue;
        }
        seen[next.1][next.0] = true;
        let len = longest_path2_bruteforce(map, next, end, seen, cur + 1);
        seen[next.1][next.0] = false;
        if len > max {
            max = len;
        }
    }
    max
}

fn longest_path2(
    map: MapRef,
    pos: Pos,
    end: Pos,
    current_path: &mut HashSet<Pos>,
    maxes: &mut HashMap<Pos, (usize, usize)>,
) -> usize {
    println!("At pos {pos:?}, current length is {}", current_path.len());
    _print_map(map, current_path);
    if pos == end {
        return current_path.len();
    }
    let ipos = (pos.0 as isize, pos.1 as isize);
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
        let max = *maxes.entry(next).or_insert((0, current_path.len()));
        if max.1 > current_path.len() && max.0 > 0 {
            continue;
        }
        current_path.insert(next);
        let len = longest_path2(map, next, end, current_path, maxes);
        current_path.remove(&next);
        if len > max.0 {
            maxes.insert(pos, (len, current_path.len()));
        }
    }
    *maxes.values().map(|(m, _)| m).max().expect("a max")
}

fn _print_map(map: MapRef, current_path: &mut HashSet<Pos>) {
    map.iter().enumerate().for_each(|(y, l)| {
        l.iter().enumerate().for_each(|(x, c)| {
            if current_path.contains(&(x, y)) {
                print!("O")
            } else {
                print!("{}", *c as char)
            }
        });
        println!();
    });
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
}

#[cfg(not(feature = "ci_no_input"))]
#[test]
fn test_real_input() {
    let map = parse(input!());
    let res = part2(&map);
    assert_eq!(res, 6258);
}

#[test]
fn test_custom_map() {
    let map = parse(concat!(
        //start
        "#.###\n", ".....\n", //"#.#.#\n",
        // ".....\n",
        ".....\n", //end
        "###.#\n",
    ));
    assert_eq!(part1(&map), part2(&map));
}
