use advent2023::*;
fn main() {
    let things = parse(input!());
    //part 1
    let res = part1(things.clone(), 200000000000000, 400000000000000);
    println!("Part 1: {}", res);
    //part 2
    let res = part2(things);
    println!("Part 2: {}", res);
}
type ParsedItem = Stone;
#[derive(Debug, Clone)]
struct Pos {
    x: i64,
    y: i64,
    z: i64,
}
#[derive(Debug, Clone)]
struct Stone {
    pos: Pos,
    speed: Pos,
}
fn parse(input: &str) -> impl Iterator<Item = ParsedItem> + Clone + '_ {
    input.lines().map(|l| {
        let (coords, v) = l.split_once('@').expect("two parts");
        let mut coords = ints(coords);
        let mut v = ints(v);
        Stone {
            pos: Pos {
                x: coords.next().expect("x"),
                y: coords.next().expect("y"),
                z: coords.next().expect("z"),
            },
            speed: Pos {
                x: v.next().expect("x speed"),
                y: v.next().expect("y speed"),
                z: v.next().expect("z speed"),
            },
        }
    })
}
fn part1<I>(things: I, min: i64, max: i64) -> usize
where
    I: Iterator<Item = ParsedItem>,
{
    let stones: Vec<_> = things.collect();
    let mut count = 0;
    for i in 0..stones.len() {
        for j in i..stones.len() {
            let a = &stones[i];
            let b = &stones[j];
            if intersect_in_2d((min as f64, max as f64), a, b) {
                println!("{a:?} intersect {b:?}");
                count += 1
            }
        }
    }
    count
}

fn intersect_in_2d(zone: (f64, f64), a: &Stone, b: &Stone) -> bool {
    if let Some((xi, yi)) = intersect_pos_2d(a, b) {
        if !a.forward_in_time_2d(xi, yi) || !b.forward_in_time_2d(xi, yi) {
            return false;
        }
        if xi < zone.0 || xi > zone.1 || yi < zone.0 || xi > zone.1 {
            return false;
        }
        return true;
    }
    false
}

fn intersect_pos_2d(a: &Stone, b: &Stone) -> Option<(f64, f64)> {
    // we have:
    // speed, vector: (N, M), start: I, J
    // At Y= J  X = I
    // At Y = J+M, X = N+I
    // At X = 0, Y =
    // coeff = ( ( J - (J + M ))/  (I - (I + N)) = M / N
    // y = coeff * x + start
    // at X = I, Y = J
    // J = coeff*I + start
    // start = J - coeff * I
    // start = J - M*I/N
    //
    // y = (M/N) * x + J - M*I/N
    // y = (M/N) * (x -I) + J
    //
    // intersection :
    // xi = ( startb - starta) / (coeffa - coeffb)
    // yi = coeffa * xi  + starta
    //
    // Do they intersect *forward* in time ? ; i.e after the start coordinate
    // Only if :
    // yi > each J if its M > 0 and yi < J if M < 0
    // xi > each I if its N > 0 and xi < I if N < 0
    let (coeff_a, start_a) = a.vector_to_affine();
    let (coeff_b, start_b) = b.vector_to_affine();
    if coeff_a - coeff_b == 0.0 {
        return None;
    }
    // xi = ( startb - starta) / (coeffa - coeffb)
    let xi = (start_b - start_a) / (coeff_a - coeff_b);
    // yi = coeffa * xi  + starta
    let yi = coeff_a * xi + start_a;
    Some((xi, yi))
}
impl Stone {
    fn vector_to_affine(&self) -> (f64, f64) {
        (
            // coeff = ( ( J - (J + M ))/  (I - (I + N)) = M / N
            self.speed.y as f64 / self.speed.x as f64,
            // start = J - M*I/N
            self.pos.y as f64 - self.speed.y as f64 * self.pos.x as f64 / self.speed.x as f64,
        )
    }
    fn forward_in_time_2d(&self, xi: f64, yi: f64) -> bool {
        // Do they intersect *forward* in time ? ; i.e after the start coordinate
        // Only if :
        // yi > each J if its M > 0 and yi < J if M < 0
        // xi > each I if its N > 0 and xi < I if N < 0
        if yi < self.pos.y as f64 && self.speed.y > 0 {
            return false;
        }
        if yi > self.pos.y as f64 && self.speed.y < 0 {
            return false;
        }
        if xi < self.pos.x as f64 && self.speed.x > 0 {
            return false;
        }
        if xi > self.pos.x as f64 && self.speed.x < 0 {
            return false;
        }
        true
    }
}

#[test]
fn test_intersect() {
    for (a, b, pos) in [
        (
            Stone {
                pos: Pos {
                    x: 19,
                    y: 13,
                    z: 30,
                },
                speed: Pos { x: -2, y: 1, z: -2 },
            },
            Stone {
                pos: Pos {
                    x: 18,
                    y: 19,
                    z: 22,
                },
                speed: Pos {
                    x: -1,
                    y: -1,
                    z: -2,
                },
            },
            Some((14.3333, 15.3333)),
        ),
        (
            Stone {
                pos: Pos {
                    x: 19,
                    y: 13,
                    z: 30,
                },
                speed: Pos { x: -2, y: 1, z: -2 },
            },
            Stone {
                pos: Pos {
                    x: 20,
                    y: 25,
                    z: 34,
                },
                speed: Pos {
                    x: -2,
                    y: -2,
                    z: -4,
                },
            },
            Some((11.667, 16.667)),
        ),
        (
            Stone {
                pos: Pos {
                    x: 18,
                    y: 19,
                    z: 22,
                },
                speed: Pos {
                    x: -1,
                    y: -1,
                    z: -2,
                },
            },
            Stone {
                pos: Pos {
                    x: 20,
                    y: 25,
                    z: 34,
                },
                speed: Pos {
                    x: -2,
                    y: -2,
                    z: -4,
                },
            },
            None,
        ),
    ]
    .into_iter()
    {
        let i = intersect_pos_2d(&a, &b);
        if pos.is_none() {
            assert!(i.is_none());
        }
        if let Some((xi, yi)) = i {
            assert!(pos.is_some());
            let (resx, resy) = pos.unwrap();
            assert!((resx - xi).abs() < 0.01);
            assert!((resy - yi).abs() < 0.01);
        }
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
    let res = part1(things.clone(), 7, 27);
    assert_eq!(res, 2);
    //part 2
    //let res = part2(things);
    //assert_eq!(res, 42);
}
