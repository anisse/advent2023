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
type MapRefMut<'a> = &'a mut [Vec<u8>];

#[derive(Debug, Clone)]
struct Pos {
    row: usize,
    col: usize,
}

fn parse(input: &str) -> Map {
    input.lines().map(|x| x.as_bytes().to_vec()).collect()
}
fn part1(map: MapRef) -> usize {
    let pos = Pos { row: 0, col: 0 };
    let dir = RIGHT;
    common_part(map, pos, dir)
}

const RIGHT: u8 = 0;
const DOWN: u8 = 1;
const LEFT: u8 = 2;
const UP: u8 = 3;

fn _print_seen(seen: MapRef) {
    seen.iter().for_each(|l| {
        l.iter().for_each(|c| print!("{c}"));
        println!();
    });
    println!();
}
fn visit_all(map: MapRef, seen: MapRefMut, pos: Pos, dir: u8) {
    if seen[pos.row][pos.col] & (1 << dir) != 0 {
        return;
    }
    /*
    println!("Now at {pos:?}, dir is {dir}");
    _print_seen(seen);
    */
    seen[pos.row][pos.col] |= 1 << dir;
    let current = map[pos.row][pos.col];
    let next: Vec<_> = match current {
        b'.' => vec![dir],
        b'/' | b'\\' => vec![next_dir(dir, current)],
        b'-' | b'|' => next_dirs(dir, current),
        _ => unreachable!(),
    };
    for dir in next.iter() {
        if let Some(next) = next_pos(pos.clone(), *dir, map) {
            visit_all(map, seen, next, *dir);
        }
    }
}
fn next_pos(pos: Pos, dir: u8, map: MapRef) -> Option<Pos> {
    let rows = map.len() as isize;
    let cols = map[0].len() as isize;
    let inc = [(0, 1), (1, 0), (0, -1), (-1, 0)][dir as usize];
    //println!("inc is {inc:?}, dir is {dir}");
    let new_row = pos.row as isize + inc.0;
    let new_col = pos.col as isize + inc.1;
    if new_row < 0 || new_row == rows || new_col < 0 || new_col == cols {
        return None;
    }
    Some(Pos {
        row: new_row as usize,
        col: new_col as usize,
    })
}

fn next_dir(dir: u8, c: u8) -> u8 {
    let inc_slash = [1, -1, 1, -1];
    ((dir as i8
        + match c {
            b'\\' => inc_slash[dir as usize],
            b'/' => -inc_slash[dir as usize],
            _ => unreachable!(),
        }
        + 4)
        % 4) as u8
}
fn next_dirs(dir: u8, c: u8) -> Vec<u8> {
    let split = vec![(dir + 1) % 4, (dir + 3) % 4];
    let res_split_dash = [vec![dir], split];
    match c {
        b'-' => res_split_dash[dir as usize % 2].clone(),
        b'|' => res_split_dash[(dir as usize + 1) % 2].clone(),
        _ => unreachable!(),
    }
}

fn common_part(map: MapRef, pos: Pos, dir: u8) -> usize {
    let mut seen_dir = vec![vec![0; map[0].len()]; map.len()];
    visit_all(map, &mut seen_dir, pos, dir);
    seen_dir
        .iter()
        .map(|l| l.iter().filter(|c| **c != 0).count())
        .sum()
}

fn part2(map: MapRef) -> usize {
    let rows = map.len();
    let cols = map[0].len();
    let max_r: usize = (0..rows)
        .map(|row| {
            [
                (LEFT, Pos { row, col: cols - 1 }),
                (RIGHT, Pos { row, col: 0 }),
            ]
            .iter()
            .map(|(dir, pos)| common_part(map, pos.clone(), *dir))
            .max()
            .unwrap()
        })
        .max()
        .unwrap();
    let max_c = (0..cols)
        .map(|col| {
            [
                (UP, Pos { row: rows - 1, col }),
                (DOWN, Pos { row: 0, col }),
            ]
            .iter()
            .map(|(dir, pos)| common_part(map, pos.clone(), *dir))
            .max()
            .unwrap()
        })
        .max()
        .unwrap();
    max_r.max(max_c)
}

#[test]
fn test() {
    let map = parse(sample!());
    //part 1
    let res = part1(&map);
    assert_eq!(res, 46);
    //part 2
    let res = part2(&map);
    assert_eq!(res, 51);
}
