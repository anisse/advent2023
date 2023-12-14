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
                    if let Some(first_row) = first_rock_row {
                        println!("at row {r}, col {c} accounting {rocks} rocks from row {first_row}, sum is now {sum}");
                    }
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
                        println!(
                            "at row {r}, col {c} moving {moved_rocks} rocks from row {first_row}"
                        );
                    }
                    move_rocks(map, c, &mut first_empty_row, &mut moved_rocks, rows);
                }
                _ => unreachable!(),
            }
        }
        move_rocks(map, c, &mut first_empty_row, &mut moved_rocks, rows);
    }
}

fn move_rocks(
    map: &mut [Vec<char>],
    c: usize,
    first_rock_row: &mut Option<usize>,
    rocks: &mut usize,
    rows: usize,
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

#[test]
fn test_move() {
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
    map_move_north(&mut map);
    assert_eq!(map, map2);
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
    assert_eq!(res, 136);
    //part 2
    /*
    let res = part2(things);
    assert_eq!(res, 42);
    */
}
