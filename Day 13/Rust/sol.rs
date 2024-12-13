/*
 * FILE: Day 13/Rust/sol.rs
 * Author: Alex Jones
 * Desc: Solution to day 13 problems (25 & 26) for Advent of Code 2024, solved in Rust.
 */
use itertools::Itertools;
use std::fs;

fn least_moves(moves: &[(i64, i64); 2], goal: (i64, i64)) -> i64 {
    let (ax, ay) = moves[0];
    let (bx, by) = moves[1];
    let (gx, gy) = goal;
    let numerator = gx * ay - ax * gy;
    let denominator = ay * bx - ax * by;
    if denominator == 0 || numerator % denominator != 0 {
        return 0; // Impossible to solve or non-integer solution (impossible)
    }
    let b_mul = numerator / denominator;
    let numerator = gy - by * b_mul;
    let denominator = ay;
    if numerator % denominator != 0 {
        return 0; // Non-integer solution (impossible)
    }
    let a_mul = numerator / denominator;
    a_mul * 3 + b_mul
}

pub fn solve() {
    let contents: String = fs::read_to_string("Day 13/data.txt")
        .expect("Could not read data file")
        .trim()
        .replace("\r", "");
    let data: Vec<[(i64, i64); 3]> = contents
        .split("\n\n")
        .map(|entry| {
            entry
                .split('\n')
                .map(|row| {
                    row.split(':')
                        .last()
                        .unwrap()
                        .split(',')
                        .map(|d| {
                            d.split(['+', '='])
                                .last()
                                .unwrap()
                                .trim()
                                .parse::<i64>()
                                .unwrap()
                        })
                        .collect_tuple()
                        .unwrap()
                })
                .collect::<Vec<_>>()
                .try_into()
                .unwrap()
        })
        .collect();

    println!(
        "Problem 25: {}",
        data.iter()
            .map(|entry| least_moves(&entry[0..2].try_into().unwrap(), entry[2]))
            .sum::<i64>()
    );
    const OFFSET: i64 = 10000000000000;
    println!(
        "Problem 26: {}",
        data.iter()
            .map(|entry| least_moves(
                &entry[0..2].try_into().unwrap(),
                (entry[2].0 + OFFSET, entry[2].1 + OFFSET)
            ))
            .sum::<i64>()
    );
}
