use std::collections::{HashMap, VecDeque};

use advent2023::*;
fn main() {
    let things = parse(input!());
    //part 1
    //let res = part1(&things);
    //println!("Part 1: {}", res);
    //part 2
    let res = part2(&things);
    println!("Part 2: {}", res);
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
enum State {
    Start,
    FlipFlop(bool),
    Conjunction(HashMap<String, bool>),
}
use ModuleType::*;

type ModuleMap<'a> = HashMap<&'a str, Module<'a>>;
type StateMap<'a> = HashMap<&'a str, State>;

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
            let name = split
                .0
                .strip_prefix(|c| c == '%' || c == '&')
                .unwrap_or(split.0);
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
fn part1(modules: &ModuleMap) -> usize {
    let mut states = StateMap::new();
    modules.iter().for_each(|(name, module)| match &module.t {
        Start => {
            states.insert(name, State::Start);
        }
        FlipFlop => {
            states.insert(name, State::FlipFlop(false));
        }
        Conjunction { inputs } => {
            let mut h = HashMap::new();
            inputs.iter().for_each(|i| {
                h.insert(i.to_string(), false);
            });
            states.insert(name, State::Conjunction(h));
        }
    });

    let (mut high, mut low) = (0, 0);
    for _i in 0..1000 {
        let (h, l) = update_state("button", "broadcaster", false, modules, &mut states);
        if _i < 3 {
            println!("Got {h} highs and {l} low\n");
        }
        high += h;
        low += l; // account for button
    }
    high * low
}

fn update_state(
    source: &str,
    name: &str,
    high: bool,
    modules: &ModuleMap,
    states: &mut StateMap,
) -> (usize, usize) {
    let mut update_state_list: VecDeque<(String, String, bool)> = VecDeque::new();
    update_state_list.push_back((source.to_string(), name.to_string(), high));
    let (mut high_count, mut low_count) = (0, 0);
    while let Some((source, name, high)) = update_state_list.pop_front() {
        //println!("{source} -{}-> {name}", if high { "high" } else { "low" });
        if high {
            high_count += 1;
        } else {
            low_count += 1;
        }
        if states.get(&name as &str).is_none() {
            //println!("Reached output {name}");
            continue;
        }
        match states.get_mut(&name as &str).unwrap() {
            State::Start => {
                modules[&name as &str].outputs.iter().for_each(|o| {
                    update_state_list.push_back((name.clone(), o.to_string(), false))
                });
            }
            State::FlipFlop(ref mut up) => {
                if high {
                    continue;
                }
                *up = !*up;
                modules[&name as &str]
                    .outputs
                    .iter()
                    .for_each(|o| update_state_list.push_back((name.clone(), o.to_string(), *up)));
            }
            State::Conjunction(inputs) => {
                inputs.insert(source.to_string(), high);
                let send = !inputs.values().all(|i| *i);
                /*
                if name == "th" {
                    println!("Conjunction {name} is sending {send}");
                }
                */
                modules[&name as &str]
                    .outputs
                    .iter()
                    .for_each(|o| update_state_list.push_back((name.clone(), o.to_string(), send)));
            }
        }
    }
    (high_count, low_count)
}

fn part2(modules: &ModuleMap) -> usize {
    let mut states = StateMap::new();
    modules.iter().for_each(|(name, module)| match &module.t {
        Start => {
            states.insert(name, State::Start);
        }
        FlipFlop => {
            states.insert(name, State::FlipFlop(false));
        }
        Conjunction { inputs } => {
            let mut h = HashMap::new();
            inputs.iter().for_each(|i| {
                h.insert(i.to_string(), false);
            });
            states.insert(name, State::Conjunction(h));
        }
    });
    let mut conjunction_reverse_tree: HashMap<String, Vec<String>> = HashMap::new();
    modules.iter().for_each(|(name, module)| {
        if let Conjunction { inputs } = &module.t {
            conjunction_reverse_tree.insert(name.to_string(), inputs.clone());
        }
    });
    modules.iter().for_each(|(name, module)| {
        for o in &module.outputs {
            if states.get(o).is_none() {
                assert_eq!(o, &"rx");
                states.insert(o, State::FlipFlop(false));
                conjunction_reverse_tree.insert(o.to_string(), vec![name.to_string()]);
                return;
            }
        }
    });

    conjunction_reverse_tree
        .iter()
        .for_each(|(con, inputs)| println!("{con} accepts inputs from {inputs:?}"));
    #[derive(Debug)]
    struct ConjHist {
        count: bool,
        last_counted: bool,
        history: Vec<usize>,
    }
    let mut conjunction_history: HashMap<String, ConjHist> = HashMap::new();
    let mut queue = VecDeque::from(["rx".to_string()]);
    let mut current = false;
    while let Some(end) = queue.pop_front() {
        if conjunction_reverse_tree.get(&end).is_none() {
            continue;
        }
        for inp in conjunction_reverse_tree[&end].iter() {
            conjunction_history.insert(
                inp.clone(),
                ConjHist {
                    count: current,
                    last_counted: false,
                    history: vec![],
                },
            );
            queue.push_back(inp.to_string());
        }
        current = !current;
    }
    dbg!(&conjunction_history);

    let mut presses = 0;
    loop {
        presses += 1;
        update_state("button", "broadcaster", false, modules, &mut states);
        conjunction_history.iter_mut().for_each(|(name, cjh)| {
            if let State::Conjunction(ins) = &states[&name as &str] {
                let val = !ins.values().all(|i| *i);
                if name == "th" {
                    println!("th ins: {ins:?} out: {val} (expected: {})", cjh.count);
                }
                if val == cjh.count {
                    if cjh.last_counted {
                        *cjh.history.last_mut().unwrap() += 1;
                    } else {
                        cjh.history.push(1);
                        cjh.last_counted = true;
                    }
                } else {
                    cjh.last_counted = false;
                }
                if presses % 1000 == 0 {
                    dbg!(&cjh.history);
                }
                if let Some((start, period)) = cycle_detect(&cjh.history) {
                    println!("Detected cycle for {name}, starts at {start}, period of {period}");
                }
            }
        });

        if let State::FlipFlop(s) = states["rx"] {
            if s {
                break;
            }
        } else {
            unreachable!();
        }
    }
    presses
}
fn cycle_detect<T>(seq: &[T]) -> Option<(usize, usize)>
where
    T: Eq,
{
    // basic floyd tortoise and hare implementation
    let mut tor = 0;
    let mut har = 0;
    loop {
        har += 2;
        tor += 1;
        if har >= seq.len() || tor >= seq.len() {
            return None;
        }
        if seq[har] == seq[tor] {
            break;
        }
    }
    let mut mu = 0;
    tor = 0;
    while seq[tor] != seq[har] {
        tor += 1;
        har += 1;
        mu += 1;
        if har >= seq.len() || tor >= seq.len() {
            return None;
        }
    }
    let mut lam = 1;
    har = tor + 1;
    while seq[tor] != seq[har] {
        if har >= seq.len() {
            return None;
        }
        har += 1;
        lam += 1;
    }
    Some((mu, lam))
}

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
