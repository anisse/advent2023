use std::ops::Range;

use advent2023::*;
fn main() {
    let (seeds, maps) = parse(input!());
    //part 1
    let res = part1(&seeds, &maps);
    println!("Part 1: {}", res);
    //part 2
    let res = part2(&seeds, &maps);
    println!("Part 2: {}", res);
}
type Map = Vec<Vec<u64>>;

const DEST: usize = 0;
const SRC: usize = 1;
const LEN: usize = 2;

const SEED_SRC: usize = 0;
const SEED_LEN: usize = 1;

fn parse(input: &str) -> (Vec<u64>, Vec<Map>) {
    let seeds = input
        .lines()
        .next()
        .expect("first line")
        .split(':')
        .nth(1)
        .expect("second part")
        .split_ascii_whitespace()
        .map(|x| x.parse::<u64>().expect("not seed int"))
        .collect();
    (
        seeds,
        input
            .split("\n\n")
            .skip(1)
            .map(|m| {
                m.lines()
                    .skip(1)
                    .map(|l| {
                        l.split_ascii_whitespace()
                            .map(|x| x.parse::<u64>().expect("not seed int"))
                            .collect::<Vec<u64>>()
                    })
                    .collect::<Vec<Vec<u64>>>()
            })
            .collect(),
    )
}
fn part1(seeds: &[u64], maps: &[Map]) -> u64 {
    seeds
        .iter()
        .map(|s| {
            let mut id = *s;
            'outer: for m in maps.iter() {
                for r in m.iter() {
                    //println!("Seed {id} testing range {} {} {}", r[DEST], r[SRC], r[LEN]);
                    if r[SRC] <= id && r[SRC] + r[LEN] > id {
                        //print!("In range {} {} {}, {id} becomes ", r[DEST], r[SRC], r[LEN]);
                        id = r[DEST] + (id - r[SRC]);
                        //println!("{id}");
                        continue 'outer;
                    }
                }
            }
            id
        })
        .min()
        .expect("a minimum")
}

fn map_step_range(
    seed_range: &Range<u64>,
    map_range: &Range<u64>,
    dest_start: u64,
) -> (Option<Range<u64>>, Vec<Range<u64>>) {
    let mut out = vec![];
    if ranges_overlap(seed_range, map_range) {
        if let Some(before) = range_difference_before(seed_range, map_range) {
            //println!("Adding before range {before:?}");
            out.push(before)
        }
        if let Some(after) = range_difference_after(seed_range, map_range) {
            //println!("Adding after range {after:?}");
            out.push(after)
        }
        let overlap = range_overlap(seed_range, map_range);
        let new = (dest_start + (overlap.start - map_range.start))
            ..(dest_start + (overlap.end - map_range.start));
        //println!("Adding converted range {overlap:?} -> {new:?}");
        return (Some(new), out);
    }
    //println!("Keeping unmatched range {seed_range:?}");
    out.push(seed_range.clone());
    (None, out)
}
fn map_step(map: &Map, seed_ranges: &[Range<u64>]) -> Vec<Range<u64>> {
    //println!("===== New map step for ranges: {seed_ranges:?}");
    seed_ranges
        .iter()
        .flat_map(|r| {
            let mut out = vec![];
            let rem = map.iter().fold(vec![r.clone()], |remaining, map_range| {
                remaining
                    .iter()
                    .flat_map(|r| {
                        let m_range = map_range[SRC]..(map_range[SRC] + map_range[LEN]);
                        //println!("Converting {r:?} to {m_range:?} -> {} ???", map_range[DEST]);
                        let (next, keep) = map_step_range(r, &m_range, map_range[DEST]);
                        if let Some(n) = next {
                            out.push(n)
                        }
                        keep
                    })
                    .collect()
            });
            [out, rem]
        })
        .flatten()
        .collect()
}

fn part2(seeds: &[u64], maps: &[Map]) -> u64 {
    seeds
        .chunks_exact(2)
        .map(|srange| srange[SEED_SRC]..(srange[SEED_SRC] + srange[SEED_LEN]))
        .map(|srange| vec![srange])
        .map(|range| {
            maps.iter()
                .fold(range, |input, m| map_step(m, &input))
                .iter()
                .map(|r| r.start)
                .min()
                .expect("a range minimum")
        })
        .min()
        .expect("a minimum")
}

fn range_difference_before(a: &Range<u64>, b: &Range<u64>) -> Option<Range<u64>> {
    if a.start >= b.start {
        None
    } else {
        Some(a.start..(b.start))
    }
}

fn range_difference_after(a: &Range<u64>, b: &Range<u64>) -> Option<Range<u64>> {
    if a.end <= b.end {
        None
    } else {
        Some((b.end)..a.end)
    }
}
fn range_overlap(range1: &Range<u64>, range2: &Range<u64>) -> Range<u64> {
    range1.start.max(range2.start)..range1.end.min(range2.end)
}

fn ranges_overlap(range1: &Range<u64>, range2: &Range<u64>) -> bool {
    range1.contains(&range2.start)
        || range1.contains(&(range2.end - 1))
        || range2.contains(&range1.start)
        || range2.contains(&(range1.end - 1))
}

#[test]
fn test() {
    let (seeds, maps) = parse(sample!());
    //part 1
    let res = part1(&seeds, &maps);
    assert_eq!(res, 35);
    //part 2
    let res = part2(&seeds, &maps);
    assert_eq!(res, 46);
}
