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
        (0..rows).for_each(|r| {
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
        });
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

fn map_move_dir(map: &mut [Vec<char>], dir: Dir) {
    let rows = map.len();
    let cols = map[0].len();
    for i_vec1 in dir_vec1_range(dir, rows, cols) {
        let mut first_empty_j_vec2: Option<_> = None;
        let mut moved_rocks = 0;
        for j_vec2 in dir_vec2_range(dir, rows, cols) {
            match dir_map_get(dir, map, j_vec2, i_vec1) {
                '.' => {
                    if first_empty_j_vec2.is_none() {
                        first_empty_j_vec2 = Some(j_vec2);
                    }
                }
                'O' => {
                    if first_empty_j_vec2.is_some() {
                        // clear this rock, it will move
                        dir_map_set(dir, map, j_vec2, i_vec1, '.');
                        moved_rocks += 1;
                    }
                }
                '#' => {
                    /*
                    if let Some(first_j_vec2) = first_empty_j_vec2 {
                        println!(
                            "Moving in dir {dir:?}, i={i_vec1}, j={j_vec2} moving {moved_rocks} rocks from c = {first_j_vec2}"
                        );
                    }
                    */
                    move_rocks_dir(dir, map, i_vec1, &mut first_empty_j_vec2, &mut moved_rocks);
                }
                _ => unreachable!(),
            }
        }
        move_rocks_dir(dir, map, i_vec1, &mut first_empty_j_vec2, &mut moved_rocks);
    }
}
fn dir_vec1_range(d: Dir, rows: usize, cols: usize) -> impl Iterator<Item = usize> {
    match d {
        Dir::North => 0..cols,
        Dir::West => 0..rows,
        Dir::South => 0..cols,
        Dir::East => 0..rows,
    }
}
fn dir_vec2_range<'a>(d: Dir, rows: usize, cols: usize) -> Box<dyn Iterator<Item = usize> + 'a> {
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
fn dir_map_set(d: Dir, map: &mut [Vec<char>], j_vec2: usize, i_vec1: usize, val: char) {
    match d {
        Dir::North | Dir::South => map[j_vec2][i_vec1] = val,
        Dir::West | Dir::East => map[i_vec1][j_vec2] = val,
    }
}
fn dir_map_inc(d: Dir, coord: usize) -> usize {
    match d {
        Dir::North | Dir::West => coord + 1,
        Dir::South | Dir::East => coord - 1,
    }
}

fn move_rocks_dir(
    dir: Dir,
    map: &mut [Vec<char>],
    i_vec1: usize,
    first_empty_j_vec2: &mut Option<usize>,
    rocks: &mut usize,
) {
    if let Some(mut j_vec2) = first_empty_j_vec2 {
        while *rocks > 0 {
            dir_map_set(dir, map, j_vec2, i_vec1, 'O');
            j_vec2 = dir_map_inc(dir, j_vec2);
            *rocks -= 1;
        }
        *first_empty_j_vec2 = None;
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
    //_print_map(map);
    map_move_dir(map, Dir::West);
    //_print_map(map);
    map_move_dir(map, Dir::South);
    //_print_map(map);
    map_move_dir(map, Dir::East);
    //_print_map(map);
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

fn _print_map(map: &[Vec<char>]) {
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
    for _i in 0..1_000_000_000 {
        one_cycle(&mut map);
        seq.push(map.clone());
        if let Some((start, period)) = cycle_detect(&seq) {
            //println!("Cycle detected: starts at i={start} period of {period}",);
            return account_map(&seq[start..][(1_000_000_000 - start - 1) % period]);
        }
        //println!("At cycle {_i}, got {} load", account_map(&map));
    }
    account_map(&map)
}
// Returns Some(start, period) of detected cycle
fn cycle_detect<T>(seq: &[T]) -> Option<(usize, usize)>
where
    T: Eq,
{
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
