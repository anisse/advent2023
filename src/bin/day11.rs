use advent2023::*;
use itertools::Itertools;

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
    common(&map, 2)
}

type MapRef<'a> = &'a [Vec<char>];

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

    coords
        .iter()
        .tuple_combinations()
        .map(|(c1, c2)| {
            let (xmax, xmin, ymax, ymin) = (
                c1.0.max(c2.0),
                c1.0.min(c2.0),
                c1.1.max(c2.1),
                c1.1.min(c2.1),
            );
            let ydiff = ymax - ymin
                + expand_y
                    .iter()
                    .skip_while(|y| **y < ymin)
                    .take_while(|y| **y < ymax)
                    .count()
                    * (expand_factor - 1);
            let xdiff = xmax - xmin
                + expand_x
                    .iter()
                    .skip_while(|x| **x < xmin)
                    .take_while(|x| **x < xmax)
                    .count()
                    * (expand_factor - 1);
            ydiff + xdiff
        })
        .sum()
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
