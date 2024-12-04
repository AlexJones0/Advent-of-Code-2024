/*
 * FILE: Day 04/Rust/sol.rs
 * Author: Alex Jones
 * Desc: Solution to day 4 problems (7 & 8) for Advent of Code 2024, solved in Rust.
 */
use std::fs;

pub fn solve() {
    let contents: String = fs::read_to_string("Day 04/data.txt")
        .expect("Could not read data file")
        .trim()
        .replace("\r", "");
    let data: Vec<&str> = contents
        .split("\n")
        .collect();

    const UNIT_CARDINALS: &[(i32,i32)] = &[(0,1),(0,-1),(1,0),(-1,0)];
    const UNIT_INTERCARDINALS: &[(i32,i32)] = &[(-1,-1),(-1,1),(1,-1),(1,1)];

    let height = data.len();
    let width = data[0].len();
    let dirs = [UNIT_CARDINALS, UNIT_INTERCARDINALS].concat();
    let mut matches = 0u32;
    for i in 0..(height as i32) {
        for j in 0..(width as i32) {
            for dir_ in &dirs {
                let last_i = i + dir_.0 * 3;
                let last_j = j + dir_.1 * 3;
                if 0 > last_i || last_i >= (height as i32) || 0 > last_j || last_j >= (width as i32) {
                    continue
                }
                let mut is_match = true;
                for k in 0..4 {
                    let ni = (i + dir_.0 * k) as usize;
                    let nj = (j + dir_.1 * k) as usize;
                    if data[ni].chars().nth(nj).unwrap() != "XMAS".chars().nth(k as usize).unwrap() {
                        is_match = false;
                        break
                    }
                }
                matches += is_match as u32;
            }
        }
    }
    println!("Problem 7: {}", matches);

    matches = 0;
    for i in 1..(height as i32-1) {
        for j in 1..(width as i32-1) {
            if data[i as usize].chars().nth(j as usize).unwrap() != 'A' {
                continue
            }
            let crosses = UNIT_INTERCARDINALS.iter()
                .filter(|dir_| {
                    let ni1 = (i + dir_.0) as usize;
                    let nj1 = (j + dir_.1) as usize;
                    let ni2 = (i - dir_.0) as usize;
                    let nj2 = (j - dir_.1) as usize;
                       data[ni1].chars().nth(nj1).unwrap() == 'M'
                    && data[ni2].chars().nth(nj2).unwrap() == 'S'
                })
                .count();
                matches += (crosses == 2) as u32;
        }
    }
    println!("Problem 8: {}", matches);
}