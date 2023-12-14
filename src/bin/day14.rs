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

fn part1<I>(things: I) -> usize
where
    I: Iterator<Item = ParsedItem>,
{
    let map: Vec<_> = things.collect();
    let rows = map.len();
    let cols = map[0].len();
    let mut sum = 0;
    for c in 0..cols {
        let mut first_rock_row: Option<_> = None;
        let mut rocks = 0;
        for r in 0..rows {
            if matches!(map[r][c], '.' | 'O') && first_rock_row.is_none() {
                first_rock_row = Some(r);
            }
            match map[r][c] {
                '.' => {}
                'O' => rocks += 1,
                '#' => {
                    /*
                    if let Some(first_row) = first_rock_row {
                        println!("at row {r}, col {c} accounting {rocks} rocks from row {first_row}, sum is now {sum}");
                    }
                    */
                    sum += account_rocks(&mut first_rock_row, &mut rocks, rows);
                }
                _ => unreachable!(),
            }
        }
        sum += account_rocks(&mut first_rock_row, &mut rocks, rows);
    }
    sum
}
fn account_rocks(first_rock_row: &mut Option<usize>, rocks: &mut usize, rows: usize) -> usize {
    let mut sum = 0;
    if let Some(mut first_row) = first_rock_row {
        while *rocks > 0 {
            sum += rows - first_row;
            first_row += 1;
            *rocks -= 1;
        }
        *first_rock_row = None;
    }
    *rocks = 0;
    sum
}

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
enum Dir {
    North = 0,
    West,
    South,
    East,
}

fn map_move_north(map: &mut [Vec<char>]) {
    let rows = map.len();
    let cols = map[0].len();
    for c in 0..cols {
        let mut first_empty_row: Option<_> = None;
        let mut moved_rocks = 0;
        for r in 0..rows {
            match map[r][c] {
                '.' => {
                    if first_empty_row.is_none() {
                        first_empty_row = Some(r);
                    }
                }
                'O' => {
                    if first_empty_row.is_some() {
                        // clear this rock, it will move
                        map[r][c] = '.';
                        moved_rocks += 1;
                    }
                }
                '#' => {
                    if let Some(first_row) = first_empty_row {
                        /*
                        println!(
                            "at row {r}, col {c} moving {moved_rocks} rocks from row {first_row}"
                        );
                        */
                    }
                    move_rocks_north(map, c, &mut first_empty_row, &mut moved_rocks);
                }
                _ => unreachable!(),
            }
        }
        move_rocks_north(map, c, &mut first_empty_row, &mut moved_rocks);
    }
}
fn map_move_dir(map: &mut [Vec<char>], dir: Dir) {
    let rows = map.len();
    let cols = map[0].len();
    for c in dir_first_range(dir, rows, cols) {
        let mut first_empty_row: Option<_> = None;
        let mut moved_rocks = 0;
        for r in dir_second_range(dir, rows, cols) {
            match dir_map_get(dir, map, r, c) {
                '.' => {
                    if first_empty_row.is_none() {
                        first_empty_row = Some(r);
                    }
                }
                'O' => {
                    if first_empty_row.is_some() {
                        // clear this rock, it will move
                        //map[r][c] = '.';
                        dir_map_set(dir, map, r, c, '.');
                        moved_rocks += 1;
                    }
                }
                '#' => {
                    /*
                    if let Some(first_row) = first_empty_row {
                        println!(
                            "Moving in dir {dir:?},  c1={r}, c2={c} moving {moved_rocks} rocks from c = {first_row}"
                        );
                    }
                    */
                    move_rocks_dir(dir, map, c, &mut first_empty_row, &mut moved_rocks);
                }
                _ => unreachable!(),
            }
        }
        move_rocks_dir(dir, map, c, &mut first_empty_row, &mut moved_rocks);
    }
}
fn dir_first_range<'a>(d: Dir, rows: usize, cols: usize) -> Box<dyn Iterator<Item = usize> + 'a> {
    match d {
        Dir::North => Box::new(0..cols),
        Dir::West => Box::new(0..rows),
        Dir::South => Box::new(0..cols),
        Dir::East => Box::new(0..rows),
    }
}
fn dir_second_range<'a>(d: Dir, rows: usize, cols: usize) -> Box<dyn Iterator<Item = usize> + 'a> {
    match d {
        Dir::North => Box::new(0..rows),
        Dir::West => Box::new(0..cols),
        Dir::South => Box::new((0..rows).rev()),
        Dir::East => Box::new((0..cols).rev()),
    }
}
fn dir_map_get(d: Dir, map: &[Vec<char>], c1: usize, c2: usize) -> char {
    match d {
        Dir::North | Dir::South => map[c1][c2],
        Dir::West | Dir::East => map[c2][c1],
    }
}
fn dir_map_set(d: Dir, map: &mut [Vec<char>], c1: usize, c2: usize, val: char) {
    match d {
        Dir::North | Dir::South => map[c1][c2] = val,
        Dir::West | Dir::East => map[c2][c1] = val,
    }
}
fn dir_map_inc(d: Dir) -> i8 {
    match d {
        Dir::North | Dir::West => 1,
        Dir::South | Dir::East => -1,
    }
}

