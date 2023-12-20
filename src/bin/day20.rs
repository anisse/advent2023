use std::collections::HashMap;

use advent2023::*;
fn main() {
    let things = parse(input!());
    //part 1
    let res = part1(&things);
    println!("Part 1: {}", res);
    //part 2
    //let res = part2(things);
    //println!("Part 2: {}", res);
}

#[derive(Debug)]
struct Module<'a> {
    outputs: Vec<&'a str>,
    t: ModuleType,
}

#[derive(Debug)]
enum ModuleType {
    Start,
    FlipFlop,
    Conjunction { inputs: Vec<String> },
}

#[derive(Debug)]
enum State<'a> {
    FlipFlop(bool),
    Conjunction(HashMap<&'a str, bool>),
}
use ModuleType::*;

type ModuleMap<'a> = HashMap<&'a str, Module<'a>>;
type StateMap<'a> = HashMap<&'a str, State<'a>>;

type ParsedItem<'a> = Module<'a>;

fn parse(input: &str) -> ModuleMap {
    let mut map: ModuleMap = input
        .lines()
        .map(|l| {
            let t = match l.as_bytes()[0] {
                b'%' => FlipFlop,
                b'&' => Conjunction { inputs: vec![] },
                b'b' => Start,
                _ => unreachable!(),
            };
            let split = l.split_once(" -> ").expect("a ->");
            let name = &split.0[1..];
            let outputs = split.1.split(", ").collect();
            (name, Module { outputs, t })
        })
        .collect();
    let names: Vec<String> = map.keys().map(|s| s.to_string()).collect();
    names.iter().for_each(|name| {
        let outputs = map.get(&name as &str).unwrap().outputs.clone();
        for o in outputs {
            if let Some(m) = map.get_mut(o) {
                if let Conjunction { ref mut inputs } = m.t {
                    inputs.push(name.to_string())
                }
            }
        }
    });
    map
}
fn part1(things: &ModuleMap) -> usize {
    for t in things.iter() {
        dbg!(&t);
    }
    0
}

/*
fn part2<I>(things: I) -> usize
where
    I: Iterator<Item = ParsedItem>,
{
    for _ in things {
        todo!()
    }
    42
}

*/
#[test]
fn test() {
    let things = parse(sample!());
    //part 1
    let res = part1(&things);
    assert_eq!(res, 32000000);
    let sample2 = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";
    assert_eq!(part1(&parse(sample2)), 11687500);

    //part 2
    //let res = part2(things);
    //assert_eq!(res, 42);
}
