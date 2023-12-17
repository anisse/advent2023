use std::collections::BinaryHeap;
use std::collections::HashMap;
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
fn parse(input: &str) -> Map {
    input
        .lines()
        .map(|l| l.as_bytes().iter().map(|x| x - b'0').collect())
        .collect()
}
fn part1(map: MapRef) -> u32 {
    shortest_path_common(map, 0..3)
}
fn shortest_path_common(map: MapRef, move_range: Range<u8>) -> u32 {
    let mut cost_map = HashMap::new();
    let mut queue = BinaryHeap::with_capacity(4096);
    let end = Pos {
        row: map.len() - 1,
        col: map[0].len() - 1,
    };

    // Start at 0, cost 0. Fake as if we were going up (next is right), or left (next is down)
    queue.push(State {
        pos: Pos { row: 0, col: 0 },
        cost: 0,
        dir: UP,
    });
    queue.push(State {
        pos: Pos { row: 0, col: 0 },
        cost: 0,
        dir: LEFT,
    });

    while let Some(cur) = queue.pop() {
        /*
        println!(
            "Now evaluating pos {:?}, dir: {}, cost is {}",
            cur.pos, cur.dir, cur.cost
        );
        */
        if cur.pos == end {
            //println!("END REACHED\n===============================\n");
            return cur.cost;
        }
        /*
        if cur.cost % 100 == 0 {
            println!("Cost {}", cur.cost);
        }
        */
        for dir in 0..4 {
            // Do not continue in the same direction, we already advanced enough
            if dir == cur.dir  ||
            // Do not reverse
                 (dir + 2) % 4 == cur.dir
            {
                continue;
            }
            let mut cost = cur.cost;
            for advance in 0..(move_range.start) {
                // Skip minimum
                if let Some(pos) = next_pos(&cur.pos, dir, map, advance as usize + 1) {
                    cost += map[pos.row][pos.col] as u32;
                } else {
                    break;
                }
            }
            for advance in move_range.clone() {
                if let Some(pos) = next_pos(&cur.pos, dir, map, advance as usize + 1) {
                    cost += map[pos.row][pos.col] as u32;
                    /*
                    println!(
                        "For pos {pos:?} in dir {dir} (step {advance}), adding cost {}, total is {cost}",
                        map[pos.row][pos.col]
                    );
                    */
                    if let Some(next_cost) = cost_map.get(&(pos.clone(), dir, advance)) {
                        if *next_cost < cost {
                            break;
                        }
                    }
                    cost_map.insert((pos.clone(), dir, advance), cost);
                    queue.push(State { pos, dir, cost });
                } else {
                    break;
                }
            }
        }
    }
    unreachable!()
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Pos {
    row: usize,
    col: usize,
}
/*
const RIGHT: u8 = 0;
const DOWN: u8 = 1;
*/
const LEFT: u8 = 2;
const UP: u8 = 3;
fn next_pos(pos: &Pos, dir: u8, map: MapRef, advance: usize) -> Option<Pos> {
    let rows = map.len() as isize;
    let cols = map[0].len() as isize;
    let inc = [(0, 1), (1, 0), (0, -1), (-1, 0)][dir as usize];
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
    cost: u32,
    dir: u8,
}
impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| (other.pos.row + other.pos.col).cmp(&(self.pos.row + self.pos.col)))
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn part2(map: MapRef) -> u32 {
    shortest_path_common(map, 3..10)
}

#[test]
fn test() {
    let map = parse(sample!());
    //part 1
    let res = part1(&map);
    assert_eq!(res, 102);
    //part 2
    let mapsmol = parse(
        "111111111111
999999999991
999999999991
999999999991
999999999991",
    );
    assert_eq!(part2(&mapsmol), 71);
    let res = part2(&map);
    assert_eq!(res, 94, "part 2 is wrong");
}
