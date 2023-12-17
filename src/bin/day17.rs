use std::collections::VecDeque;

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
    let state = State {
        pos: Pos { row: 0, col: 0 },
        cost: 0,
        dir: UP,
        //last_cur_dir: 0,
    };
    shortest_path_three(map, &state)
}
fn shortest_path_three(map: MapRef, state: &State) -> usize {
    let mut cost_map = vec![vec![usize::MAX; map[0].len()]; map.len()];
    let mut queue: VecDeque<State> = VecDeque::new();
    let mut mincost = usize::MAX;
    let end = Pos {
        row: map.len() - 1,
        col: map[0].len() - 1,
    };

    queue.push_back(state.clone());

    while let Some(cur) = queue.pop_front() {
        /*
        println!(
            "Now evaluating pos {:?}, dir: {}, cost is {}",
            cur.pos,
            cur.dir,
            cur.cost, //cur.last_cur_dir,
        );
        */
        let cost_pos = cost_map[cur.pos.row][cur.pos.col];
        if cost_pos < cur.cost {
            continue;
        }
        cost_map[cur.pos.row][cur.pos.col] = cur.cost;
        if cur.pos == end {
            //println!("END REACHED\n===============================\n");
            if cur.cost < mincost {
                mincost = cur.cost;
            }
            continue;
        }
        for dir in 0..4 {
            if dir == cur.dir {
                continue;
            }
            let mut p1 = cur.pos.clone();
            let mut cost = cur.cost;
            for _ in 0..3 {
                if let Some(pos) = next_pos(&p1, dir, map) {
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
                    //println!("For pos {pos:?}, adding cost {}", map[pos.row][pos.col]);
                    cost += map[pos.row][pos.col] as usize;
                    queue.push_back(State {
                        pos: pos.clone(),
                        dir,
                        cost,
                        //last_cur_dir,
                    });
                    p1 = pos;
                } else {
                    break;
                }
            }
        }
    }
    mincost
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
fn next_pos(pos: &Pos, dir: u8, map: MapRef) -> Option<Pos> {
    let rows = map.len() as isize;
    let cols = map[0].len() as isize;
    let inc = [(0, 1), (1, 0), (0, -1), (-1, 0)][dir as usize];
    //println!("inc is {inc:?}, dir is {dir}");
    let new_row = pos.row as isize + inc.0;
    let new_col = pos.col as isize + inc.1;
    if new_row < 0 || new_row == rows || new_col < 0 || new_col == cols {
        return None;
    }
    Some(Pos {
        row: new_row as usize,
        col: new_col as usize,
    })
}

#[derive(Debug, Clone)]
struct State {
    pos: Pos,
    cost: usize,
    dir: u8,
    //last_cur_dir: u8,
}

/*
#[derive(Debug, Clone)]
struct Cost {
    c: [u32; 12],
}
impl Default for Cost {
    fn default() -> Self {
        Cost { c: [u32::MAX; 12] }
    }
}
impl Cost {
    fn current(&self, dir: u8, last_cur_dir: u8) -> usize {
        assert!(dir < 4);
        assert!(last_cur_dir < 3);
        self.c[dir as usize * 3 + last_cur_dir as usize] as usize
    }
    fn set(&mut self, dir: u8, last_cur_dir: u8, val: usize) {
        assert!(dir < 4);
        assert!(last_cur_dir < 3);
        assert!(val < u32::MAX as usize);
        self.c[dir as usize * 3 + last_cur_dir as usize] = val as u32;
    }
}
*/
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
    for _ in map {
        todo!()
    }
    42
}

#[test]
fn test() {
    let map = parse(sample!());
    //part 1
    let res = part1(&map);
    assert_eq!(res, 102);
    //part 2
    /*
    let res = part2(&map);
    assert_eq!(res, 42);
    */
}
