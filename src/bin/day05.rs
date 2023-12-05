use std::ops::Range;

use advent2023::*;
fn main() {
    let (seeds, maps) = parse(input!());
    //part 1
    let res = operation(&seeds, &maps);
    println!("Part 1: {}", res);
    //part 2
    let res = operation2(&seeds, &maps);
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
fn operation(seeds: &[u64], maps: &[Map]) -> u64 {
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
        let before = range_before(seed_range, map_range);
        if !before.is_empty() {
            println!("Adding before range {before:?}");
            out.push(before)
        }
        let after = range_after(seed_range, map_range);
        if !after.is_empty() {
            println!("Adding after range {after:?}");
            out.push(after)
        }
        let overlap = range_overlap(seed_range, map_range);
        let new = (dest_start + (overlap.start - map_range.start))
            ..(dest_start + (overlap.end - map_range.start));
        println!("Adding converted range {overlap:?} -> {new:?}");
        return (Some(new), out);
    }
    println!("Keeping unmatched range {seed_range:?}");
    out.push(seed_range.clone());
    (None, out)
}
fn map_step(map: &Map, seed_ranges: &[Range<u64>]) -> Vec<Range<u64>> {
    let mut out = vec![];
    println!("===== New map step for ranges: {seed_ranges:?}");
    for r in seed_ranges.iter() {
        let mut remaining = vec![r.clone()];
        let mut next_remaining = vec![];
        for map_range in map.iter() {
            for r in remaining.iter() {
                let m_range = map_range[SRC]..(map_range[SRC] + map_range[LEN]);
                println!("Converting {r:?} to {m_range:?} -> {} ???", map_range[DEST]);
                let (next, keep) = map_step_range(r, &m_range, map_range[DEST]);
                if let Some(n) = next {
                    out.push(n)
                }
                next_remaining.extend_from_slice(&keep);
            }
            remaining = next_remaining;
            next_remaining = Vec::new();
        }
        out.extend_from_slice(&remaining);
    }
    out
}

fn operation2(seeds: &[u64], maps: &[Map]) -> u64 {
    seeds
        .chunks_exact(2)
        .map(|srange| srange[SEED_SRC]..(srange[SEED_SRC] + srange[SEED_LEN]))
        .map(|srange| vec![srange])
        .map(|range| {
            let mut input = range.clone();
            maps.iter().for_each(|m| input = map_step(m, &input));
            input
                .iter()
                .map(|r| r.start)
                .min()
                .expect("a range minimum")
        })
        .min()
        .expect("a minimum")
}

fn range_before(keep_range1: &Range<u64>, range2: &Range<u64>) -> Range<u64> {
    if keep_range1.start == range2.start {
        return keep_range1.start..range2.start;
    }
    if keep_range1.start < range2.start {
        return keep_range1.start..(range2.start);
    }
    0..0 // TODO: Option
}

fn range_after(keep_range1: &Range<u64>, range2: &Range<u64>) -> Range<u64> {
    if keep_range1.end == range2.end {
        return (keep_range1.end)..range2.end;
    }
    if keep_range1.end < range2.end {
        return 0..0; // TODO: Option
    }
    (range2.end)..keep_range1.end
}
fn range_overlap(range1: &Range<u64>, range2: &Range<u64>) -> Range<u64> {
    range1.start.max(range2.start)..range1.end.min(range2.end)
    //(range_before(range1, range2).end)..(range_after(range1, range2).start + 1)
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
    let res = operation(&seeds, &maps);
    assert_eq!(res, 35);
    //part 2
    let res = operation2(&seeds, &maps);
    assert_eq!(res, 46);
}
