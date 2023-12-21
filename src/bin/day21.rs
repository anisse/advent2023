use std::collections::{BinaryHeap, HashMap};

use advent2023::*;
fn main() {
    let map = parse(input!());
    //part 1
    let res = part1(&map, 64);
    println!("Part 1: {}", res);
    //part 2
    let res = part2(&map, 26501365);
    println!("Part 2: {}", res);
}

type Map = Vec<Vec<u8>>;
type MapRef<'a> = &'a [Vec<u8>];

type SeenMapRef<'a> = &'a [Vec<Seen>];
type SeenMapRefMut<'a> = &'a mut [Vec<Seen>];

#[derive(Debug, Default, Clone)]
struct Seen {
    even: Option<usize>,
    odd: Option<usize>,
}

fn parse(input: &str) -> Map {
    input.lines().map(|x| x.as_bytes().to_vec()).collect()
}
fn part1(map: MapRef, steps: usize) -> usize {
    let mut seen = vec![vec![Seen::default(); map[0].len()]; map.len()];
    let spos = map
        .iter()
        .enumerate()
        .flat_map(|(y, l)| l.iter().position(|c| *c == b'S').map(|x| (x, y)))
        .next()
        .expect("S pos");
    //dbg!(&spos);
    explore_full(map, &mut seen, spos);
    _print_map_max(map, &seen, steps, steps % 2 == 0);
    seen.iter()
        .flatten()
        .filter(|seen_at| {
            let s = if steps % 2 == 0 {
                &seen_at.even
            } else {
                &seen_at.odd
            };
            if let Some(count) = *s {
                count <= steps
            } else {
                false
            }
        })
        .count()
}
fn _print_map(map: MapRef, seen: SeenMapRef) {
    (0..map.len()).for_each(|y| {
        (0..map[y].len()).for_each(|x| {
            print!(
                "{}",
                match (map[y][x], &seen[y][x]) {
                    (b'#', _) => "#",
                    (b'S', _) => "S",
                    (
                        b'.',
                        Seen {
                            even: Some(_),
                            odd: _,
                        },
                    ) => "O",
                    (b'.', _) => ".",
                    _ => unreachable!(),
                }
            );
        });
        println!();
    });
}
fn _print_map_max(map: MapRef, seen: SeenMapRef, max: usize, even: bool) {
    (0..map.len()).for_each(|y| {
        (0..map[y].len()).for_each(|x| {
            print!(
                "{}",
                match (map[y][x], &seen[y][x]) {
                    (b'#', _) => "#",
                    (
                        _,
                        Seen {
                            even: _,
                            odd: Some(od),
                        },
                    ) =>
                        if *od <= max && !even {
                            "O"
                        } else {
                            "."
                        },
                    (
                        _,
                        Seen {
                            even: Some(ev),
                            odd: _,
                        },
                    ) =>
                        if *ev <= max && even {
                            "O"
                        } else {
                            "."
                        },
                    (b'S', _) => "S",
                    (b'.', _) => ".",
                    _ => unreachable!(),
                }
            );
        });
        println!();
    });
}

fn explore(map: MapRef, seen: SeenMapRefMut, pos: (usize, usize), remaining_steps: usize) {
    if remaining_steps % 2 == 0 {
        seen[pos.1][pos.0].even = Some(remaining_steps);
    } else {
        seen[pos.1][pos.0].odd = Some(remaining_steps);
    }
    //println!("Now at pos {pos:?}, remaining: {remaining_steps}");
    if remaining_steps == 0 {
        return;
    }
    let ipos = (pos.0 as isize, pos.1 as isize);
    for d in [(0, 1), (0, -1), (1, 0), (-1, 0)].into_iter() {
        let inext = (ipos.0 + d.0, ipos.1 + d.1);
        let next = (inext.0 as usize, inext.1 as usize);
        if inext.0 < 0 || next.0 >= map[0].len() || inext.1 < 0 || next.1 >= map.len() {
            continue;
        }
        if map[next.1][next.0] == b'#' {
            continue;
        }
        let is_even = (remaining_steps - 1) % 2 == 0;
        let s = &seen[next.1][next.0];
        let go = if is_even {
            match s.even {
                Some(ev) => ev < remaining_steps - 1,
                None => true,
            }
        } else {
            match s.odd {
                Some(od) => od < remaining_steps - 1,
                None => true,
            }
        };
        if go {
            explore(map, seen, next, remaining_steps - 1);
        }
    }
}
fn even_odd_rhombus_squares(a: usize) -> (usize, usize) {
    println!("a={a} a/2 = {}", a / 2);
    let even = 1 + (1..=a / 2).map(|n| 4 * (2 * n)).sum::<usize>();
    let odd: usize = (0..(a + 1) / 2).map(|n| 4 * (1 + 2 * n)).sum();
    (even, odd)
}

