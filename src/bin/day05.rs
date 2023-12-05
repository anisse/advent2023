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
type ParsedItem = Map;
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
                    println!("Seed {id} testing range {} {} {}", r[DEST], r[SRC], r[LEN]);
                    if r[SRC] <= id && r[SRC] + r[LEN] > id {
                        print!("In range {} {} {}, {id} becomes ", r[DEST], r[SRC], r[LEN]);
                        id = r[DEST] + (id - r[SRC]);
                        println!("{id}");
                        continue 'outer;
                    }
                }
            }
            id
        })
        .min()
        .expect("a minimum")
}

fn operation2(seeds: &[u64], maps: &[Map]) -> u64 {
    seeds
        .chunks_exact(2)
        .map(|srange| {
            (srange[SEED_SRC]..(srange[SEED_SRC] + srange[SEED_LEN]))
                .map(|s| {
                    let mut id = s;
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
        })
        .min()
        .expect("a minimum")
}

#[test]
fn test() {
    let (seeds, maps) = parse(sample!());
    //part 1
    let res = operation(&seeds, &maps);
    assert_eq!(res, 35);
    //part 2
    let res = operation2(&seeds, &maps);
    assert_eq!(res, 42);
}
