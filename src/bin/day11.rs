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
    input.lines().map(|x| x.chars().collect())
}
fn part1<I>(things: I) -> usize
where
    I: Iterator<Item = ParsedItem>,
{
    let map: Vec<_> = things.collect();
    let newmap = expand(&map);
    let coords: Vec<_> = newmap
        .iter()
        .enumerate()
        .flat_map(|(y, l)| {
            l.iter()
                .enumerate()
                .filter(|(_, c)| **c == '#')
                .map(move |(x, _)| (x, y))
        })
        .collect();

    let mut sum = 0;
    for i in 0..coords.len() {
        for j in (i + 1)..coords.len() {
            let c1 = coords[i];
            let c2 = coords[j];
            sum += c1.0.max(c2.0) - c1.0.min(c2.0) + (c1.1.max(c2.1) - c1.1.min(c2.1));
        }
    }
    sum
}

type MapRef<'a> = &'a [Vec<char>];

fn print_map(map: MapRef) {
    for l in map {
        for c in l {
            print!("{c}");
        }
        println!();
    }
}

fn expand(map: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut newmap = vec![];

    for y in 0..map.len() {
        newmap.push(map[y].clone());
        if map[y].iter().all(|c| *c == '.') {
            newmap.push(map[y].clone());
        }
    }
    let mut x = 0;
    while x < newmap[0].len() {
        if newmap.iter().all(|l| l[x] == '.') {
            for y in 0..newmap.len() {
                newmap[y].insert(x, '.')
            }
            x += 1;
        }
        x += 1;
    }
    newmap
}

#[test]
fn expand_test() {
    let map1: Vec<_> = parse(
        "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....",
    )
    .collect();
    let map2: Vec<_> = parse(
        "....#........
.........#...
#............
.............
.............
........#....
.#...........
............#
.............
.............
.........#...
#....#.......",
    )
    .collect();
    let expanded = expand(&map1);
    println!("Transforming:");
    print_map(&map1);
    println!("Into:");
    print_map(&expanded);
    print_map(&expanded);
    println!("Expected:");
    print_map(&map2);
    assert_eq!(expanded, map2);
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
    assert_eq!(res, 374);
    //part 2
    let res = part2(things);
    assert_eq!(res, 42);
}
