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
    arrangements_internal(r, 0, 0, 0)
}
fn arrangements_internal(
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
                        println!("group has not ended and we found a .");
                        return 0; // impossible arrangement
                    }
                    if group_length == current_group {
                        println!(". marks end of group, break");
                        group_length = 0;
                        consumed += 1;
                        group_ended = true;
                        break;
                    }
                }
                '#' => {
                    if group_length >= current_group {
                        println!("group {current_group} is too big vs group size {current_group}");
                        return 0; // impossible arrangement
                    }
                    println!("# in group");
                    group_length += 1;
                }
                '?' => {
                    // must be one or the other
                    if group_length > 0 && group_length < current_group {
                        println!("? must be #");
                        group_length += 1;
                    } else if group_length == current_group {
                        println!("? must be .");
                        group_length = 0;
                        consumed += 1;
                        group_ended = true;
                        break;
                    }
                    // can be either . or #
                    else if group_length == 0 {
                        // not in group
                        println!("? can be . or # recursion");
                        // #
                        let c1 = arrangements_internal(r, group_length + 1, consumed + 1, g);
                        println!("recursion {consumed} # end: {c1}");
                        // .
                        let c2 = arrangements_internal(r, group_length, consumed + 1, g);
                        println!("recursion {consumed} . end: {c2}");
                        return c1 + c2;
                    } else {
                        unreachable!();
                    }
                }
                _ => unreachable!(),
            }
            println!(
                "matching of character {consumed} = {} in group {current_group} done",
                r.row[consumed]
            );
            consumed += 1;
        }
        println!("Group {g} = {current_group} end");
        if consumed == r.row.len() {
            println!(
                "Reached end of row; group_length = {group_length}, group_ended = {group_ended}"
            );
            if group_length == 0 && !group_ended {
                // Impossible configuration
                println!("Group not done: expected {current_group} to be done");
                return 0;
            }
            if group_length != 0 && group_length != current_group {
                // Impossible configuration
                println!("Group not done: expected {current_group}, got {group_length}");
                return 0;
            }
            if g != r.groups.len() - 1 {
                // All groups not used
                println!("There are remaining groups");
                return 0;
            }
        }
    }
    // remaining characters
    if consumed < r.row.len() {
        if !r.row.iter().skip(consumed).all(|c| *c != '#') {
            return 0;
        }
    }
    println!("Record done with count 1");
    1
}

#[test]
fn test_arr() {
    for (t, res) in [
        (".??..??...?##. 1,1,3", 4),
        ("???.### 1,1,3", 1),
        ("?#?#?#?#?#?#?#? 1,3,1,6", 1),
        ("????.#...#... 4,1,1", 1),
        ("????.######..#####. 1,6,5", 4),
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
    ]
    .iter()
    {
        let r = parse(t).next().unwrap();
        assert_eq!(arrangements(&r), *res, "test {t} is not {res}");
    }
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
    let res = part1(things.clone());
    assert_eq!(res, 21);
    //part 2
    /*
    let res = part2(things);
    assert_eq!(res, 42);
    */
}
