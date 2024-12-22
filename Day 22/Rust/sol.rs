/*
 * FILE: Day 22/Rust/sol.rs
 * Author: Alex Jones
 * Desc: Solution to day 22 problems (43 & 44) for Advent of Code 2024, solved in Rust.
 */
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs;

fn get_next(mut num: u32) -> u32 {
    num = ((num << 6) ^ num) & 0xFFFFFF;
    num = ((num >> 5) ^ num) & 0xFFFFFF;
    ((num << 11) ^ num) & 0xFFFFFF
}

pub fn solve() {
    let contents: String = fs::read_to_string("Day 22/data.txt")
        .expect("Could not read data file")
        .trim()
        .replace("\r", "");
    let mut data: Vec<u32> = contents.split("\n").map(|n| n.parse().unwrap()).collect();

    let mut bananas = [0; 130321];
    let mut deltas = VecDeque::with_capacity(4);
    deltas.extend([0; 4]);
    for num in &mut data {
        let mut price = *num % 10;
        let mut seen = HashSet::with_capacity(2000);
        for _ in 0..3 {
            *num = get_next(*num);
            let new_price = *num % 10;
            deltas.pop_front();
            deltas.push_back(new_price - price);
            price = new_price;
        }
        for _ in 0..1997 {
            *num = get_next(*num);
            let new_price = *num % 10;
            deltas.pop_front();
            deltas.push_back(new_price - price);
            price = new_price;
            let perfect_hash =
                (61731 + deltas[0] * 6859 + deltas[1] * 361 + deltas[2] * 19 + deltas[3]) as usize;
            if !seen.contains(&perfect_hash) {
                seen.insert(perfect_hash);
                bananas[perfect_hash] += price;
            }
        }
    }
    println!(
        "Problem 43: {}",
        data.iter().map(|&n| n as u64).sum::<u64>()
    );
    println!("Problem 44: {}", bananas.iter().max().unwrap());
}
