use advent2023::*;
fn main() {
    let things = parse(input!());
    //part 1
    let res = operation(things.clone());
    println!("Part 1: {}", res);
    //part 2
    let res = operation2(things);
    println!("Part 2: {}", res);
}
type ParsedItem = Vec<u8>;
fn parse(input: &str) -> impl Iterator<Item = ParsedItem> + Clone + '_ {
    input.lines().map(|x| {
        x.split_ascii_whitespace()
            .skip(2)
            .filter(|s| *s != "|")
            .map(|n| n.parse().expect("not int"))
            .collect()
    })
}
fn operation<I>(things: I) -> usize
where
    I: Iterator<Item = ParsedItem>,
{
    let mut sum = 0;
    for l in things {
        let mut winning = [false; 100];
        let mut score = 0;
        for i in l.iter().take(10) {
            winning[*i as usize] = true;
        }
        for n in l.iter().skip(10) {
            if winning[*n as usize] {
                if score == 0 {
                    score = 1;
                } else {
                    score *= 2;
                }
            }
        }
        //println!("Line {l:?} is worth {score}");
        sum += score;
    }
    sum
}

fn operation2<I>(things: I) -> usize
where
    I: Iterator<Item = ParsedItem>,
{
    let cards: Vec<_> = things.collect();
    let mut copies = vec![1_usize; cards.len()];
    for (i, l) in cards.iter().enumerate() {
        let mut winning = [false; 100];
        let mut matches = 0;
        for i in l.iter().take(10) {
            winning[*i as usize] = true;
        }
        for n in l.iter().skip(10) {
            if winning[*n as usize] {
                matches += 1;
            }
        }
        while matches >= 1 {
            copies[i + matches] += copies[i];
            matches -= 1;
        }
    }
    copies.iter().sum()
}

#[test]
fn test() {
    let things = parse(sample!());
    //part 1
    let res = operation(things.clone());
    assert_eq!(res, 42);
    //part 2
    let res = operation2(things);
    assert_eq!(res, 42);
}
