/*
 * FILE: Day 11/Rust/sol.rs
 * Author: Alex Jones
 * Desc: Solution to day 11 problems (21 & 22) for Advent of Code 2024, solved in Rust.
 */
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::fs;

fn blink(stone: u64) -> [Option<u64>; 2] {
    if stone == 0 {
        return [Some(1), None];
    }
    let stone_str: String = stone.to_string();
    if stone_str.len() % 2 == 0 {
        let mid: usize = stone_str.len() / 2;
        [stone_str[..mid].parse().ok(), stone_str[mid..].parse().ok()]
    } else {
        [Some(stone * 2024), None]
    }
}

type Memo = HashMap<(u64, u32), u64>;
fn stones_after(memo: &mut Memo, stone: u64, blinks: u32) -> u64 {
    let entry = memo.entry((stone, blinks));
    if let Entry::Occupied(e) = entry {
        *e.get()
    } else if blinks == 0 {
        *entry.or_insert(1u64)
    } else {
        let result: u64 = blink(stone)
            .iter()
            .map(|st| {
                if let Some(val) = st {
                    stones_after(memo, *val, blinks - 1)
                } else {
                    0u64
                }
            })
            .sum();
        memo.insert((stone, blinks), result);
        result
    }
}

pub fn solve() {
    let contents: String = fs::read_to_string("Day 11/data.txt")
        .expect("Could not read data file")
        .trim()
        .replace("\r", "");
    let data: Vec<u64> = contents
        .split_whitespace()
        .map(|val| val.parse().unwrap())
        .collect();

    let mut memoized = Memo::new();
    println!(
        "Problem 21: {}",
        data.iter()
            .map(|stone| stones_after(&mut memoized, *stone, 25))
            .sum::<u64>()
    );
    println!(
        "Problem 22: {}",
        data.iter()
            .map(|stone| stones_after(&mut memoized, *stone, 75))
            .sum::<u64>()
    );
}
