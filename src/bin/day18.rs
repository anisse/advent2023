use std::collections::HashMap;

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
type ParsedItem = Dig;
#[derive(Debug)]
struct Dig {
    dir: u8,
    len: u8,
    color: u32,
}
fn parse(input: &str) -> impl Iterator<Item = ParsedItem> + Clone + '_ {
    input.lines().map(|l| {
        let mut parts = l.split_ascii_whitespace();
        Dig {
            dir: parts.next().expect("dir").as_bytes()[0],
            len: parts.next().expect("len").as_bytes()[0] - b'0',
            color: 0,
        }
    })
}
fn part1<I>(things: I) -> usize
where
    I: Iterator<Item = ParsedItem>,
{
    let mut vertices: Vec<_> = things
        .scan((0_isize, 0_isize), |coord, op| {
            let mul = op.len as isize;
            let d = HashMap::from([
                (b'U', [0, -1]),
                (b'D', [0, 1]),
                (b'L', [-1, 0]),
                (b'R', [1, 0]),
            ]);
            let inc = d[&op.dir];
            *coord = (coord.0 + inc[0] * mul, coord.1 + inc[1] * mul);
            Some(*coord)
        })
        .collect();
    let (mut xmin, mut ymin) = vertices[0];
    let (mut xmax, mut ymax) = vertices[0];
    for v in vertices.iter().skip(1) {
        if xmin > v.0 {
            xmin = v.0;
        }
        if xmax < v.0 {
            xmax = v.0;
        }
        if ymin > v.1 {
            ymin = v.1;
        }
        if ymax < v.1 {
            ymax = v.1;
        }
    }
    let rows = (ymax - ymin) as usize + 1;
    let cols = (xmax - xmin) as usize + 1;
    let xoffset = 0 - xmin;
    let yoffset = 0 - ymin;

    let mut map = vec![vec![false; cols]; rows];
    vertices.push(vertices[0]);
    dbg!(&vertices);
    for i in 0..(vertices.len() - 1) {
        let rmin = vertices[i].1.min(vertices[i + 1].1);
        let rmax = vertices[i].1.max(vertices[i + 1].1);
        for r in rmin..(rmax + 1) {
            let cmin = vertices[i].0.min(vertices[i + 1].0);
            let cmax = vertices[i].0.max(vertices[i + 1].0);
            for c in cmin..(cmax + 1) {
                //println!("Tracing dot at ({c}, {r})");
                map[(r + yoffset) as usize][(c + xoffset) as usize] = true;
            }
        }
    }
    map.iter().for_each(|l| {
        l.iter()
            .for_each(|c| if *c { print!("#") } else { print!(".") });
        println!();
    });
    let edge: usize = map.iter().map(|l| l.iter().filter(|c| **c).count()).sum();
    let inside: usize = map
        .iter()
        .enumerate()
        .map(|(y, l)| {
            l.iter()
                .enumerate()
                .filter(|(x, edge)| {
                    !*edge && (*x..map[y].len()).map(|c| map[y][c]).filter(|c| *c).count() % 2 == 1
                })
                .count()
        })
        .sum();
    println!("Got {edge} edges and {inside} insides");
    edge + inside
    //shoelace(&vertices)
}

fn shoelace(vertices: &[(isize, isize)]) -> usize {
    let len = vertices.len();
    let mut sum1: isize = 0;
    let mut sum2: isize = 0;
    for i in 0..(len - 1) {
        sum1 += vertices[i].0 * vertices[i + 1].1;
        sum2 += vertices[i].1 * vertices[i + 1].0;
    }
    // first and last
    sum1 += vertices[len - 1].0 * vertices[0].1;
    sum2 += vertices[0].0 * vertices[len - 1].1;

    // Try to add edge ?
    let mut edge_len = 0;
    for i in 0..(len - 1) {
        edge_len += (vertices[i].0 - vertices[i + 1].0).abs()
            + (vertices[i].1 - vertices[i + 1].1).abs()
            - 1;
    }
    edge_len += (vertices[0].0 - vertices[len - 1].0).abs()
        + (vertices[0].1 - vertices[len - 1].1).abs()
        - 1;
    println!("Got {sum1} - {sum2}, len: {edge_len}");
    ((sum1 - sum2).abs() / 2) as usize + edge_len as usize
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
    assert_eq!(res, 62);
    //part 2
    //let res = part2(things);
    //assert_eq!(res, 42);
}
