use advent2023::*;
use itertools::Itertools;
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
                //println!("{a:?} intersect {b:?}");
                count += 1
            }
        }
    }
    count
}

fn intersect_in_2d(zone: (f64, f64), a: &Stone, b: &Stone) -> bool {
    if let Some((xi, yi)) = a.intersect_pos_2d(b) {
        if !a.forward_in_time_2d(xi, yi) || !b.forward_in_time_2d(xi, yi) {
            return false;
        }
        if xi < zone.0 || xi > zone.1 || yi < zone.0 || yi > zone.1 {
            return false;
        }
        return true;
    }
    false
}

impl Stone {
    fn intersect_pos_2d(&self, b: &Stone) -> Option<(f64, f64)> {
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
        let (coeff_a, start_a) = self.vector_to_affine_2d();
        let (coeff_b, start_b) = b.vector_to_affine_2d();
        if coeff_a - coeff_b == 0.0 {
            return None;
        }
        // xi = ( startb - starta) / (coeffa - coeffb)
        let xi = (start_b - start_a) / (coeff_a - coeff_b);
        // yi = coeffa * xi  + starta
        let yi = coeff_a * xi + start_a;
        Some((xi, yi))
    }
    fn vector_to_affine_2d(&self) -> (f64, f64) {
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
        let i = a.intersect_pos_2d(&b);
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
    let stones: Vec<_> = things.collect();
    let d = part2_bruteforce(&stones);
    (d[0].0 + d[1].0 + d[2].0) as usize
}
fn part2_bruteforce(stones: &[Stone]) -> [Dim; 3] {
    /*
    let dims: Vec<[Dim; 3]> = stones
        .iter()
        .map(|s| {
            [
                (s.pos.x, s.speed.x),
                (s.pos.y, s.speed.y),
                (s.pos.z, s.speed.z),
            ]
        })
        .collect();
    */
    let dims: Vec<[Dim; 3]> = stones
        .iter()
        .map(|s| {
            (
                stones
                    .iter()
                    .map(|s2| {
                        // manhattan dist
                        (s.pos.x - s2.pos.x).abs()
                            + (s.pos.y - s2.pos.y).abs()
                            + (s.pos.z - s2.pos.z).abs()
                    })
                    .sum::<i64>(),
                [
                    (s.pos.x, s.speed.x),
                    (s.pos.y, s.speed.y),
                    (s.pos.z, s.speed.z),
                ],
            )
        })
        // Sort stones by "centrality": distances to all other stones
        // the idea being that they are the closest to colliding to others
        //.inspect(|(a, b)| println!("{b:?}: {a}"))
        .sorted()
        //.inspect(|(a, b)| println!("sorted {b:?}: {a}"))
        .map(|(_, b)| b)
        .collect();
    cross_dim(&dims)
}

// 1-dimensionnal vectors
type Dim = (i64, i64);

fn cross_dim(stones: &[[Dim; 3]]) -> [Dim; 3] {
    let mut ret: [Dim; 3] = Default::default();
    let mut xi0 = vec![0.0; stones.len()];
    for x1 in 0..9000 {
        for s1 in 0..stones.len() {
            for s2 in (s1 + 1)..stones.len() {
                'n_selection: for n in 1..1000 {
                    let mut found = false;
                    'dim_selection: for d in 0..3 {
                        let y1 = stones[s1][d].0 + stones[s1][d].1 * x1;
                        let y2 = stones[s2][d].0 + stones[s2][d].1 * (x1 + n);
                        if (y2 - y1) % n != 0 {
                            continue 'n_selection;
                        }
                        let v = (y2 - y1) / n;
                        let start = y1 - v * x1;
                        //assert_eq!(start, y2 - v * (x1 + n));
                        let stone = Stone {
                            pos: Pos {
                                x: 0,
                                y: start,
                                z: 0,
                            },
                            speed: Pos { x: 1, y: v, z: 0 },
                        };
                        // Does d intersect the net point at an integer coordinate ?
                        for s3 in 0..stones.len() {
                            if s3 == s2 || s3 == s1 {
                                continue;
                            }
                            let stone2 = Stone {
                                pos: Pos {
                                    x: 0,
                                    y: stones[s3][d].0,
                                    z: 0,
                                },
                                speed: Pos {
                                    x: 1,
                                    y: stones[s3][d].1,
                                    z: 0,
                                },
                            };
                            if let Some((xi, yi)) = stone.intersect_pos_2d(&stone2) {
                                if !stone.forward_in_time_2d(xi, yi)
                                    || !stone2.forward_in_time_2d(xi, yi)
                                {
                                    found = false;
                                    break 'dim_selection;
                                }
                                if xi.fract() > 1e-10 || yi.fract() > 1e-10 {
                                    found = false;
                                    break 'dim_selection;
                                }
                                if d == 0 {
                                    xi0[s3] = xi;
                                } else if (xi0[s3] - xi).abs() > 1e-10 {
                                    // are Xi all the same ?
                                    found = false;
                                    break 'dim_selection;
                                }
                                found = true;
                                println!(
                                    "{:?} at {x1} and {:?} at {}, {:?} intersects at n={}",
                                    stones[s1],
                                    stones[s2],
                                    x1 + n,
                                    stones[s3],
                                    xi
                                );
                            } else {
                                if d == 0 && start == 24 && v == -3 {
                                    println!("does not intersects");
                                }
                                found = false;
                                break 'dim_selection;
                            }
                        }
                        println!("After other stones: {found}");
                        if found {
                            ret[d] = (start, v);
                        }
                    }
                    if found {
                        println!(
                            "Found intersection with every other point for vectors ({ret:?}, n={}",
                            x1 + n
                        );
                        return ret;
                    }
                }
            }
        }
    }

    [(0, 0), (0, 0), (0, 0)]
}

#[test]
fn test_cross() {
    let stones: Vec<_> = parse(sample!()).collect();
    let d = part2_bruteforce(&stones);
    assert_eq!(d[0], (24, -3));
    assert_eq!(d[1], (13, 1));
    assert_eq!(d[2], (10, 2));
}

#[test]
fn test() {
    let things = parse(sample!());
    //part 1
    let res = part1(things.clone(), 7, 27);
    assert_eq!(res, 2);
    //part 2
    let res = part2(things);
    assert_eq!(res, 47);
}
