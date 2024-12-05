/*
 * FILE: Day 05/Rust/sol.rs
 * Author: Alex Jones
 * Desc: Solution to day 5 problems (9 & 10) for Advent of Code 2024, solved in Rust.
 */
use std::fs;
use std::collections::HashMap;
use std::cmp::Ordering;

pub fn solve() {
    let contents: String = fs::read_to_string("Day 05/data.txt")
        .expect("Could not read data file")
        .trim()
        .replace("\r", "");
    let data: Vec<&str> = contents
        .split("\n\n")
        .collect();
    let orderings = data[0].split("\n")
        .map(|x| {
            let elems = x.split("|").collect::<Vec<_>>();
            (elems[0], elems[1])
        })
        .collect::<Vec<_>>();
    let updates = data[1].split("\n")
        .map(|x| x.split(",").collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut comes_before: HashMap<&str,Vec<&str>> = HashMap::new();
    for (pred, succ) in orderings {
        if comes_before.contains_key(pred) {
            comes_before.get_mut(pred).unwrap().push(succ);
        } else {
            comes_before.insert(pred, vec![succ]);
        }
    }

    let (mut p1, mut p2) = (0, 0);
    for update in updates {
        let mut ordered = update.clone();
        ordered.sort_by(|fst, snd| {
            if comes_before[fst].contains(snd) { Ordering::Less }
            else { Ordering::Equal }
        });
        if update == ordered {
            p1 += update[update.len()/2].parse::<i32>().unwrap();
        } else {
            p2 += ordered[ordered.len()/2].parse::<i32>().unwrap();
        }
    }
    
    println!("Problem 9: {}", p1);
    println!("Problem 10: {}", p2);
}