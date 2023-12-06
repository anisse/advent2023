use advent2023::*;
fn main() {
    let (t, d) = parse(input!());
    //part 1
    let res = operation(&t, &d);
    println!("Part 1: {}", res);
    //part 2
    let res = operation2(&t, &d);
    println!("Part 2: {}", res);
}
fn parse(input: &str) -> (Vec<u64>, Vec<u64>) {
    let td: Vec<_> = input
        .lines()
        .map(|l| {
            l.split_ascii_whitespace()
                .skip(1)
                .map(|x| x.parse().expect("not int"))
                .collect::<Vec<u64>>()
        })
        .collect();
    (td[0].clone(), td[1].clone())
}
fn operation(t: &[u64], d: &[u64]) -> usize {
    t.iter()
        .zip(d.iter())
        .map(|(time, distance)| {
            (1..*time)
                .map(|hold| (time - hold) * hold)
                .filter(|d| d > distance)
                .count()
        })
        .product()
}

fn operation2(t: &[u64], d: &[u64]) -> usize {
    let time: usize = t
        .iter()
        .map(|x| x.to_string())
        .reduce(|acc, x| acc + &x)
        .unwrap()
        .parse()
        .unwrap();
    let distance: usize = d
        .iter()
        .map(|x| x.to_string())
        .reduce(|acc, x| acc + &x)
        .unwrap()
        .parse()
        .unwrap();
    // Let's find when this equation is positive:
    // y = -x^2 + time * x - distance
    // https://en.wikipedia.org/wiki/Quadratic_equation#Discriminant
    let a: f64 = -1.0;
    let b: f64 = time as f64;
    let c: f64 = -(distance as f64);
    let s_delta = (b * b - 4.0 * a * c).sqrt();
    let x1 = (-b + s_delta) / (2.0 * a);
    let x2 = (-b - s_delta) / (2.0 * a);
    x2 as usize - x1 as usize
}

#[test]
fn test() {
    let (t, d) = parse(sample!());
    //part 1
    let res = operation(&t, &d);
    assert_eq!(res, 288);
    //part 2
    let res = operation2(&t, &d);
    assert_eq!(res, 71503);
}
