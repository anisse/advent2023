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
    //println!("===================================");
    //println!("Start of record testing {r:?}");
    let mut memo = HashMap::new();
    //arrangements_memo(&mut memo, r, 0, 0, 0)
    arrangements_memo(&mut memo, r, 0, 0)
}
#[derive(PartialEq, Eq, Hash)]
struct Key {
    consumed: usize,
    group_idx: usize,
}
type Memo = HashMap<Key, usize>;
fn arrangements_memo(memo: &mut Memo, r: &Record, consumed: usize, group_idx: usize) -> usize {
    let key = Key {
        consumed,
        group_idx,
    };
    return *memo
        .entry(key)
        .or_insert_with(|| arrangements_internal(memo, r, consumed, group_idx));
}
fn arrangements_internal(
    memo: &mut Memo,
    r: &Record,
    mut current: usize,
    mut group_idx: usize,
) -> usize {
    while current < r.row.len() && group_idx < r.groups.len() {
        let group_len = r.groups[group_idx] as usize;
        let rest = &r.row[current..];
        match r.row[current] {
            '.' => {}
            '#' => {
                // consume group
                if !consume(rest, group_len) {
                    return 0;
                }
                current += group_len;
                group_idx += 1;
            }
            '?' => {
                return if !consume(rest, group_len) {
                    0 // # does not work
                } else {
                    // It can be a #
                    arrangements_memo(memo, r, current + group_len + 1, group_idx + 1)
                } + arrangements_memo(memo, r, current + 1, group_idx); // Or a .
            }
            _ => unreachable!(),
        }
        current += 1;
    }
    if group_idx != r.groups.len()  // remaining groups
    // OR remaining unconsumed #, part of no group
        || !r.row.iter().skip(current).all(|c| *c != '#')
    {
        return 0;
    }
    1
}

fn consume(s: &[char], len: usize) -> bool {
    len <= s.len()
        && s.iter().take(len).all(|c| *c != '.')
        && (len == s.len() /* end */|| s[len] != '#'/* has separator */)
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
        .map(|r| {
            let mut row = vec![];
            let mut groups = vec![];
            for i in 0..5 {
                row.extend_from_slice(&r.row);
                if i != 4 {
                    row.push('?');
                }
                groups.extend_from_slice(&r.groups);
            }
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
