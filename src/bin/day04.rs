use advent2023::*;
fn main() {
    let (cards, sep) = parse(input!());
    //part 1
    let res = operation(cards.clone(), sep);
    println!("Part 1: {}", res);
    //part 2
    let res = operation2(cards, sep);
    println!("Part 2: {}", res);
}
type ParsedItem = Vec<u8>;
fn parse(input: &str) -> (impl Iterator<Item = ParsedItem> + Clone + '_, usize) {
    let (sep, _) = input
        .split_ascii_whitespace()
        .enumerate()
        .skip(2)
        .find(|(_, s)| *s == "|")
        .expect("a | separator");
    (
        input.lines().map(|x| {
            x.split_ascii_whitespace()
                .skip(2)
                .filter(|s| *s != "|")
                .map(|n| n.parse().expect("not int"))
                .collect()
        }),
        sep - 2, // we did skip the first two fields, so take that into account
    )
}
fn operation<I>(things: I, sep: usize) -> usize
where
    I: Iterator<Item = ParsedItem>,
{
    let mut sum = 0;
    for l in things {
        let mut winning = [false; 100];
        let mut score = 0;
        for i in l.iter().take(sep) {
            winning[*i as usize] = true;
        }
        for n in l.iter().skip(sep) {
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

fn operation2<I>(things: I, sep: usize) -> usize
where
    I: Iterator<Item = ParsedItem>,
{
    let cards: Vec<_> = things.collect();
    let mut copies = vec![1_usize; cards.len()];
    for (i, l) in cards.iter().enumerate() {
        let mut winning = [false; 100];
        let mut matches = 0;
        for i in l.iter().take(sep) {
            winning[*i as usize] = true;
        }
        for n in l.iter().skip(sep) {
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
    let (cards, sep) = parse(sample!());
    //part 1
    let res = operation(cards.clone(), sep);
    assert_eq!(res, 13);
    //part 2
    let res = operation2(cards, sep);
    assert_eq!(res, 30);
}