#[test]
fn even_odd_test() {
    for (v, res) in [(1, (1, 4)), (2, (1 + 8, 4)), (3, (1 + 8, 4 + 12))].into_iter() {
        assert_eq!(
            even_odd_rhombus_squares(v),
            res,
            "wrong result for {v}, expected {res:?}"
        );
    }
}

fn part2(map: MapRef, steps: usize) -> usize {
    assert_eq!(map.len(), map[0].len());
    let start_dist = (map.len() - 1) / 2;
    assert_eq!((steps - start_dist) % map.len(), 0);
    // we remove the start steps (65) and skip the central map.
    let full_maps_1dir = (steps - start_dist) / map.len() - 1; //remove corner
    let full_maps_1quadrant = full_maps_1dir * (full_maps_1dir + 1) / 2;
    let full_maps = full_maps_1quadrant * 4 + 1;
    let (full_maps_even, full_maps_odd) = even_odd_rhombus_squares(full_maps_1dir);

    assert_eq!(full_maps, full_maps_even + full_maps_odd);
    let mut seen = vec![vec![Seen::default(); map[0].len()]; map.len()];
    let spos = map
        .iter()
        .enumerate()
        .flat_map(|(y, l)| l.iter().position(|c| *c == b'S').map(|x| (x, y)))
        .next()
        .expect("S pos");
    assert_eq!(spos, (start_dist, start_dist));
    let end = map.len() - 1;
    //dbg!(&spos);
    explore_full(map, &mut seen, spos);
    let seen_even = seen
        .iter()
        .flatten()
        .filter(|seen_at| seen_at.even.is_some())
        .count();
    let seen_odd = seen
        .iter()
        .flatten()
        .filter(|seen_at| seen_at.odd.is_some())
        .count();
    assert_eq!(seen_odd, area_for(&seen, end, false));
    assert_eq!(seen_even, area_for(&seen, end, true));
    println!("Even full:");
    _print_map_max(map, &seen, end, true);
    println!("Odd full:");
    _print_map_max(map, &seen, end, false);

    let mut seen_edges = HashMap::new();
    (0..map.len()).for_each(|y| {
        (0..map[0].len()).for_each(|x| {
            if ![
                (0, 0),
                (0, start_dist),
                (0, end),
                (start_dist, 0),
                //(start_dist, start_dist),
                (start_dist, end),
                (end, 0),
                (end, start_dist),
                (end, end),
            ]
            .contains(&(x, y))
            {
                return;
            }
            let mut edge = vec![vec![Seen::default(); map[0].len()]; map.len()];
            explore_full(map, &mut edge, (x, y));
            print!("for {x}, {y}; ");
            if [(0, 0), (0, end), (end, 0), (end, end)].contains(&(x, y)) {
                println!("edge 1:");
                _print_map_max(map, &edge, start_dist - 1, false);
                println!("         edge 2:");
                _print_map_max(map, &edge, end + start_dist, true);
            } else {
                println!("corner:");
                _print_map_max(map, &edge, end, true);
            }
            seen_edges.insert((x, y), edge);
        });
    });
    println!("there are {full_maps} = {full_maps_odd} odds + {full_maps_even} even square maps in the rhombus");
    println!("In full square maps: {seen_odd} odd and {seen_even} even");
    let mega_rhombus_fullmaps_area = seen_odd * full_maps_odd + seen_even * full_maps_even;
    let mega_rhombus_corners_area = area_for(&seen_edges[&(start_dist, 0)], end, true)
        + area_for(&seen_edges[&(end, start_dist)], end, true)
        + area_for(&seen_edges[&(start_dist, end)], end, true)
        + area_for(&seen_edges[&(0, start_dist)], end, true);
    let mega_rhombus_edges_area = /* edges
       */ area_for(&seen_edges[&(0, 0)], start_dist-1, false) * (full_maps_1dir + 1)
        + area_for(&seen_edges[&(0, 0)], end + start_dist, true) * full_maps_1dir

        + area_for(&seen_edges[&(0, end)], start_dist-1, false) * (full_maps_1dir + 1)
        + area_for(&seen_edges[&(0, end)], end + start_dist, true) * full_maps_1dir

        + area_for(&seen_edges[&(end, 0)], start_dist-1, false) * (full_maps_1dir + 1)
        + area_for(&seen_edges[&(end, 0)], end + start_dist, true) * full_maps_1dir

        + area_for(&seen_edges[&(end, end)], start_dist-1, false) * (full_maps_1dir + 1)
        + area_for(&seen_edges[&(end, end)], end + start_dist, true) * full_maps_1dir;
    mega_rhombus_fullmaps_area + mega_rhombus_corners_area + mega_rhombus_edges_area
}

