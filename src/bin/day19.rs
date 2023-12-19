use std::collections::HashMap;

use advent2023::*;
fn main() {
    let (wf, parts) = parse(input!());
    //part 1
    let res = part1(&wf, parts.clone());
    println!("Part 1: {}", res);
    //part 2
    let res = part2(&wf);
    println!("Part 2: {}", res);
}
type ParsedItem = Part;

type Part = [usize; 4];

type Workflows<'a> = HashMap<&'a str, Vec<Rule<'a>>>;

struct Rule<'a> {
    exp: Option<Exp>,
    dest: &'a str,
}

struct Exp {
    operand: u8,
    op: u8,
    val: usize,
}

fn parse(input: &str) -> (Workflows, impl Iterator<Item = ParsedItem> + Clone + '_) {
    let (ws, ps) = input.split_once("\n\n").expect("two parts");
    let workflows = ws
        .lines()
        .map(|l| {
            let mut parts = l.split(|c| c == '{' || c == '}');
            let name = parts.next().expect("a name");
            let rs = parts.next().expect("a part list");
            //println!("{name}, {rs}");
            let rules = rs
                .split(',')
                .map(|r| {
                    //println!("Parsing rule {r}");
                    let (exp, dest) = if r.contains(':') {
                        let (es, dest) = r.split_once(':').expect("rule in two part");
                        let esb = es.as_bytes();
                        let operand = esb[0];
                        let op = esb[1];
                        let val = ints(es).next().expect("an int val");
                        (Some(Exp { operand, op, val }), dest)
                    } else {
                        (None, r)
                    };
                    Rule { exp, dest }
                })
                .collect();

            (name, rules)
        })
        .collect();
    (
        workflows,
        ps.lines().map(|l| {
            ints(l)
                .collect::<Vec<_>>()
                .try_into()
                .expect("a fixed size int array")
        }),
    )
}
fn part1<I>(wf: &Workflows, parts: I) -> usize
where
    I: Iterator<Item = ParsedItem>,
{
    parts
        .filter(|p| accept(wf, p))
        .map(|p| p.iter().sum::<usize>())
        .sum()
}

fn accept(wf: &Workflows, part: &Part) -> bool {
    let mut w = "in";
    loop {
        for rule in &wf[w] {
            let res = if let Some(exp) = &rule.exp {
                match exp.op {
                    b'<' => part[operand_to_idx(exp.operand)] < exp.val,
                    b'>' => part[operand_to_idx(exp.operand)] > exp.val,
                    _ => unreachable!(),
                }
            } else {
                false
            };
            if res || rule.exp.is_none() {
                match rule.dest {
                    "A" => return true,
                    "R" => return false,
                    _ => {
                        w = rule.dest;
                        break;
                    }
                }
            }
        }
    }
}

fn operand_to_idx(operand: u8) -> usize {
    match operand {
        b'x' => 0,
        b'm' => 1,
        b'a' => 2,
        b's' => 3,
        _ => unreachable!(),
    }
}

#[derive(Debug, Clone, Copy)]
struct Accepted {
    min: usize,
    max: usize,
}

fn part2(wf: &Workflows) -> usize {
    accept2(wf)
}
fn accept2(wf: &Workflows) -> usize {
    let res = [Accepted { min: 1, max: 4001 }; 4];
    accept2_tree(wf, "in", res).expect("accepted")
}

fn accept2_tree(wf: &Workflows, name: &str, mut ranges: [Accepted; 4]) -> Option<usize> {
    let mut res = None;
    //println!("Evaluating rule {name}");
    for rule in &wf[name] {
        //println!("dest is {}", rule.dest);
        let mut new = ranges;
        if let Some(exp) = &rule.exp {
            match exp.op {
                b'<' => {
                    new[operand_to_idx(exp.operand)].max = exp.val;
                    ranges[operand_to_idx(exp.operand)].min = exp.val;
                }
                b'>' => {
                    new[operand_to_idx(exp.operand)].min = exp.val + 1;
                    ranges[operand_to_idx(exp.operand)].max = exp.val + 1;
                }
                _ => unreachable!(),
            }
            eval_dest(&mut res, wf, rule.dest, &new);
        } else {
            eval_dest(&mut res, wf, rule.dest, &ranges);
        }
    }
    res
}
fn eval_dest(res: &mut Option<usize>, wf: &Workflows, name: &str, ranges: &[Accepted; 4]) {
    match name {
        "A" => {
            *res = add(
                *res,
                Some(
                    ranges
                        .iter()
                        //.inspect(|r| println!("Accepted: {r:?}"))
                        .map(|r| if r.max < r.min { 0 } else { r.max - r.min })
                        //.map(|r| r.max - r.min)
                        .product::<usize>(),
                ),
            )
        }
        "R" => {}
        _ => {
            *res = add(*res, accept2_tree(wf, name, *ranges));
        }
    }
}

fn add(a: Option<usize>, b: Option<usize>) -> Option<usize> {
    if a.is_none() && b.is_none() {
        return None;
    }
    if a.is_none() {
        return b;
    }
    if b.is_none() {
        return a;
    }
    Some(a.unwrap() + b.unwrap())
}

#[test]
fn test() {
    let (wf, parts) = parse(sample!());
    //part 1
    let res = part1(&wf, parts.clone());
    assert_eq!(res, 19114);
    //part 2
    let res = part2(&wf);
    assert_eq!(res, 167409079868000);
}
