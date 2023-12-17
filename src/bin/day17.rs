use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::ops::Range;

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
//type MapRefMut<'a> = &'a mut [Vec<u8>];
fn parse(input: &str) -> Map {
    input
        .lines()
        .map(|l| l.as_bytes().iter().map(|x| x - b'0').collect())
        .collect()
}
fn part1(map: MapRef) -> usize {
    shortest_path_common(map, 0..3)
}
fn shortest_path_common(map: MapRef, move_range: Range<usize>) -> usize {
    let mut cost_map = vec![vec![Cost::default(); map[0].len()]; map.len()];
    let mut queue = BinaryHeap::new();
    //let mut mincost = usize::MAX;
    let end = Pos {
        row: map.len() - 1,
        col: map[0].len() - 1,
    };

    let mut state = State {
        pos: Pos { row: 0, col: 0 },
        cost: 0,
        dir: UP,
        //last_cur_dir: 0,
    };
    queue.push(Reverse(state.clone()));
    state.dir = LEFT;
    queue.push(Reverse(state));

    while let Some(Reverse(cur)) = queue.pop() {
        let cost_pos = &cost_map[cur.pos.row][cur.pos.col];
        if cost_pos.current(cur.dir /*, cur.last_cur_dir*/) < cur.cost {
            continue;
        }
        /*
        println!(
            "Now evaluating pos {:?}, dir: {}, cost is {}",
            cur.pos,
            cur.dir,
            cur.cost, //cur.last_cur_dir,
        );
        */
        cost_map[cur.pos.row][cur.pos.col].set(cur.dir, /*cur.last_cur_dir,*/ cur.cost);
        if cur.pos == end {
            //println!("END REACHED\n===============================\n");
            return cur.cost;
            /*
            if cur.cost < mincost {
                mincost = cur.cost;
            }
            continue;
            */
        }
        if cur.cost % 25 == 0 {
            println!("Cost {}", cur.cost);
        }
        for dir in 0..4 {
            if dir == cur.dir || (dir + 2) % 4 == cur.dir {
                continue;
            }
            let mut cost = cur.cost;
            for advance in 0..(move_range.start) {
                if let Some(pos) = next_pos(&cur.pos, dir, map, advance + 1) {
                    cost += map[pos.row][pos.col] as usize;
                } else {
                    break;
                }
            }
            for advance in move_range.clone() {
                if let Some(pos) = next_pos(&cur.pos, dir, map, advance + 1) {
                    /*
                    let last_cur_dir = if dir == cur.dir {
                    cur.last_cur_dir + 1
                    } else {
                    0
                    };
                    if last_cur_dir > 2 {
                    continue;
                    }
                    */
                    cost += map[pos.row][pos.col] as usize;
                    /*
                    println!(
                        "For pos {pos:?} in dir {dir} (step {advance}, adding cost {}, total is {cost}",
                        map[pos.row][pos.col]
                    );
                    */
                    queue.push(Reverse(State {
                        pos: pos.clone(),
                        dir,
                        cost,
                        //last_cur_dir,
                    }));
                } else {
                    break;
                }
            }
        }
    }
    0
    //mincost
}
#[derive(Debug, Clone, PartialEq, Eq)]
struct Pos {
    row: usize,
    col: usize,
}
const RIGHT: u8 = 0;
const DOWN: u8 = 1;
const LEFT: u8 = 2;
const UP: u8 = 3;
fn next_pos(pos: &Pos, dir: u8, map: MapRef, advance: usize) -> Option<Pos> {
    let rows = map.len() as isize;
    let cols = map[0].len() as isize;
    let inc = [(0, 1), (1, 0), (0, -1), (-1, 0)][dir as usize];
    //println!("inc is {inc:?}, dir is {dir}");
    let new_row = pos.row as isize + inc.0 * advance as isize;
    let new_col = pos.col as isize + inc.1 * advance as isize;
    if new_row < 0 || new_row >= rows || new_col < 0 || new_col >= cols {
        return None;
    }
    Some(Pos {
        row: new_row as usize,
        col: new_col as usize,
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct State {
    pos: Pos,
    cost: usize,
    dir: u8,
    //last_cur_dir: u8,
}
impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost)
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone)]
struct Cost {
    c: [u32; 2],
}
impl Default for Cost {
    fn default() -> Self {
        Cost { c: [u32::MAX; 2] }
    }
}
impl Cost {
    fn current(&self, dir: u8 /*last_cur_dir: u8*/) -> usize {
        assert!(dir < 4);
        //assert!(last_cur_dir < 3);
        self.c[dir as usize % 2] /* * 3 + last_cur_dir as usize]*/ as usize
    }
    fn set(&mut self, dir: u8, /*last_cur_dir: u8,*/ val: usize) {
        assert!(dir < 4);
        //assert!(last_cur_dir < 3);
        assert!(val < u32::MAX as usize);
        self.c[dir as usize % 2 /* * 3 + last_cur_dir as usize*/] = val as u32;
    }
}
/*
#[derive(Debug, Clone)]
struct Cost {
    cost: usize,
    dir: u8,
    last_cur_dir: u8,
}
impl Default for Cost {
    fn default() -> Self {
        Cost {
            cost: usize::MAX,
            dir: 5, // nonexistent
            last_cur_dir: 0,
        }
    }
}
*/

fn part2(map: MapRef) -> usize {
    shortest_path_common(map, 3..10)
}

#[test]
fn test() {
    let map = parse(sample!());
    //part 1
    let res = part1(&map);
    assert_eq!(res, 102);
    //part 2
    let res = part2(&map);
    assert_eq!(res, 94, "part 2 is wrong");

    let map = parse(
        "111111111111
999999999991
999999999991
999999999991
999999999991",
    );
    assert_eq!(part2(&map), 71);
}
