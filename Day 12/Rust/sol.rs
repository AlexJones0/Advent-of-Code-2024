/*
 * FILE: Day 12/Rust/sol.rs
 * Author: Alex Jones
 * Desc: Solution to day 12 problems (23 & 24) for Advent of Code 2024, solved in Rust.
 */
use std::collections::HashSet;
use std::fs;

type Pos = (i32, i32);
type Dir = (i32, i32);
type Side = (Pos, Dir);

struct Region {
    plots: HashSet<Pos>,
    area: usize,
    perimeter: usize,
    sides: usize,
}

const UNIT_CARDINALS: [Dir; 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

fn right(dir: Dir) -> Dir {
    match dir {
        (-1, 0) => (0, 1),
        (0, 1) => (1, 0),
        (1, 0) => (0, -1),
        (0, -1) => (-1, 0),
        _ => panic!("Invalid unit cardinal direction!"),
    }
}

fn contributed_sides(pos: Pos, dir: Dir, perimeter: &HashSet<Side>) -> i32 {
    let right_dir = right(dir);
    let right_side = (pos.0 + right_dir.0, pos.1 + right_dir.1);
    let left_side = (pos.0 - right_dir.0, pos.1 - right_dir.1);
    let left_present = perimeter.contains(&(left_side, dir));
    let right_present = perimeter.contains(&(right_side, dir));
    if left_present && right_present {
        -1
    } else if !left_present && !right_present {
        1
    } else {
        0
    }
}

fn get_region(data: &[&str], start: Pos) -> Region {
    let mut visited = HashSet::new();
    let mut perimeter = HashSet::new();
    let mut area = 0;
    let mut sides = 0;
    let mut plots = Vec::from([start]);
    while let Some(pos) = plots.pop() {
        if visited.contains(&pos)
            || data[start.0 as usize].chars().nth(start.1 as usize)
                != data[pos.0 as usize].chars().nth(pos.1 as usize)
        {
            continue;
        }
        visited.insert(pos);
        area += 1;
        for dir in UNIT_CARDINALS {
            let ngb = (pos.0 + dir.0, pos.1 + dir.1);
            if visited.contains(&ngb) {
                let opposite = (-dir.0, -dir.1);
                sides -= contributed_sides(ngb, opposite, &perimeter);
                perimeter.remove(&(ngb, opposite));
            } else {
                sides += contributed_sides(pos, dir, &perimeter);
                perimeter.insert((pos, dir));
                if 0 <= ngb.0
                    && ngb.0 < data.len() as i32
                    && 0 <= ngb.1
                    && ngb.1 < data[0].len() as i32
                {
                    plots.push(ngb);
                }
            }
        }
    }
    Region {
        plots: visited,
        area,
        perimeter: perimeter.len(),
        sides: sides as usize,
    }
}

fn get_regions(data: &[&str]) -> Vec<Region> {
    let mut visited = HashSet::<Pos>::new();
    let mut regions = Vec::new();
    for y in 0..data.len() {
        for x in 0..data[0].len() {
            if visited.contains(&(y as i32, x as i32)) {
                continue;
            }
            let region = get_region(data, (y as i32, x as i32));
            visited.extend(region.plots.clone().into_iter());
            regions.push(region);
        }
    }
    regions
}

pub fn solve() {
    let contents: String = fs::read_to_string("Day 12/data.txt")
        .expect("Could not read data file")
        .trim()
        .replace("\r", "");
    let data: Vec<&str> = contents.split("\n").collect();

    let regions = get_regions(&data);
    println!(
        "Problem 23: {}",
        regions
            .iter()
            .map(|region| region.area * region.perimeter)
            .sum::<usize>()
    );
    println!(
        "Problem 24: {}",
        regions
            .iter()
            .map(|region| region.area * region.sides)
            .sum::<usize>()
    );
}
