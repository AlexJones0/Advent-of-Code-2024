/*
 * FILE: Day 16/Rust/sol.rs
 * Author: Alex Jones
 * Desc: Solution to day 16 problems (31 & 32) for Advent of Code 2024, solved in Rust.
 */
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

type Pos = (usize, usize);
type Dir = (i32, i32);
type State = (Pos, Dir);
type Move = (u32, State);

fn find_loc(grid: &[&str], target: u8) -> Option<Pos> {
    let mut loc = None;
    for (i, row) in grid.iter().enumerate() {
        for (j, c) in row.bytes().enumerate() {
            if c == target {
                loc = Some((i, j));
            }
        }
    }
    loc
}

fn left(dir: Dir) -> Option<Dir> {
    match dir {
        (-1, 0) => Some((0, -1)),
        (0, -1) => Some((1, 0)),
        (1, 0) => Some((0, 1)),
        (0, 1) => Some((-1, 0)),
        _ => None,
    }
}

fn right(dir: Dir) -> Option<Dir> {
    match dir {
        (-1, 0) => Some((0, 1)),
        (0, 1) => Some((1, 0)),
        (1, 0) => Some((0, -1)),
        (0, -1) => Some((-1, 0)),
        _ => None,
    }
}

fn in_bounds(grid: &[&str], pos: Pos) -> bool {
    pos.0 < grid.len() && pos.1 < grid[0].len()
}

fn moves(grid: &Vec<&str>, pos: Pos, dir: Dir) -> Vec<Move> {
    let dirs = [dir, left(dir).unwrap(), right(dir).unwrap()];
    dirs.iter()
        .map(|&next_dir| {
            let next = (
                (pos.0 as i32 + next_dir.0) as usize,
                (pos.1 as i32 + next_dir.1) as usize,
            );
            (next, next_dir)
        })
        .filter(|(next, _)| in_bounds(grid, *next) && grid[next.0].as_bytes()[next.1] != b'#')
        .map(|state| (1 + (state.1 != dir) as u32 * 1000, state))
        .collect()
}

fn get_nodes_on_paths(preds: &HashMap<State, Vec<State>>, end: State) -> HashSet<Pos> {
    let mut nodes = Vec::from([end]);
    let mut seen = HashSet::new();
    while !nodes.is_empty() {
        let mut next = Vec::<State>::new();
        for node in nodes {
            if seen.contains(&node) {
                continue;
            }
            seen.insert(node);
            if let Some(predecessors) = preds.get(&node) {
                next.extend(predecessors);
            }
        }
        nodes = next;
    }
    HashSet::<Pos>::from_iter(seen.iter().map(|(pos, _)| *pos))
}

fn dijkstra(grid: &Vec<&str>, start: Pos, dir: Dir, end: Pos) -> (u32, usize) {
    let mut queue = BinaryHeap::from([(0i32, (start, dir))]);
    let mut lowest_cost = HashMap::from([((start, dir), 0)]);
    let mut preds = HashMap::<State, Vec<State>>::new();
    let mut explored = HashSet::<State>::new();
    let mut on_end_path = HashSet::<Pos>::new();
    let mut best = u32::MAX;
    while let Some((cost, state)) = queue.pop() {
        let cost = (-cost) as u32;
        if cost > best {
            continue;
        }
        if state.0 == end {
            best = cost;
            on_end_path.extend(get_nodes_on_paths(&preds, state));
        }
        if explored.contains(&state) {
            continue;
        }
        explored.insert(state);
        for (cost, ngb) in moves(grid, state.0, state.1) {
            let new_cost = lowest_cost.get(&state).unwrap() + cost;
            let current_lowest = *lowest_cost.get(&ngb).unwrap_or(&u32::MAX);
            if current_lowest > new_cost {
                preds.insert(ngb, Vec::new());
                lowest_cost.insert(ngb, new_cost);
            }
            if current_lowest >= new_cost {
                preds.get_mut(&ngb).unwrap().push(state);
                queue.push((-(new_cost as i32), ngb));
            }
        }
    }
    (best, on_end_path.len())
}

pub fn solve() {
    let contents: String = fs::read_to_string("Day 16/data.txt")
        .expect("Could not read data file")
        .trim()
        .replace("\r", "");
    let data: Vec<&str> = contents.split("\n").collect();

    let start = find_loc(&data, b'S').unwrap();
    let end = find_loc(&data, b'E').unwrap();
    let (p1, p2) = dijkstra(&data, start, (0, 1), end);
    println!("Problem 31: {}", p1);
    println!("Problem 32: {}", p2);
}
