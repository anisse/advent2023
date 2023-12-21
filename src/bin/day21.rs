use advent2023::*;
fn main() {
    let map = parse(input!());
    //part 1
    let res = part1(&map, 64);
    println!("Part 1: {}", res);
    //part 2
    let res = part2(&map);
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
    //let mut reachable = vec![vec![false; map[0].len()]; map.len()];
    let spos = map
        .iter()
        .enumerate()
        .flat_map(|(y, l)| l.iter().position(|c| *c == b'S').map(|x| (x, y)))
        .next()
        .expect("S pos");
    dbg!(&spos);
    explore(map, &mut seen, spos, steps);
    _print_map(map, &seen);
    seen.iter()
        .flatten()
        .filter(|seen_at| seen_at.even.is_some())
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

fn explore(map: MapRef, seen: SeenMapRefMut, pos: (usize, usize), remaining_steps: usize) {
    if remaining_steps % 2 == 0 {
        seen[pos.1][pos.0].even = Some(remaining_steps);
    } else {
        seen[pos.1][pos.0].odd = Some(remaining_steps);
    }
    println!("Now at pos {pos:?}, remaining: {remaining_steps}");
    if remaining_steps == 0 {
        return;
    }
    let ipos = (pos.0 as isize, pos.1 as isize);
    for d in [(0, 1), (0, -1), (1, 0), (-1, 0)].into_iter() {
        let inext = (ipos.0 + d.0, ipos.1 + d.1);
        let next = (inext.0 as usize, inext.1 as usize);
        if d.0 == -1 {
            println!("{remaining_steps}: x - 1: {inext:?}");
        }
        if inext.0 < 0 || next.0 >= map[0].len() || inext.1 < 0 || next.1 >= map.len() {
            continue;
        }
        if map[next.1][next.0] == b'#' {
            continue;
        }
        /*
        if let Seen {
            even: None,
            odd: None,
        } = seen[next.1][next.0]
        {
            explore(map, seen, next, remaining_steps - 1);
        }
        */
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

fn part2(map: MapRef) -> usize {
    for _ in map {
        todo!()
    }
    42
}

#[test]
fn test() {
    let map = parse(sample!());
    //part 1
    let res = part1(&map, 2);
    assert_eq!(res, 4);
    //let res = part1(&map, 3);
    //assert_eq!(res, 6);
    let res = part1(&map, 6);
    assert_eq!(res, 16);
    //part 2
    //let res = part2(&map);
    //assert_eq!(res, 42);
}
