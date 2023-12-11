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

fn common(map: MapRef, expand_factor: usize) -> usize {
    let expand_y: Vec<_> = map
        .iter()
        .enumerate()
        .filter(|(_, l)| l.iter().all(|c| *c == '.'))
        .map(|(y, _)| y)
        .collect();
    let expand_x: Vec<_> = map[0]
        .iter()
        .enumerate()
        .filter(|(x, _)| map.iter().all(|l| l[*x] == '.'))
        .map(|(x, _)| x)
        .collect();
    let coords: Vec<_> = map
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
            let (xmax, xmin, ymax, ymin) = (
                c1.0.max(c2.0),
                c1.0.min(c2.0),
                c1.1.max(c2.1),
                c1.1.min(c2.1),
            );
            let mut ydiff = ymax - ymin;
            for y in expand_y.iter() {
                if *y < ymax && *y > ymin {
                    ydiff += expand_factor - 1;
                }
            }
            let mut xdiff = xmax - xmin;
            for x in expand_x.iter() {
                if *x < xmax && *x > xmin {
                    xdiff += expand_factor - 1;
                }
            }
            sum += ydiff + xdiff;
        }
    }
    sum
}
fn part2<I>(things: I) -> usize
where
    I: Iterator<Item = ParsedItem>,
{
    let map: Vec<_> = things.collect();
    common(&map, 1000000)
}

#[test]
fn test() {
    let things = parse(sample!());
    //part 1
    let res = part1(things.clone());
    assert_eq!(res, 374);
    //part 2
    let map: Vec<_> = things.collect();
    assert_eq!(common(&map, 2), 374);
    assert_eq!(common(&map, 10), 1030);
    assert_eq!(common(&map, 100), 8410);
}
