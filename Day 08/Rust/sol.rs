/*
 * FILE: Day 08/Rust/sol.rs
 * Author: Alex Jones
 * Desc: Solution to day 8 problems (15 & 16) for Advent of Code 2024, solved in Rust.
 */
use itertools::enumerate;
use std::collections::{HashMap, HashSet};
use std::fs;

type Pos = (i32, i32);
type Dims = (usize, usize);

fn in_bounds(pos: Pos, dims: Dims) -> bool {
    0 <= pos.0 && pos.0 < (dims.0 as i32) && 0 <= pos.1 && pos.1 < (dims.1 as i32)
}

fn find_antinodes(
    mut node: Pos,
    diff: Pos,
    dims: Dims,
    p1_set: &mut HashSet<Pos>,
    p2_set: &mut HashSet<Pos>,
) {
    let mut count = 0;
    while in_bounds(node, dims) {
        if count == 1 {
            p1_set.insert(node);
        }
        count += 1;
        p2_set.insert(node);
        node = (node.0 + diff.0, node.1 + diff.1);
    }
}

pub fn solve() {
    let contents: String = fs::read_to_string("Day 08/data.txt")
        .expect("Could not read data file")
        .trim()
        .replace("\r", "");
    let data: Vec<&str> = contents.split("\n").collect();
    let dims = (data.len(), data[0].len());

    let mut node_frequencies = HashMap::<u8, Vec<Pos>>::new();
    for (i, row) in enumerate(&data) {
        for (j, node) in enumerate(row.bytes()) {
            if node != b'.' {
                node_frequencies
                    .entry(node)
                    .or_default()
                    .push((i as i32, j as i32));
            }
        }
    }

    let mut antinodes_1 = HashSet::<Pos>::new();
    let mut antinodes_2 = HashSet::<Pos>::new();
    for nodes in node_frequencies.values() {
        for (i, node1) in enumerate(nodes) {
            for node2 in nodes[(i + 1)..].iter() {
                let (dy, dx) = (node2.0 - node1.0, node2.1 - node1.1);
                find_antinodes(*node2, (dy, dx), dims, &mut antinodes_1, &mut antinodes_2);
                find_antinodes(*node1, (-dy, -dx), dims, &mut antinodes_1, &mut antinodes_2);
            }
        }
    }

    println!("Problem 15: {}", antinodes_1.len());
    println!("Problem 16: {}", antinodes_2.len());
}
