/*
 * FILE: Day 14/Rust/sol.rs
 * Author: Alex Jones
 * Desc: Solution to day 14 problems (27 & 28) for Advent of Code 2024, solved in Rust.
 */
use itertools::Itertools;
use std::collections::HashSet;
use std::fs;

type Pos = (i32, i32);
type Vel = (i32, i32);
type Robot = (Pos, Vel);

const SPACE_WIDTH: i32 = 101;
const SPACE_HEIGHT: i32 = 103;

fn simulate_robots(robots: &mut [Robot], timesteps: i32) -> Option<i32> {
    let mut t = 0;
    while t != timesteps {
        for elem in robots.iter_mut() {
            let (pos, vel) = elem;
            *pos = (
                (pos.0 + vel.0).rem_euclid(SPACE_WIDTH),
                (pos.1 + vel.1).rem_euclid(SPACE_HEIGHT),
            );
        }
        if timesteps == -1 {
            let positions = HashSet::<(i32, i32)>::from_iter(robots.iter().map(|&r| r.0));
            if positions.len() == robots.len() {
                return Some(t + 1);
            }
        }
        t += 1;
    }
    None
}

fn get_safety_factor(robots: &[Robot]) -> u64 {
    let mut quad = [0; 4];
    const MID_X: i32 = SPACE_WIDTH / 2;
    const MID_Y: i32 = SPACE_HEIGHT / 2;
    for &((x, y), _) in robots {
        if x == MID_X || y == MID_Y {
            continue;
        }
        let index = 2 * (x < MID_X) as usize + (y < MID_Y) as usize;
        quad[index] += 1;
    }
    quad.iter().product()
}

pub fn solve() {
    let contents: String = fs::read_to_string("Day 14/data.txt")
        .expect("Could not read data file")
        .trim()
        .replace("\r", "");
    let mut data: Vec<Robot> = contents
        .split("\n")
        .map(|line| {
            line.split_whitespace()
                .map(|item| {
                    item.split("=")
                        .last()
                        .unwrap()
                        .split(",")
                        .map(|val| val.trim().parse::<i32>().unwrap())
                        .collect_tuple()
                        .unwrap()
                })
                .collect_tuple()
                .unwrap()
        })
        .collect();

    simulate_robots(&mut data, 100);
    println!("Problem 27: {}", get_safety_factor(&data));
    println!(
        "Problem 28: {}",
        simulate_robots(&mut data, -1).unwrap() + 100
    );
}
