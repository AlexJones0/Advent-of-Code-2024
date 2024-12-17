/*
 * FILE: Day 17/Rust/sol.rs
 * Author: Alex Jones
 * Desc: Solution to day 17 problems (33 & 34) for Advent of Code 2024, solved in Rust.
 */
use std::fs;

const A: usize = 0;
const B: usize = 1;
const C: usize = 2;

fn simulate(mut regs: [u64; 3], instrs: &[usize]) -> Vec<u64> {
    let mut out = Vec::new();
    let mut pcc = 0;
    while pcc < instrs.len() {
        let (opcode, operand) = (instrs[pcc], instrs[pcc + 1]);
        let combo = if operand < 4 || operand == 7 {
            operand as u64
        } else {
            regs[operand - 4]
        };
        match opcode {
            0 => regs[A] /= 2u64.pow(combo as u32),
            1 => regs[B] ^= operand as u64,
            2 => regs[B] = combo % 8,
            3 => {
                if regs[A] != 0 {
                    pcc = operand;
                    continue;
                }
            }
            4 => regs[B] ^= regs[C],
            5 => out.push(combo % 8),
            6 => regs[B] = regs[A] / (2u64.pow(combo as u32)),
            7 => regs[C] = regs[A] / (2u64.pow(combo as u32)),
            _ => panic!(),
        }
        pcc += 2;
    }
    out
}

fn dfs(regs: [u64; 3], instrs: &[usize], state: u64, instr: usize) -> Option<u64> {
    for i in 0..8 {
        let test_state = state + i;
        let test_regs = [test_state, regs[B], regs[C]];
        if simulate(test_regs, instrs)[0] == instrs[instr] as u64 {
            if instr == 0 {
                return Some(test_state);
            }
            let res = dfs(regs, instrs, test_state * 8, instr - 1);
            if res.is_some() {
                return res;
            }
        }
    }
    None
}

pub fn solve() {
    let contents: String = fs::read_to_string("Day 17/data.txt")
        .expect("Could not read data file")
        .trim()
        .replace("\r", "");
    let data: Vec<&str> = contents.split("\n\n").collect();

    let regs: Vec<u64> = data[0]
        .split("\n")
        .map(|line| line.split(":").last().unwrap().trim().parse().unwrap())
        .collect();
    let regs: [u64; 3] = [regs[0], regs[1], regs[2]];
    let instrs: Vec<usize> = data[1]
        .split(":")
        .last()
        .unwrap()
        .split(",")
        .map(|val| val.trim().parse().unwrap())
        .collect();

    println!(
        "Problem 33: {}",
        simulate(regs, &instrs)
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(",")
    );

    println!(
        "Problem 34: {}",
        dfs(regs, &instrs, 0, instrs.len() - 1).unwrap()
    );
}
