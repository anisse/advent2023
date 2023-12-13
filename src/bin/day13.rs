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
type ParsedItem = Vec<Vec<char>>;
fn parse(input: &str) -> impl Iterator<Item = ParsedItem> + Clone + '_ {
    input
        .split("\n\n")
        .map(|p| p.lines().map(|l| l.chars().collect()).collect())
}
fn part1<I>(things: I) -> usize
where
    I: Iterator<Item = ParsedItem>,
{
    things.map(reflect_score).sum()
}

fn reflect_score(p: ParsedItem) -> usize {
    let rows = p.len();
    let cols = p[0].len();
    for r in 0..rows {
        let mut mirror = true;
        let mut tested = false;
        'outer: for c in 0..cols {
            //println!("Testing row {r}, col {c}");
            for (r1, r2) in (0..=r).rev().zip((r + 1)..rows) {
                tested = true;
                //println!("({r1}, {c}) vs ({r2}, {c}) = {}", p[r1][c] != p[r2][c]);
                if p[r1][c] != p[r2][c] {
                    mirror = false;
                    break 'outer;
                }
            }
        }
        if mirror && tested {
            //println!("Row {r} is mirror");
            return (r + 1) * 100;
        }
    }
    for c in 0..cols {
        let mut mirror = true;
        (0..rows).for_each(|r| {
            for (c1, c2) in (0..=c).rev().zip((c + 1)..cols) {
                //println!("({r}, {c1}) vs ({r}, {c2})");
                if p[r][c1] != p[r][c2] {
                    mirror = false;
                    return;
                }
            }
        });
        if mirror {
            //println!("col {c} is mirror");
            return c + 1;
        }
    }
    0
}
fn part2<I>(things: I) -> usize
where
    I: Iterator<Item = ParsedItem>,
{
    things.map(reflect_score_2).sum()
}
fn reflect_score_2(p: ParsedItem) -> usize {
    let rows = p.len();
    let cols = p[0].len();
    for r in 0..rows {
        let mut unmatch = 0;
        'outer: for c in 0..cols {
            //println!("Testing row {r}, col {c}");
            for (r1, r2) in (0..=r).rev().zip((r + 1)..rows) {
                //println!("({r1}, {c}) vs ({r2}, {c}) = {}", p[r1][c] != p[r2][c]);
                if p[r1][c] != p[r2][c] {
                    unmatch += 1;
                }
                if unmatch > 1 {
                    break 'outer;
                }
            }
        }
        if unmatch == 1 {
            //println!("Row {r} is mirror");
            return (r + 1) * 100;
        }
    }
    for c in 0..cols {
        let mut unmatch = 0;
        (0..rows).for_each(|r| {
            for (c1, c2) in (0..=c).rev().zip((c + 1)..cols) {
                //println!("({r}, {c1}) vs ({r}, {c2})");
                if p[r][c1] != p[r][c2] {
                    unmatch += 1;
                }
                if unmatch > 1 {
                    return;
                }
            }
        });
        if unmatch == 1 {
            //println!("col {c} is mirror");
            return c + 1;
        }
    }
    0
}

#[test]
fn test() {
    let things = parse(sample!());
    //part 1
    let res = part1(things.clone());
    assert_eq!(res, 405);
    //part 2
    let res = part2(things);
    assert_eq!(res, 400);
}
