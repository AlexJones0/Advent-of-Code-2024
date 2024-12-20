/*
 * FILE: Day 06/Rust/sol.rs
 * Author: Alex Jones
 * Desc: Solution to day 6 problems (11 & 12) for Advent of Code 2024, solved in Rust.
 */
use std::collections::hash_map::Entry;
use std::collections::HashMap;
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

type Memo = HashMap<State, (i32, i32)>;
fn find_next_collision(
    memo: &mut Memo,
    dims: (usize, usize),
    obstacles: &HashSet<(usize, usize)>,
    mut pos: (i32, i32),
    dir: (i32, i32),
) -> (i32, i32) {
    let entry = memo.entry((pos, dir));
    if let Entry::Occupied(e) = entry {
        *e.get()
    } else {
        while pos_in_bounds(dims, pos) && !obstacles.contains(&(pos.0 as usize, pos.1 as usize)) {
            pos = (pos.0 + dir.0, pos.1 + dir.1);
        }
        entry.or_insert(pos);
        pos
    }
}

fn between(item: i32, one: i32, two: i32) -> bool {
    one <= item && item < two || two <= item && item < one
}

fn new_obstacle_blocks_path(
    current_pos: (i32, i32),
    current_dir: (i32, i32),
    new_pos: (i32, i32),
    obstacle: (i32, i32),
) -> bool {
    if current_dir.0 != 0 && current_pos.1 == obstacle.1 {
        between(obstacle.0, current_pos.0, new_pos.0)
    } else if current_dir.1 != 0 && current_pos.0 == obstacle.0 {
        between(obstacle.1, current_pos.1, new_pos.1)
    } else {
        false
    }
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

    let mut memo = Memo::new();

    let mut ans = 0u32;
    let mut visited_on_path = HashSet::new();
    // Optimisation 1: Only try placing obstacles on nodes explored in our original path
    for i in 1..path.len() {
        let pos = path[i].0;
        let (prev_pos, prev_dir) = path[i - 1];
        // Only check each position once, the first time encountered
        if pos == prev_pos || visited_on_path.contains(&pos) {
            continue;
        }
        visited_on_path.insert(pos);
        // Optimisation 2: Restore simulation from just before the point of collision
        let mut current_pos = prev_pos;
        let mut next_pos = prev_pos;
        let mut current_dir = prev_dir;
        let mut seen = HashSet::new();
        while pos_in_bounds(dims, next_pos) && !seen.contains(&(current_pos, current_dir)) {
            seen.insert((current_pos, current_dir));
            // Optimisation 3: Use a memoized global result to automatically find the
            // next collision given the current posiition and direction. Then, just check
            // if the newly inserted obstacle would have been on that collision path - if
            // so, set the new position to be just before the obstacle instead.
            next_pos = find_next_collision(&mut memo, dims, &obstacles, current_pos, current_dir);
            if new_obstacle_blocks_path(current_pos, current_dir, next_pos, pos) {
                next_pos = pos;
            }
            current_pos = (next_pos.0 - current_dir.0, next_pos.1 - current_dir.1);
            current_dir = turn_right(current_dir);
        }
        // Only increment if we stop searching because we found a cycle, instead
        // of finding the grid border.
        ans += pos_in_bounds(dims, next_pos) as u32;
    }
    println!("Problem 12: {}", ans);
}