fn area_for(seen: SeenMapRef, steps: usize, even: bool) -> usize {
    seen.iter()
        .flatten()
        .filter(|seen_at| {
            let s = if even { &seen_at.even } else { &seen_at.odd };
            if let Some(count) = *s {
                count <= steps
            } else {
                false
            }
        })
        .count()
}

fn explore_full(map: MapRef, seen: SeenMapRefMut, pos: (usize, usize)) {
    let mut queue = BinaryHeap::new();

    queue.push((0_isize, pos));

    while let Some((total_steps, pos)) = queue.pop() {
        let total_steps = (-total_steps) as usize;
        //println!("{total_steps}: Now at pos {pos:?}");
        let ipos = (pos.0 as isize, pos.1 as isize);
        for d in [(0, 1), (0, -1), (1, 0), (-1, 0)].into_iter() {
            let inext = (ipos.0 + d.0, ipos.1 + d.1);
            let next = (inext.0 as usize, inext.1 as usize);
            if inext.0 < 0 || next.0 >= map[0].len() || inext.1 < 0 || next.1 >= map.len() {
                continue;
            }
            if map[next.1][next.0] == b'#' {
                continue;
            }
            let next_steps = total_steps + 1;
            let is_even = next_steps % 2 == 0;
            let s = &seen[next.1][next.0];
            let go = if is_even {
                match s.even {
                    Some(ev) => ev > next_steps,
                    None => true,
                }
            } else {
                match s.odd {
                    Some(od) => od > next_steps,
                    None => true,
                }
            };
            if go {
                //println!("{next_steps}: going to {next:?} {s:?}");
                if next_steps % 2 == 0 {
                    seen[next.1][next.0].even = Some(next_steps);
                } else {
                    seen[next.1][next.0].odd = Some(next_steps);
                }
                queue.push((-(next_steps as isize), next));
            }
        }
    }
}

/*
#[test]
fn test_full_input() {
    let map = parse(input!());
    assert_eq!(3574, part1(&map, 64),);
}
*/

#[test]
fn test() {
    let map = parse(sample!());
    //part 1
    for (steps, res) in [(2, 4), (3, 6), (6, 16)].into_iter() {
        assert_eq!(
            res,
            part1(&map, steps),
            "At {steps} steps, expected {res} positions"
        );
    }
    //part 2
    /*
    for (steps, res) in [
        (6, 16),
        (10, 50),
        (50, 1594),
        (100, 6536),
        (500, 167004),
        (1000, 668697),
        (5000, 16733044),
    ]
    .into_iter()
    {
        assert_eq!(res, part2(&map, steps));
    }
    */
    //part 2 custom test case
    let map_small = parse(
        ".....
...#.
..S..
.#...
.....
",
    );
    let map_big = parse(
        ".........................
...#....#....#....#....#.
.........................
.#....#....#....#....#...
.........................
.........................
...#....#....#....#....#.
.........................
.#....#....#....#....#...
.........................
.........................
...#....#....#....#....#.
............S............
.#....#....#....#....#...
.........................
.........................
...#....#....#....#....#.
.........................
.#....#....#....#....#...
.........................
.........................
...#....#....#....#....#.
.........................
.#....#....#....#....#...
.........................",
    );

    assert_eq!(part1(&map_big, 12), part2(&map_small, 12));
}
