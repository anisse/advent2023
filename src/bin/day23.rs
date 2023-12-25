use std::collections::HashSet;

use advent2023::*;
fn main() {
    let map = parse(input!());
    //part 1
    let res = part1(&map);
    println!("Part 1: {}", res);
    //part 2
    let res = part2(&map);
    println!("Part 2: {}", res);
}
type Map = Vec<Vec<u8>>;
type MapRef<'a> = &'a [Vec<u8>];

type Pos = (u16, u16);

fn parse(input: &str) -> Map {
    input.lines().map(|x| x.as_bytes().to_vec()).collect()
}
fn part1(map: MapRef) -> usize {
    let start_x = map[0].iter().position(|c| *c == b'.').expect("start pos");
    let end_x = map[map.len() - 1]
        .iter()
        .position(|c| *c == b'.')
        .expect("end pos");
    longest_path(
        map,
        (start_x as u16, 0),
        (end_x as u16, map.len() as u16 - 1),
        &mut HashSet::new(),
    )
}

fn longest_path(map: MapRef, pos: Pos, end: Pos, current_path: &mut HashSet<Pos>) -> usize {
    if pos == end {
        return current_path.len();
    }
    let ipos = (pos.0 as isize, pos.1 as isize);
    let mut max = 0;
    for d in [(0, 1), (0, -1), (1, 0), (-1, 0)].into_iter() {
        let inext = (ipos.0 + d.0, ipos.1 + d.1);
        let next = (inext.0 as u16, inext.1 as u16);
        if inext.0 < 0 || next.0 >= map[0].len() as u16 || inext.1 < 0 || next.1 >= map.len() as u16
        {
            continue;
        }
        match (map[next.1 as usize][next.0 as usize], d) {
            (b'.', _) => {}
            (b'v', (0, 1)) => {}
            (b'>', (1, 0)) => {}
            (b'^', (0, -1)) => {}
            (b'<', (-1, 0)) => {}
            _ => continue,
        }
        if current_path.contains(&next) {
            continue;
        }
        current_path.insert(next);
        let len = longest_path(map, next, end, current_path);
        current_path.remove(&next);
        if len > max {
            max = len;
        }
    }
    max
}

fn part2(map: MapRef) -> usize {
    let start_x = map[0].iter().position(|c| *c == b'.').expect("start pos") as u16;
    let end_x = map[map.len() - 1]
        .iter()
        .position(|c| *c == b'.')
        .expect("end pos") as u16;

    let g = build_graph(map, (start_x, 0), (end_x, map.len() as u16 - 1));
    //g._print_dot();
    g.longest_path((start_x, 0), (end_x, map.len() as u16 - 1))
}

#[derive(Debug, Default)]
struct Edge {
    next: usize,
    weight: u16,
}
type Edges = [Option<Edge>; 4];
#[derive(Debug, Default)]
struct Graph {
    names: Vec<u32>,
    edges: Vec<Edges>,
}
impl Graph {
    fn coord_to_name(pos: Pos) -> u32 {
        pos.0 as u32 | (pos.1 as u32) << 16
    }
    fn _name_to_coord(name: u32) -> Pos {
        ((name & 0xFFFF) as u16, (name >> 16) as u16)
    }

