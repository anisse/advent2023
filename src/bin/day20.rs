use std::collections::{HashMap, VecDeque};

use advent2023::*;
fn main() {
    let things = parse(input!());
    //part 1
    let res = part1(&things);
    println!("Part 1: {}", res);
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
    let mut states = state_init(modules);

    let (mut high, mut low) = (0, 0);
    for _i in 0..1000 {
        let (h, l) = update_state(
            "button",
            "broadcaster",
            false,
            modules,
            &mut states,
            &mut HashMap::new(),
        );
        high += h;
        low += l;
    }
    high * low
}

fn state_init<'a>(modules: &ModuleMap<'a>) -> StateMap<'a> {
    modules
        .iter()
        .map(|(name, module)| match &module.t {
            Start => (*name, State::Start),
            FlipFlop => (*name, State::FlipFlop(false)),
            Conjunction { inputs } => {
                let mut h = HashMap::new();
                inputs.iter().for_each(|i| {
                    h.insert(i.to_string(), false);
                });
                (*name, State::Conjunction(h))
            }
        })
        .collect()
}

fn update_state(
    source: &str,
    name: &str,
    high: bool,
    modules: &ModuleMap,
    states: &mut StateMap,
    watch_list: &mut HashMap<String, bool>,
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
                if send {
                    if let Some(v) = watch_list.get_mut(&name) {
                        *v = true;
                    }
                }
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
    let mut states = state_init(modules);
    let mut rx_input = String::new();
    modules.iter().for_each(|(name, module)| {
        for o in &module.outputs {
            if states.get(o).is_none() {
                assert_eq!(o, &"rx");
                states.insert(o, State::FlipFlop(false));
                rx_input = name.to_string();
                return;
            }
        }
    });
    assert!(!rx_input.is_empty());

    let mut lb_input_was_up: HashMap<String, bool> =
        if let Conjunction { inputs } = &modules[&rx_input as &str].t {
            inputs.iter().map(|i| (i.clone(), false)).collect()
        } else {
            unreachable!()
        };
    let mut lb_input_first_up_cycle: HashMap<String, usize> = HashMap::new();
    let mut presses = 0;
    loop {
        presses += 1;
        update_state(
            "button",
            "broadcaster",
            false,
            modules,
            &mut states,
            &mut lb_input_was_up,
        );
        lb_input_was_up
            .iter()
            .filter(|(_, v)| **v)
            .for_each(|(name, _)| {
                //println!("{name} was up at cycle {presses}");
                lb_input_first_up_cycle
                    .entry(name.to_string())
                    .or_insert(presses);
            });
        if lb_input_was_up.len() == lb_input_first_up_cycle.len() {
            break;
        }
        lb_input_was_up.values_mut().for_each(|v| *v = false);
    }
    lb_input_first_up_cycle
        .into_values()
        .reduce(lcm)
        .expect("an lcm")
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
