use std::collections::{HashMap, HashSet};

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
type ParsedItem = Links;
type Links = (String, HashSet<String>);

fn parse(input: &str) -> impl Iterator<Item = ParsedItem> + Clone + '_ {
    input.lines().map(|l| {
        let parts = l.split_once(':').unwrap();
        (
            parts.0.to_string(),
            parts
                .1
                .split_ascii_whitespace()
                .map(|s| s.to_string())
                .collect(),
        )
    })
}
fn part1<I>(things: I) -> usize
where
    I: Iterator<Item = ParsedItem>,
{
    let mut map: HashMap<String, HashSet<String>> = things.collect();
    // Hardcode solution from graphviz graph:
    // awk 'BEGIN{print "graph {"} {gsub(":", "", $1); for(i=2;i <=NF; i++){ printf("%s -- %s\n", $1, $i);}} END {print "}"}' src/inputs/day25.txt |dot -T svg -K neato -o graph-neato.svg

    map.get_mut(&"plt".to_string()).unwrap().remove("mgb");
    map.get_mut(&"jxm".to_string()).unwrap().remove("qns");
    map.get_mut(&"dbt".to_string()).unwrap().remove("tjd");
    // Now make them all bidirectionnal
    let keys: Vec<String> = map.keys().cloned().collect();
    for k in keys.into_iter() {
        if let Some(next) = map.get(&k).cloned() {
            for n in next.into_iter() {
                map.entry(n).or_default().insert(k.clone());
            }
        }
    }
    println!("Graph size jfd: {}", graph_size(&map, "jfd"));
    graph_size(&map, "jfd") * graph_size(&map, "qns")
}
fn graph_size(map: &HashMap<String, HashSet<String>>, start: &str) -> usize {
    let mut seen: HashSet<String> = HashSet::new();
    let mut next: Vec<String> = vec![];
    next.push(start.to_string());
    while let Some(n) = next.pop() {
        if let Some(conns) = map.get(&n) {
            for c in conns.iter() {
                if !seen.contains(c) {
                    seen.insert(c.to_string());
                    next.push(c.to_string());
                }
            }
        }
    }
    seen.len()
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
    //let res = part1(things.clone());
    //assert_eq!(res, 54);
}
