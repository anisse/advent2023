use std::collections::HashMap;

use advent2023::*;
fn main() {
    let (ins, map) = parse(input!());
    //part 1
    let res = part1(&ins, &map);
    println!("Part 1: {}", res);
    //part 2
    let res = part2(&ins, &map);
    println!("Part 2: {}", res);
}
struct LR<'a> {
    left: &'a str,
    right: &'a str,
}

#[derive(Clone, Copy, Debug)]
enum Ins {
    L,
    R,
}

fn parse(input: &str) -> (Vec<Ins>, HashMap<&str, LR>) {
    let mut i = input.lines();
    (
        i.next()
            .expect("first line")
            .chars()
            .map(|c| match c {
                'L' => Ins::L,
                'R' => Ins::R,
                _ => unreachable!(),
            })
            .collect(),
        i.skip(1)
            .map(|l| {
                let (a, rest) = l.split_at(3);
                let (l, rest) = rest.split_at(4).1.split_at(3);
                let (r, _) = rest.split_at(2).1.split_at(3);
                (a, LR { left: l, right: r })
            })
            .collect(),
    )
}
fn part1(ins: &[Ins], map: &HashMap<&str, LR>) -> usize {
    let mut i = 0;
    let mut current = "AAA";
    loop {
        current = match ins[i % ins.len()] {
            Ins::L => map[current].left,
            Ins::R => map[current].right,
        };
        i += 1;
        if current == "ZZZ" {
            return i;
        }
    }
}

fn part2(ins: &[Ins], map: &HashMap<&str, LR>) -> usize {
    map.keys()
        .filter(|s| s.chars().last().expect("last A") == 'A')
        .map(|start| {
            let mut current: &str = start;
            let mut i = 0;
            loop {
                current = match ins[i % ins.len()] {
                    Ins::L => &map[&current].left,
                    Ins::R => &map[&current].right,
                };
                i += 1;
                if current.chars().last().expect("last Z") == 'Z' {
                    break i;
                }
            }
        })
        .reduce(|lcm, a| lcm * a / gcd(lcm, a))
        .expect("an lcm")
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while a != b {
        if a > b {
            a -= b;
        } else {
            b -= a;
        }
    }
    a
}
#[test]
fn test() {
    let (ins, map) = parse(sample!());
    //part 1
    let res = part1(&ins, &map);
    assert_eq!(res, 2);
    //part 2
    let (ins, map) = parse(
        "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)",
    );
    let res = part2(&ins, &map);
    assert_eq!(res, 6);
}
