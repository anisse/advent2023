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
type ParsedItem = String;
fn parse(input: &str) -> impl Iterator<Item = ParsedItem> + Clone + '_ {
    input.lines().map(|x| x.to_string())
}
fn operation<I>(things: I) -> usize
where
    I: Iterator<Item = ParsedItem>,
{
    things
        .map(|l| l.chars().filter(|c| c.is_digit(10)).collect())
        .map(|s: String| {
            [s.chars().next().unwrap(), s.chars().last().unwrap()]
                .iter()
                .collect()
        })
        .map(|l: String| l.parse::<usize>().expect("not int"))
        .sum()
}

fn operation2<I>(things: I) -> usize
where
    I: Iterator<Item = ParsedItem>,
{
    things
        .filter(|s| {
            println!("before: {s}");
            true
        })
        .map(|s| [first_digit(&s), last_digit(&s)].iter().collect())
        .filter(|s| {
            println!("int: {s}");
            true
        })
        .map(|l: String| l.parse::<usize>().expect("not int"))
        .sum()
}

fn first_digit(s: &str) -> char {
    let mut i = 0;
    let mut s = s.to_string();
    while i <= s.len() {
        s = s[..i]
            .replace("four", "4")
            .replace("five", "5")
            .replace("six", "6")
            .replace("seven", "7")
            .replace("nine", "9")
            .replace("one", "1")
            .replace("eight", "8")
            .replace("two", "2")
            .replace("three", "3")
            .to_string()
            + &s[i..];
        i += 1;
    }
    s.chars().find(|c| c.is_ascii_digit()).unwrap()
}
fn last_digit(s: &str) -> char {
    let mut i = s.len();
    let mut s = s.to_string();
    loop {
        s = s[..i].to_string()
            + &s[i..]
                .replace("four", "4")
                .replace("five", "5")
                .replace("six", "6")
                .replace("seven", "7")
                .replace("nine", "9")
                .replace("one", "1")
                .replace("eight", "8")
                .replace("two", "2")
                .replace("three", "3");
        if i == 0 {
            break;
        }
        i -= 1;
    }
    s.chars().filter(|c| c.is_ascii_digit()).last().unwrap()
}

#[test]
fn test() {
    let things = parse(sample!());
    //part 1
    //let res = operation(things.clone());
    //assert_eq!(res, 42);
    //part 2
    let res = operation2(things);
    assert_eq!(res, 281);
}
