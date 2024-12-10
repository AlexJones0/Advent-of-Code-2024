/*
 * FILE: Day 09/Rust/sol.rs
 * Author: Alex Jones
 * Desc: Solution to day 9 problems (17 & 18) for Advent of Code 2024, solved in Rust.
 */
use itertools::enumerate;
use std::collections::BinaryHeap;
use std::fs;

fn add_to_total(total: u64, count: u64, value: u64, size: u64) -> (u64, u64) {
    (
        total + value * (size * count + size * (size - 1) / 2),
        count + size,
    )
}

pub fn solve() {
    let data: Vec<u64> = fs::read_to_string("Day 09/data.txt")
        .expect("Could not read data file")
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u64)
        .collect();

    // O(n) Two pointer solution using sizes
    let mut files: Vec<u64> = data.iter().skip(2).step_by(2).cloned().collect();
    let mut gaps: Vec<u64> = data.iter().skip(1).step_by(2).cloned().collect();
    let mut total: u64 = 0;
    let mut count: u64 = data[0];

    let mut gap_index: usize = 0;
    let mut file_index: usize = files.len() - 1;
    while gap_index < file_index {
        let size = files[file_index];
        let gap_size = gaps[gap_index];
        if size <= gap_size {
            (total, count) = add_to_total(total, count, file_index as u64 + 1, size);
            file_index -= 1;
            gaps[gap_index] -= size;
        } else {
            (total, count) = add_to_total(total, count, file_index as u64 + 1, gap_size);
            files[file_index] = size - gap_size;
            gaps[gap_index] = 0
        }
        if gaps[gap_index] == 0 {
            let next_file_size = files[gap_index];
            (total, count) = add_to_total(total, count, gap_index as u64 + 1, next_file_size);
            gap_index += 1;
        }
    }
    if gap_index == file_index {
        let last_file_size = files[gap_index];
        (total, _) = add_to_total(total, count, gap_index as u64 + 1, last_file_size);
    }

    println!("Problem 17: {}", total);

    let mut files = Vec::<(u64, u64)>::new();
    let mut gaps = [const { BinaryHeap::new() }; 10];

    let mut position = 0u64;
    for (index, size) in enumerate(data) {
        if index % 2 == 0 {
            files.push((position, size));
        } else if size > 0 {
            gaps[size as usize].push(-(position as i64));
        }
        position += size;
    }

    for value in (0..files.len()).rev() {
        let (file_pos, file_size) = files[value];
        let first_gap = (file_size..10)
            .map(|size| (gaps[size as usize].peek().map(|&v| -v), size as usize))
            .min();
        if let Some((Some(gap_pos), gap_size)) = first_gap {
            if file_pos <= gap_pos as u64 {
                continue;
            }
            files[value] = (gap_pos as u64, file_size);
            gaps[gap_size].pop();
            gaps[gap_size - file_size as usize].push(-(gap_pos + file_size as i64));
        }
    }

    println!(
        "Problem 18: {}",
        &files
            .iter()
            .enumerate()
            .map(|(value, &(start, size))| (start..(start + size)).sum::<u64>() * value as u64)
            .sum::<u64>()
    );
}
