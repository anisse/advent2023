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
type ParsedItem = String;
fn parse(input: &str) -> impl Iterator<Item = ParsedItem> + Clone + '_ {
    input.split(',').map(|s| s.to_string())
}

fn hash(s: String) -> usize {
    s.chars().fold(0, |acc, c| ((acc + c as usize) * 17) % 256)
}
fn part1<I>(things: I) -> usize
where
    I: Iterator<Item = ParsedItem>,
{
    things.map(hash).sum()
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
    let things = parse("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7");
    //part 1
    let res = part1(things.clone());
    assert_eq!(res, 1320);
    //part 2
    /*
    let res = part2(things);
    assert_eq!(res, 42);
    */
}
