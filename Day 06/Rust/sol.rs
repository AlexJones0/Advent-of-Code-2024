/*
 * FILE: Day 06/Rust/sol.rs
 * Author: Alex Jones
 * Desc: Solution to day 6 problems (11 & 12) for Advent of Code 2024, solved in Rust.
 */
use std::collections::HashSet;
use std::fs;

use itertools::{enumerate, Itertools};

fn turn_right(dir: (i32, i32)) -> (i32, i32) {
    match dir {
        (-1, 0) => (0, 1),
        (0, 1) => (1, 0),
        (1, 0) => (0, -1),
        (0, -1) => (-1, 0),
        other => other,
    }
}

pub fn pos_in_bounds(dims: (usize, usize), pos: (i32, i32)) -> bool {
    0 <= pos.0 && 0 <= pos.1 && (dims.0 as i32) > pos.0 && (dims.1 as i32) > pos.1
}

type State = ((i32, i32), (i32, i32));

pub fn simulate(
    dims: (usize, usize),
    obstacles: &HashSet<(usize, usize)>,
    mut pos: (i32, i32),
    mut dir: (i32, i32),
    visited: &mut Vec<State>,
) -> bool {
    let mut seen = HashSet::<State>::from_iter(visited.iter().cloned());
    while pos_in_bounds(dims, pos) && !seen.contains(&(pos, dir)) {
        // Maintain a `visited` path to allow shorter simulations in part 2
        visited.push((pos, dir));
        seen.insert((pos, dir));
        let next_pos = (pos.0 + dir.0, pos.1 + dir.1);
        if obstacles.contains(&((next_pos.0 as usize), (next_pos.1 as usize))) {
            dir = turn_right(dir);
            continue;
        }
        pos = next_pos;
    }
    !seen.contains(&(pos, dir))
}

pub fn solve() {
    let contents: String = fs::read_to_string("Day 06/data.txt")
        .expect("Could not read data file")
        .trim()
        .replace("\r", "");
    let data: Vec<&str> = contents.split("\n").collect();

    let mut obstacles: HashSet<(usize, usize)> = HashSet::new();
    let mut guard: Option<(usize, usize)> = None;
    for (i, row) in enumerate(&data) {
        for (j, col) in enumerate(row.bytes()) {
            match col {
                b'^' => guard = Some((i, j)),
                b'#' => drop(obstacles.insert((i, j))),
                _ => (),
            }
        }
    }
    let dims = (data.len(), data[0].len());
    let guard = guard.expect("No guard found?");

    let mut path: Vec<State> = Vec::new();
    assert!(simulate(
        dims,
        &obstacles,
        (guard.0 as i32, guard.1 as i32),
        (-1, 0),
        &mut path
    ));
    println!("Problem 11: {}", path.iter().map(|x| x.0).unique().count());

    let mut ans = 0u32;
    let mut visited = HashSet::<(i32, i32)>::new();
    // Opt 1: Only ned to check placing obstacles on our path
    for i in 1..path.len() {
        let pos = path[i].0;
        let (prev_pos, prev_dir) = path[i - 1];
        // Only check each position once, the first time encountered
        if pos == prev_pos || visited.contains(&pos) {
            continue;
        }
        visited.insert(pos);
        // Opt 2: simulate restoring from just before the point of collision
        let mut new_obstacles = obstacles.clone();
        new_obstacles.insert((pos.0 as usize, pos.1 as usize));
        let mut prev_path = vec![((0, 0), (0, 0)); i - 1];
        prev_path.clone_from_slice(&path[..(i - 1)]);
        ans += !simulate(dims, &new_obstacles, prev_pos, prev_dir, &mut prev_path) as u32;
    }
    println!("Problem 12: {}", ans);
}