fn move_rocks_north(
    map: &mut [Vec<char>],
    c: usize,
    first_rock_row: &mut Option<usize>,
    rocks: &mut usize,
) {
    if let Some(mut first_row) = first_rock_row {
        while *rocks > 0 {
            map[first_row][c] = 'O';
            first_row += 1;
            *rocks -= 1;
        }
        *first_rock_row = None;
    }
}
fn move_rocks_dir(
    dir: Dir,
    map: &mut [Vec<char>],
    c: usize,
    first_rock_row: &mut Option<usize>,
    rocks: &mut usize,
) {
    if let Some(mut first_row) = first_rock_row {
        while *rocks > 0 {
            dir_map_set(dir, map, first_row, c, 'O');
            first_row = (first_row as isize + dir_map_inc(dir) as isize) as usize;
            *rocks -= 1;
        }
        *first_rock_row = None;
    }
}

#[test]
fn test_move() {
    let map: Vec<_> = parse(
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
    let map2: Vec<_> = parse(
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
    let mut map_moved = map.clone();
    map_move_north(&mut map_moved);
    assert_eq!(map_moved, map2);
    let mut map_moved = map.clone();
    map_move_dir(&mut map_moved, Dir::North);
    assert_eq!(map_moved, map2);
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

fn one_cycle(map: &mut [Vec<char>]) {
    map_move_dir(map, Dir::North);
    //print_map(map);
    map_move_dir(map, Dir::West);
    //print_map(map);
    map_move_dir(map, Dir::South);
    //print_map(map);
    map_move_dir(map, Dir::East);
    print_map(map);
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
        one_cycle(&mut map);
        assert_eq!(map_result, map);
    }
}

fn print_map(map: &[Vec<char>]) {
    map.iter().for_each(|l| {
        l.iter().for_each(|c| print!("{c}"));
        println!();
    })
}

fn part2<I>(things: I) -> usize
where
    I: Iterator<Item = ParsedItem>,
{
    let mut map: Vec<_> = things.collect();
    let mut seq = vec![];
    for i in 0..1_000_000_000 {
        //let prev_map = map.clone();
        one_cycle(&mut map);
        /*
        if map == prev_map {
            println!("Stabilized after {i} cycles");
            return account_map(&map);
        }
        */
        seq.push(account_map(&map));
        if let Some((start, period)) = cycle_detect(&seq) {
            println!(
                "Cycle detected: starts at i={start} (val={}, period of {period})",
                seq[start]
            );
            if i % period == 0 {
                println!("cycle coucou");
            }
            return seq[start..][1_000_000_000 % period];
        }
        println!("At cycle {i}, got {} load", account_map(&map));
    }
    account_map(&map)
}
fn cycle_detect(seq: &[usize]) -> Option<(usize, usize)> {
    // basic floyd tortoise and hare implementation
    let mut tor = 0;
    let mut har = 0;
    loop {
        har += 2;
        tor += 1;
        if har >= seq.len() || tor >= seq.len() {
            return None;
        }
        if seq[har] == seq[tor] {
            break;
        }
    }
    let mut mu = 0;
    tor = 0;
    while seq[tor] != seq[har] {
        tor += 1;
        har += 1;
        mu += 1;
        if har >= seq.len() || tor >= seq.len() {
            return None;
        }
    }
    let mut lam = 1;
    har = tor + 1;
    while seq[tor] != seq[har] {
        if har >= seq.len() {
            return None;
        }
        har += 1;
        lam += 1;
    }
    Some((mu, lam))
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
