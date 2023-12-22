use std::collections::HashSet;

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
type ParsedItem = Brick;
type Pos = [usize; 3];
#[derive(Debug, PartialEq, Eq, Clone)]
struct Brick {
    start: Pos,
    end: Pos,
}
const X: usize = 0;
//const Y: usize = 1;
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
fn common_compress<I>(things: I) -> (Vec<Brick>, Vec<Vec<usize>>, Vec<Vec<usize>>)
where
    I: Iterator<Item = ParsedItem>,
{
    let mut bricks: Vec<_> = things.collect();
    fall(&mut bricks);
    let mut bricks_end: Vec<_> = (0..bricks.len()).collect();
    bricks_end.sort_by(|a, b| bricks[*a].end[Z].cmp(&bricks[*b].end[Z]));
    let supported_by: Vec<Vec<usize>> = (0..bricks.len())
        .map(|idx| {
            let b = &bricks[idx];
            let mut end_pos = bricks_end
                .iter()
                .position(|i| *i == idx)
                .expect("index not found in end sorted array");
            let mut supported_by = vec![];
            while end_pos > 0 {
                end_pos -= 1;
                let b1 = &bricks[bricks_end[end_pos]];
                //println!("Evaluating if {b1:?} supports {b:?}");
                if b1.end[Z] + 1 < b.start[Z] {
                    //   println!("Stopping, {b1:?} is too low");
                    break;
                }
                if b.supported_by(b1) {
                    supported_by.push(bricks_end[end_pos]);
                }
            }
            supported_by
        })
        .collect();
    let mut supports: Vec<Vec<usize>> = vec![vec![]; bricks.len()];
    supported_by.iter().enumerate().for_each(|(i, sby)| {
        //println!("Brick {i} is supported by {} bricks", sby.len());
        for s in sby.iter() {
            supports[*s].push(i);
        }
    });
    (bricks, supports, supported_by)
}
fn part1<I>(things: I) -> usize
where
    I: Iterator<Item = ParsedItem>,
{
    let (bricks, supports, supported_by) = common_compress(things);
    (0..bricks.len())
        .map(|b| {
            //println!("Brick {b} supports {} bricks", supports[b].len());
            supports[b].is_empty()
                || supports[b].iter().all(|s| {
                    /*
                    println!(
                        "Brick {b} supports brick {s} which is itself supported by {} bricks",
                        supported_by[*s].len()
                    );
                    */
                    supported_by[*s].len() > 1
                    // can be disintegrated
                })
        })
        .filter(|x| *x)
        .count()
}

fn fall(bricks: &mut [Brick]) {
    bricks.sort_by(|a, b| a.start[Z].cmp(&b.start[Z]));
    let mut bricks_end: Vec<_> = (0..bricks.len()).collect();
    //println!("bricks_end: {bricks_end:?}");
    bricks_end.sort_by(|a, b| bricks[*a].end[Z].cmp(&bricks[*b].end[Z]));
    //println!("bricks_end sorted: {bricks_end:?}");

    for idx in 0..bricks.len() {
        let space = may_fall(idx, bricks, &bricks_end);
        /*
        println!(
            "Brick {idx} {:?} has {space} empty spaces below it",
            &bricks[idx]
        );
        */
        if space > 0 {
            bricks[idx].start[Z] -= space;
            bricks[idx].end[Z] -= space;
        }
        bricks_end.sort_by(|a, b| bricks[*a].end[Z].cmp(&bricks[*b].end[Z]));
    }
}
fn may_fall(idx: usize, bricks: &[Brick], bricks_end: &[usize]) -> usize {
    let b = &bricks[idx];
    //println!("may fall {idx}, bricks_end: {bricks_end:?}");
    let end_pos = bricks_end
        .iter()
        .position(|i| *i == idx)
        .expect("index not found in end sorted array");
    //println!("brick {idx} is at {end_pos} in end_z array");
    if end_pos == 0 {
        return 0;
    }
    let mut min_space = usize::MAX;
    for below in (0..end_pos).rev() {
        let b2 = &bricks[bricks_end[below]];
        if b.overlap(b2) {
            //println!("{b:?} overlaps {b2:?}");
            return 0;
        }
        //println!();
        let mut b1 = b.clone();
        let mut space = 0;
        loop {
            if b1.start[Z] <= 1 {
                break;
            }
            b1.start[Z] -= 1;
            b1.end[Z] -= 1;
            if b1.overlap(b2) {
                //println!("mod {b:?} overlaps {b2:?} after {space} spaces");
                break;
            } else {
                //println!("mod {b1:?} does not overlap {b2:?} after {space} spaces");
            }
            space += 1;
        }
        //println!("{b:?} does not overlap {b2:?}, and has {space} spaces below");
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
    // Used to determine if self supports b
    fn supports(&self, b: &Brick) -> bool {
        !self.overlap(b) && {
            let mut b1 = self.clone();
            b1.end[Z] += 1;
            b1.overlap(b)
        }
    }
    // Used to determine if b supports self
    fn supported_by(&self, b: &Brick) -> bool {
        b.supports(self)
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
    let (bricks, supports, supported_by) = common_compress(things);
    (0..bricks.len())
        .map(|b| would_fall(b, &mut HashSet::new(), &supports, &supported_by))
        .sum()
}
fn would_fall(
    idx: usize,
    fallen: &mut HashSet<usize>,
    supports: &[Vec<usize>],
    supported_by: &[Vec<usize>],
) -> usize {
    fallen.insert(idx);
    'outer: for s in supports[idx].iter() {
        for sb in supported_by[*s].iter() {
            if !fallen.contains(sb) {
                continue 'outer;
            }
        }
        // s has fallen
        would_fall(*s, fallen, supports, supported_by);
    }
    fallen.len() - 1
}

#[test]
fn test() {
    let things = parse(sample!());
    //part 1
    let res = part1(things.clone());
    assert_eq!(res, 5);
    //part 2
    let res = part2(things);
    assert_eq!(res, 7);
}
