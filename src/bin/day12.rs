use std::collections::HashMap;

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
type ParsedItem = Record;

#[derive(Debug)]
struct Record {
    row: Vec<char>,
    groups: Vec<u8>,
}
fn parse(input: &str) -> impl Iterator<Item = ParsedItem> + Clone + '_ {
    input.lines().map(|l| {
        let (row, g) = l.split_once(' ').expect("two groups");
        Record {
            row: row.chars().collect(),
            groups: ints(g).collect(),
        }
    })
}
fn part1<I>(records: I) -> usize
where
    I: Iterator<Item = ParsedItem>,
{
    records.map(|r| arrangements(&r)).sum()
}

fn arrangements(r: &Record) -> usize {
    println!("===================================");
    println!("===================================");
    println!("===================================");
    println!("===================================");
    println!("===================================");
    println!("Start of record testing {r:?}");
    let mut memo = HashMap::new();
    arrangements_memo(&mut memo, r, 0, 0, 0)
}
#[derive(PartialEq, Eq, Hash)]
struct Key {
    consumed: usize,
    group_idx: usize,
    group_length: u8,
}
type Memo = HashMap<Key, usize>;
fn arrangements_memo(
    memo: &mut Memo,
    r: &Record,
    group_length: u8,
    consumed: usize,
    group_idx: usize,
) -> usize {
    if let Some(count) = memo.get(&Key {
        consumed,
        group_idx,
        group_length,
    }) {
        return *count;
    }
    let count = arrangements_internal(memo, r, group_length, consumed, group_idx);
    memo.insert(
        Key {
            consumed,
            group_idx,
            group_length,
        },
        count,
    );
    count
}
fn arrangements_internal(
    memo: &mut Memo,
    r: &Record,
    mut group_length: u8,
    mut consumed: usize,
    group_idx: usize,
) -> usize {
    for g in group_idx..r.groups.len() {
        let current_group = r.groups[g];
        let mut group_ended = false;
        while consumed < r.row.len() {
            match r.row[consumed] {
                '.' => {
                    if !(group_length == current_group || group_length == 0) {
                        //println!("group has not ended and we found a .");

                        return 0; // impossible arrangement
                    }
                    if group_length == current_group {
                        //println!(". marks end of group, break");
                        group_length = 0;
                        consumed += 1;
                        group_ended = true;
                        break;
                    }
                }
                '#' => {
                    if group_length >= current_group {
                        //println!("group {current_group} is too big vs group size {current_group}");
                        return 0; // impossible arrangement
                    }
                    //println!("# in group");
                    group_length += 1;
                }
                '?' => {
                    // must be one or the other
                    if group_length > 0 && group_length < current_group {
                        //println!("? must be #");
                        group_length += 1;
                    } else if group_length == current_group {
                        //println!("? must be .");
                        group_length = 0;
                        consumed += 1;
                        group_ended = true;
                        break;
                    }
                    // can be either . or #
                    else if group_length == 0 {
                        // not in group
                        //println!("? can be . or # recursion");
                        // #
                        let c1 = arrangements_memo(memo, r, group_length + 1, consumed + 1, g);
                        //println!("recursion {consumed} # end: {c1}");
                        // .
                        let c2 = arrangements_memo(memo, r, group_length, consumed + 1, g);
                        //println!("recursion {consumed} . end: {c2}");
                        return c1 + c2;
                    } else {
                        unreachable!();
                    }
                }
                _ => unreachable!(),
            }
            /*
            println!(
                "matching of character {consumed} = {} in group {current_group} done",
                r.row[consumed]
            );
            */
            consumed += 1;
        }
        //println!("Group {g} = {current_group} end");
        if consumed == r.row.len() {
            /*
            println!(
                "Reached end of row; group_length = {group_length}, group_ended = {group_ended}"
            );
            */
            if group_length == 0 && !group_ended {
                // Impossible configuration
                //println!("Group not done: expected {current_group} to be done");
                return 0;
            }
            if group_length != 0 && group_length != current_group {
                // Impossible configuration
                //println!("Group not done: expected {current_group}, got {group_length}");
                return 0;
            }
            if g != r.groups.len() - 1 {
                // All groups not used
                //println!("There are remaining groups");
                return 0;
            }
        }
        let remaining = r.row.len() - consumed;
        let group_total = r
            .groups
            .iter()
            .skip(g + 1)
            .map(|x| *x as usize)
            .sum::<usize>();
        let groups_left = r.groups.len() - (g + 1);
        //println!("Groups left: {groups_left}, char remaining: {remaining}");
        if groups_left > 0 && remaining < group_total + groups_left - 1 {
            //println!("Skipping rest of {remaining} characters, cannot fit {groups_left} group of {group_total} elements");
            return 0;
        }
        if remaining > 1 && groups_left == 1 && r.row.iter().skip(consumed).all(|c| *c == '?') {
            let last = r.groups[g + 1] as usize;
            /*
            println!(
                "Skip last group of {last} chars: {remaining} all ? : {} possibilities",
                remaining - last + 1
            );
            */
            return remaining - last + 1;
        }
    }
    // remaining characters
    if consumed < r.row.len() && !r.row.iter().skip(consumed).all(|c| *c != '#') {
        return 0;
    }
    //println!("Record done with count 1");
    1
}

#[test]
fn test_arr() {
    for (t, res) in [
        (". 1", 0),
        ("???? 2,1", 1),
        ("????? 2,1", 3),
        ("?????? 2,1", 6),
        ("??????? 2,1", 10),
        ("?? 1", 2),
        ("??? 1", 3),
        ("???? 1", 4),
        ("??# 1", 1),
        ("??#? 1", 1),
        (".??..??...?##. 1,1,3", 4),
        ("???.### 1,1,3", 1),
        ("?#?#?#?#?#?#?#? 1,3,1,6", 1),
        ("????.#...#... 4,1,1", 1),
        ("????.######..#####. 1,6,5", 4),
        ("??????? 1", 7),
        ("??????? 2", 6),
        ("??????? 3", 5),
    ]
    .iter()
    {
        let r = parse(t).next().unwrap();
        assert_eq!(arrangements(&r), *res, "test {t} is not {res}");
    }
}

fn part2<I>(records: I) -> usize
where
    I: Iterator<Item = ParsedItem>,
{
    records
        .enumerate()
        .map(|(i, r)| {
            let mut row = vec![];
            let mut groups = vec![];
            for i in 0..5 {
                row.extend_from_slice(&r.row);
                if i != 4 {
                    row.push('?');
                }
                groups.extend_from_slice(&r.groups);
            }
            println!("{i}");
            Record { row, groups }
        })
        .map(|r| arrangements(&r))
        .sum()
}
#[test]
fn test_arr_2() {
    for (t, res) in [
        ("?###???????? 3,2,1", 506250),
        (".??..??...?##. 1,1,3", 16384),
    ]
    .iter()
    {
        let r = parse(t);
        assert_eq!(part2(r), *res, "test {t} is not {res}");
    }
}

#[test]
fn test() {
    let things = parse(sample!());
    //part 1
    let res = part1(things.clone());
    assert_eq!(res, 21);
    //part 2
    let res = part2(things);
    assert_eq!(res, 525152);
}
