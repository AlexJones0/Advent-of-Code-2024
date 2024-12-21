/*
 * FILE: Day 21/Rust/sol.rs
 * Author: Alex Jones
 * Desc: Solution to day 21 problems (41 & 42) for Advent of Code 2024, solved in Rust.
 */
use itertools::Itertools;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fs;

type Pos = (usize, usize);

fn find_pos(char: u8, is_directional: bool) -> Pos {
    // Manual mapping with match/case to avoid runtime overhead of a couple
    // dozen microseconds from using str.iter().position() and divmod.
    if is_directional {
        match char {
            b' ' => (0, 0),
            b'^' => (0, 1),
            b'A' => (0, 2),
            b'<' => (1, 0),
            b'v' => (1, 1),
            b'>' => (1, 2),
            _ => panic!(),
        }
    } else {
        match char {
            b'7' => (0, 0),
            b'8' => (0, 1),
            b'9' => (0, 2),
            b'4' => (1, 0),
            b'5' => (1, 1),
            b'6' => (1, 2),
            b'1' => (2, 0),
            b'2' => (2, 1),
            b'3' => (2, 2),
            b' ' => (3, 0),
            b'0' => (3, 1),
            b'A' => (3, 2),
            _ => panic!(),
        }
    }
}

fn invalid_path(start: Pos, end: Pos, invalid: Pos, dim: usize) -> bool {
    match dim {
        0 => end.0 == invalid.0 && start.1 == invalid.1,
        1 => end.1 == invalid.1 && start.0 == invalid.0,
        _ => panic!(),
    }
}

type Memo = HashMap<(Pos, Pos, usize), u64>;
fn find_shortest(memo: &mut Memo, start: Pos, end: Pos, robot: usize, robots: usize) -> u64 {
    let entry = memo.entry((start, end, robot));
    if let Entry::Occupied(e) = entry {
        return *e.get();
    }
    let delta = (end.0 as i32 - start.0 as i32, end.1 as i32 - start.1 as i32);
    let abs_delta = (
        delta.0.unsigned_abs() as usize,
        delta.1.unsigned_abs() as usize,
    );
    let path_size = abs_delta.0 + abs_delta.1;
    // For robot 1, it just requires moving the taxicab distance plus 1 ('activate')
    if robot == 1 {
        let retval = path_size as u64 + 1;
        memo.insert((start, end, robot), retval);
        return retval;
    }
    let invalid = find_pos(b' ', robot != robots);
    let vertical = if delta.0 < 0 { b'^' } else { b'v' };
    let horizontal = if delta.1 < 0 { b'<' } else { b'>' };
    let mut vert_hori_path = vec![vertical; abs_delta.0];
    vert_hori_path.extend(vec![horizontal; abs_delta.1]);
    let mut hori_vert_path = vec![horizontal; abs_delta.1];
    hori_vert_path.extend(vec![vertical; abs_delta.0]);
    let mut shortest_length = None;
    for verts in (0..path_size).combinations(abs_delta.0) {
        let mut path = vec![horizontal; path_size];
        for i in verts {
            path[i] = vertical;
        }
        // Check it doesn't path through invalid
        if invalid_path(start, end, invalid, 0) && path == vert_hori_path
            || invalid_path(start, end, invalid, 1) && path == hori_vert_path
        {
            continue;
        }
        path.push(b'A');
        // Find the cost of the path by recursively using the next robot to make
        // each button press on the path
        let mut current_pos = find_pos(b'A', true);
        let mut path_len = 0;
        for button in path {
            let pos = find_pos(button, true);
            path_len += find_shortest(memo, current_pos, pos, robot - 1, robots);
            current_pos = pos;
        }
        if shortest_length.is_none() {
            shortest_length = Some(path_len);
        } else {
            shortest_length = Some(shortest_length.unwrap().min(path_len));
        }
    }
    // The shortest length is the shortest length of all possible paths
    let result = shortest_length.unwrap();
    memo.insert((start, end, robot), result);
    result
}

fn calc(memo: &mut Memo, code: &str, robots: usize) -> u64 {
    let mut result = 0;
    let mut start = find_pos(b'A', false);
    for button in code.bytes() {
        let pos = find_pos(button, false);
        result += find_shortest(memo, start, pos, robots, robots);
        start = pos;
    }
    result * code[..code.len() - 1].parse::<u64>().unwrap()
}

pub fn solve() {
    let contents: String = fs::read_to_string("Day 21/data.txt")
        .expect("Could not read data file")
        .trim()
        .replace("\r", "");
    let data: Vec<&str> = contents.split("\n").collect();

    let mut memoized = Memo::with_capacity(600);
    println!(
        "Problem 41: {}",
        data.iter()
            .map(|code| calc(&mut memoized, code, 3))
            .sum::<u64>()
    );
    println!(
        "Problem 42: {}",
        data.iter()
            .map(|code| calc(&mut memoized, code, 26))
            .sum::<u64>()
    );
}