    fn node_id(&mut self, node: Pos) -> usize {
        self.names
            .iter()
            .position(|s| *s == Self::coord_to_name(node))
            .unwrap_or_else(|| {
                self.names.push(Self::coord_to_name(node));
                self.edges.push(Default::default());
                self.names.len() - 1
            })
    }
    // can panic
    fn existing_node_id(&self, node: Pos) -> usize {
        self.names
            .iter()
            .position(|s| *s == Self::coord_to_name(node))
            .unwrap()
    }
    fn _print_dot(&self) {
        println!("strict graph {{");
        for (i, name) in self.names.iter().enumerate() {
            for edge in self.edges[i].iter().flatten() {
                println!(
                    "\"{:?}\" -- \"{:?}\" [label={}]",
                    Self::_name_to_coord(*name),
                    Self::_name_to_coord(self.names[edge.next]),
                    edge.weight
                );
            }
        }
        println!("}}");
    }
    fn longest_path(&self, start: Pos, end: Pos) -> usize {
        let mut seen = vec![false; self.edges.len()];
        self.longest_path_inner(
            self.existing_node_id(start),
            self.existing_node_id(end),
            &mut seen,
            0,
        )
    }
    fn longest_path_inner(&self, pos: usize, end: usize, seen: &mut [bool], cur: u16) -> usize {
        if pos == end {
            return cur as usize;
        }
        let mut max = 0;
        for next in self.edges[pos].iter().flatten() {
            if seen[next.next] {
                continue;
            }
            seen[next.next] = true;
            let len = self.longest_path_inner(next.next, end, seen, cur + next.weight);
            seen[next.next] = false;
            if len > max {
                max = len;
            }
        }
        max
    }
}
fn build_graph(map: MapRef, start: Pos, end: Pos) -> Graph {
    let mut graph = Graph::default();
    let mut queue = vec![];
    let mut seen = vec![vec![false; map[0].len()]; map.len()];
    queue.push(start);
    while let Some(pos) = queue.pop() {
        let mut i = 0;
        let mut conns = Edges::default();
        seen[pos.1 as usize][pos.0 as usize] = true;
        for next in adj(map, pos) {
            let (next, weight, dead_end) = next_compress(map, pos, next);
            if dead_end && next != end {
                //println!("Dead end edge {pos:?} skipped");
                continue;
            }
            conns[i] = Some(Edge {
                next: graph.node_id(next),
                weight,
            });
            i += 1;
            if !seen[next.1 as usize][next.0 as usize] {
                queue.push(next);
            }
        }
        let id = graph.node_id(pos);
        graph.edges[id] = conns;
    }
    graph
}

fn adj(map: MapRef, pos: Pos) -> impl Iterator<Item = Pos> + '_ {
    [(0, 1), (0, -1), (1, 0), (-1, 0)]
        .into_iter()
        .flat_map(move |d| {
            // inline iterator
            let ipos = (pos.0 as isize, pos.1 as isize);
            let inext = (ipos.0 + d.0, ipos.1 + d.1);
            let next = (inext.0 as usize, inext.1 as usize);
            if inext.0 < 0 || next.0 >= map[0].len() || inext.1 < 0 || next.1 >= map.len() {
                return None;
            }
            if map[next.1][next.0] == b'#' {
                return None;
            }
            Some((next.0 as u16, next.1 as u16))
        })
}
fn next_compress(map: MapRef, mut prev: Pos, mut next: Pos) -> (Pos, u16, bool) {
    let mut weight = 1;
    loop {
        let edges: Vec<_> = adj(map, next).collect();
        match edges.len() {
            2 => {
                let ni = if edges[0] == prev { 1 } else { 0 };
                prev = next;
                next = edges[ni];
                weight += 1;
            }
            1 => {
                // dead end
                //
                return (next, weight, true);
            }
            _ => return (next, weight, false),
        }
    }
}

fn _print_map(map: MapRef, current_path: &mut HashSet<Pos>) {
    map.iter().enumerate().for_each(|(y, l)| {
        l.iter().enumerate().for_each(|(x, c)| {
            if current_path.contains(&(x as u16, y as u16)) {
                print!("O")
            } else {
                print!("{}", *c as char)
            }
        });
        println!();
    });
}

#[test]
fn test() {
    let map = parse(sample!());
    //part 1
    let res = part1(&map);
    assert_eq!(res, 94);
    //part 2
    let res = part2(&map);
    assert_eq!(res, 154);
}

#[cfg(not(feature = "ci_no_input"))]
#[test]
fn test_real_input() {
    let map = parse(input!());
    let res = part2(&map);
    assert_eq!(res, 6258);
}

#[test]
fn test_custom_map() {
    let map = parse(concat!(
        //start
        "#.###\n", ".....\n", //"#.#.#\n",
        // ".....\n",
        ".....\n", //end
        "###.#\n",
    ));
    assert_eq!(part1(&map), part2(&map));
}
