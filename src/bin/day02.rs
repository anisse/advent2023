use advent2023::*;
fn main() {
    let things = parse(input!());
    //dbg!(things.clone().collect::<Vec<_>>());
    //part 1
    let res = operation(things.clone());
    println!("Part 1: {}", res);
    //part 2
    let res = operation2(things);
    println!("Part 2: {}", res);
}
type Game = Vec<Cubes>;
#[derive(Clone, Copy, PartialEq, Debug, Default)]
struct Cubes {
    red: u16,
    green: u16,
    blue: u16,
}

type ParsedItem = Game;
fn parse(input: &str) -> impl Iterator<Item = ParsedItem> + Clone + '_ {
    input.lines().map(|l| {
        l.split(':')
            .nth(1)
            .expect("no colon")
            .split(';')
            .map(|g| {
                let mut cubes = Cubes::default();
                //dbg!(g);
                g.trim().split(',').for_each(|c| {
                    //dbg!(c);
                    let mut set = c.trim().split(' ');
                    let num = set.next().expect("num").parse::<u16>().expect("not int");
                    match set.next().expect("color") {
                        "red" => cubes.red = num,
                        "green" => cubes.green = num,
                        "blue" => cubes.blue = num,
                        _ => panic!("unknown color"),
                    }
                });
                cubes
            })
            .collect()
    })
}
fn operation<I>(games: I) -> usize
where
    I: Iterator<Item = ParsedItem>,
{
    games
        .enumerate()
        .filter_map(|(i, g)| {
            if g.iter()
                .all(|c| c.red <= 12 && c.green <= 13 && c.blue <= 14)
            {
                Some(i + 1)
            } else {
                None
            }
        })
        .sum()
}

fn operation2<I>(games: I) -> usize
where
    I: Iterator<Item = ParsedItem>,
{
    games
        .map(|g| {
            g.iter()
                .copied()
                .reduce(|acc, c| Cubes {
                    red: c.red.max(acc.red),
                    green: c.green.max(acc.green),
                    blue: c.blue.max(acc.blue),
                })
                //.map(|c| dbg!(c))
                .map(|c| c.red as usize * c.blue as usize * c.green as usize)
                .unwrap()
        })
        .sum()
}

#[test]
fn test() {
    let things = parse(sample!());
    //part 1
    let res = operation(things.clone());
    assert_eq!(res, 8);
    //part 2
    let res = operation2(things);
    assert_eq!(res, 2286);
}
