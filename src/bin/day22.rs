use std::collections::HashMap;

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
type ParsedItem = Brick;
type Pos = [usize; 3];
#[derive(Debug, PartialEq, Eq, Clone)]
struct Brick {
    start: Pos,
    end: Pos,
}
const X: usize = 0;
const Y: usize = 1;
const Z: usize = 2;
fn parse(input: &str) -> impl Iterator<Item = ParsedItem> + Clone + '_ {
    input.lines().map(|x| {
        let mut coords = ints(x);
        let start = coords
            .by_ref()
            .take(3)
            .collect::<Vec<_>>()
            .try_into()
            .expect("start coord");
        Brick {
            start,
            end: coords.collect::<Vec<_>>().try_into().expect("start coord"),
        }
    })
}
fn part1<I>(things: I) -> usize
where
    I: Iterator<Item = ParsedItem>,
{
    let mut bricks: Vec<_> = things.collect();
    fall(&mut bricks);
    42
}

fn is_support(idx: usize, bricks: &[Brick], bricks_end: &[usize]) -> bool {
    false
}

fn fall(bricks: &mut [Brick]) {
    bricks.sort_by(|a, b| a.start[Z].cmp(&b.start[Z]));
    let mut bricks_end: Vec<_> = (0..bricks.len()).collect();
    bricks_end.sort_by(|a, b| bricks[*a].end[Z].cmp(&bricks[*b].end[Z]));

    for idx in 0..bricks.len() {
        let space = may_fall(idx, bricks, &bricks_end);
        println!(
            "Brick {idx} {:?} has {space} empty spaces below it",
            &bricks[idx]
        );
        if space > 0 {
            bricks[idx].start[Z] -= space;
            bricks[idx].end[Z] -= space;
        }
        bricks_end.sort_by(|a, b| bricks[*a].end[Z].cmp(&bricks[*b].end[Z]));
    }
}
fn may_fall(idx: usize, bricks: &[Brick], bricks_end: &[usize]) -> usize {
    let b = &bricks[idx];
    let end_pos = bricks_end
        .binary_search(&idx)
        .expect("index not found in end sorted array");
    println!("brick {idx} is at {end_pos} in end_z array");
    if end_pos == 0 {
        return 0;
    }
    let mut min_space = usize::MAX;
    for below in (0..end_pos).rev() {
        let b2 = &bricks[bricks_end[below]];
        if b.overlap(b2) {
            println!("{b:?} overlaps {b2:?}");
            return 0;
        }
        println!();
        let mut b1 = b.clone();
        let mut space = 0;
        loop {
            if b1.start[Z] <= 1 {
                break;
            }
            b1.start[Z] -= 1;
            b1.end[Z] -= 1;
            if b1.overlap(b2) {
                println!("mod {b:?} overlaps {b2:?} after {space} spaces");
                break;
            } else {
                println!("mod {b1:?} does not overlap {b2:?} after {space} spaces");
            }
            space += 1;
        }
        println!("{b:?} does not overlap {b2:?}, and has {space} spaces below");
        if space < min_space {
            min_space = space;
        }
    }
    min_space
}
impl Brick {
    fn overlap(&self, b: &Brick) -> bool {
        let mut touch = [false; 3];
        (X..=Z).for_each(|coord| {
            let range_a = self.start[coord]..=self.end[coord];
            let range_b = b.start[coord]..=b.end[coord];
            if range_a.contains(&b.start[coord])
                || range_a.contains(&b.end[coord])
                || range_b.contains(&self.start[coord])
                || range_b.contains(&self.end[coord])
            {
                touch[coord] = true;
            }
        });
        touch.iter().all(|v| *v)
    }
}
#[test]
fn test_overlap() {
    // mod does not overlap after 2 spaces
    let b1 = Brick {
        start: [0, 1, 3],
        end: [2, 1, 3],
    };
    let b2 = Brick {
        start: [2, 0, 3],
        end: [2, 2, 3],
    };
    assert!(b1.overlap(&b2));
}

#[test]
fn test_fall() {
    let mut sample_bricks: Vec<_> = parse(sample!()).collect();
    let sample_bricks_fallen: Vec<_> = parse(concat!(
        "1,0,1~1,2,1\n", // A
        "0,0,2~2,0,2\n", // B
        "0,2,2~2,2,2\n", // C fell
        "0,0,3~0,2,3\n", // D fell
        "2,0,3~2,2,3\n", // E fell
        "0,1,4~2,1,4\n", // F fell
        "1,1,5~1,1,6",   // G
    ))
    .collect();
    fall(&mut sample_bricks);
    assert_eq!(sample_bricks_fallen, sample_bricks);
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
    assert_eq!(res, 42);
    //part 2
    let res = part2(things);
    assert_eq!(res, 42);
}
