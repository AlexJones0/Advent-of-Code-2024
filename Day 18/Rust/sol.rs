/*
 * FILE: Day 18/Rust/sol.rs
 * Author: Alex Jones
 * Desc: Solution to day 18 problems (35 & 36) for Advent of Code 2024, solved in Rust.
 */
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs;

type Coord = (i32, i32);
type Dir = (i32, i32);

const GRID_HEIGHT: usize = 71;
const GRID_WIDTH: usize = 71;
const START: Coord = (0, 0);
const END: Coord = (GRID_WIDTH as i32 - 1, GRID_HEIGHT as i32 - 1);
const CARDINALS: [Dir; 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn neighbours(pos: Coord, walls: &HashSet<Coord>) -> Vec<Coord> {
    CARDINALS
        .iter()
        .map(move |(dx, dy)| (pos.0 + dx, pos.1 + dy))
        .filter(|&(nx, ny)| {
            !walls.contains(&(nx, ny))
                && nx >= 0
                && nx < GRID_WIDTH as i32
                && ny >= 0
                && ny < GRID_HEIGHT as i32
        })
        .collect()
}

fn h_taxicab(p1: Coord, p2: Coord) -> u32 {
    p2.0.abs_diff(p1.0) + p2.1.abs_diff(p1.1)
}

fn min_steps_a_star(start: Coord, end: Coord, walls: &HashSet<Coord>) -> Option<u32> {
    let mut to_explore = BinaryHeap::from([(0, start)]);
    let mut min_cost = HashMap::from([(start, 0)]);
    let mut explored = HashSet::new();
    while let Some((_, pos)) = to_explore.pop() {
        if explored.contains(&pos) {
            continue;
        }
        explored.insert(pos);
        if pos == end {
            return Some(*min_cost.get(&pos).unwrap());
        }
        for ngb in neighbours(pos, walls) {
            let path_len = min_cost.get(&pos).unwrap() + 1;
            if !min_cost.contains_key(&ngb) || path_len < *min_cost.get(&ngb).unwrap() {
                min_cost.insert(ngb, path_len);
                let h_cost = path_len + h_taxicab(ngb, end);
                to_explore.push((-(h_cost as i32), ngb));
            }
        }
    }
    None
}

fn bin_search_for_first_blocking(start: Coord, end: Coord, data: &[Coord]) -> String {
    let mut left = 1024;
    let mut right = data.len();
    while left < right {
        let mid = (left + right) / 2;
        if min_steps_a_star(
            start,
            end,
            &HashSet::from_iter(data.iter().take(mid).cloned()),
        )
        .is_none()
        {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    data[right - 1].0.to_string() + "," + &data[right - 1].1.to_string()
}

pub fn solve() {
    let contents: String = fs::read_to_string("Day 18/data.txt")
        .expect("Could not read data file")
        .trim()
        .replace("\r", "");
    let data: Vec<Coord> = contents
        .split("\n")
        .map(|coord| {
            let mut coords = coord.split(",");
            (
                coords.next().unwrap().parse().unwrap(),
                coords.next().unwrap().parse().unwrap(),
            )
        })
        .collect();

    println!(
        "Problem 35: {}",
        min_steps_a_star(
            START,
            END,
            &HashSet::from_iter(data.iter().take(1024).cloned())
        )
        .unwrap()
    );

    println!(
        "Problem 36: {}",
        bin_search_for_first_blocking(START, END, &data)
    );
}
