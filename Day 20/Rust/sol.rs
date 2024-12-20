/*
 * FILE: Day 20/Rust/sol.rs
 * Author: Alex Jones
 * Desc: Solution to day 20 problems (39 & 40) for Advent of Code 2024, solved in Rust.
 *
 * Comments:
 * Optimised for runtime today, when removing File IO and basic newline parsing this
 * takes around 12 ms (0.012 s) when compiled in release using rustc on my system.
 */
use std::fs;

type Pos = (i32, i32);
type Dir = (i32, i32);

const UNIT_CARDINALS: [Dir; 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn find_loc(grid: &[&str], target: u8) -> Option<Pos> {
    let mut loc = None;
    for (i, row) in grid.iter().enumerate() {
        for (j, c) in row.bytes().enumerate() {
            if c == target {
                loc = Some((i as i32, j as i32));
            }
        }
    }
    loc
}

fn in_bounds(grid: &[&str], pos: Pos) -> bool {
    0 <= pos.0 && (pos.0 as usize) < grid.len() && 0 <= pos.1 && (pos.1 as usize) < grid[0].len()
}

fn find_path_len_from(grid: &[&str], start: Pos) -> (Vec<i32>, Vec<Pos>) {
    let height = grid.len();
    let width = grid[0].len();
    let mut path_len = 0;
    let mut dist_from = vec![-1; height * width];
    let mut nodes = Vec::with_capacity(height * width);
    let mut w_pred = None;
    let mut w_node = Some(start);
    while let Some(node) = w_node {
        let index = (node.0 as usize) * width + (node.1 as usize);
        dist_from[index] = path_len;
        nodes.push(node);
        let mut ngb = None;
        for dir in UNIT_CARDINALS {
            let next = (node.0 + dir.0, node.1 + dir.1);
            let is_pred = w_pred.is_some() && w_pred.unwrap() == next;
            if !is_pred
                && in_bounds(grid, next)
                && grid[next.0 as usize].as_bytes()[next.1 as usize] != b'#'
            {
                ngb = Some(next);
                break;
            }
        }
        w_pred = w_node;
        w_node = ngb;
        path_len += 1;
    }
    (dist_from, nodes)
}

fn find_num_cheats(grid: &[&str], start: Pos, cheat_length: usize, cutoff: usize) -> u64 {
    let height = grid.len();
    let width = grid.len();
    let (start_dist, nodes) = find_path_len_from(grid, start);
    let mut count = 0;
    for (y1, x1) in nodes {
        let dist_1 = start_dist[(y1 as usize) * width + (x1 as usize)];
        let min_y2 = 0.max(y1 - cheat_length as i32) as usize;
        let max_y2 = height.min(y1 as usize + cheat_length + 1);
        for y2 in min_y2..max_y2 {
            let dy = y2.abs_diff(y1 as usize);
            let min_x2 = 0.max(x1 - cheat_length as i32 + dy as i32) as usize;
            let max_x2 = height.min(x1 as usize + cheat_length + 1 - dy);
            for x2 in min_x2..max_x2 {
                let dist_2 = start_dist[y2 * width + x2];
                if dist_2 - (cutoff as i32) <= dist_1 {
                    continue;
                }
                let taxicab = dy + x2.abs_diff(x1 as usize);
                let moves_saved = (dist_2 as usize) - (dist_1 as usize) - taxicab;
                count += (moves_saved >= cutoff) as u64;
            }
        }
    }
    count
}

pub fn solve() {
    let contents: String = fs::read_to_string("Day 20/data.txt")
        .expect("Could not read data file")
        .trim()
        .replace("\r", "");
    let data: Vec<&str> = contents.split("\n").collect();

    let start = find_loc(&data, b'S').unwrap();
    println!("Problem 39: {}", find_num_cheats(&data, start, 2, 100));
    println!("Problem 40: {}", find_num_cheats(&data, start, 20, 100));
}
