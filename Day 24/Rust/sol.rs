/*
 * FILE: Day 24/Rust/sol.rs
 * Author: Alex Jones
 * Desc: Solution to day 24 problems (47 & 48) for Advent of Code 2024, solved in Rust.
 */
use std::collections::{HashMap, HashSet};
use std::fs;

use itertools::Itertools;

pub fn solve() {
    let contents: String = fs::read_to_string("Day 24/data.txt")
        .expect("Could not read data file")
        .trim()
        .replace("\r", "");
    let data: Vec<&str> = contents.split("\n\n").collect();
    let mut wires = HashMap::<&str, bool>::from_iter(data[0].split("\n").map(|wire| {
        let parts = wire.split(": ").collect::<Vec<_>>();
        (parts[0], parts[1].parse::<u32>().unwrap() != 0)
    }));
    let gates = data[1]
        .split("\n")
        .map(|gate| {
            let parts = gate.split(" ").collect::<Vec<_>>();
            (parts[1], parts[0], parts[2], parts[4])
        })
        .collect::<Vec<_>>();
    type BinOp = fn(bool, bool) -> bool;
    let operators: [(&str, BinOp); 3] = [
        ("AND", |x, y| x & y),
        ("OR", |x, y| x | y),
        ("XOR", |x, y| x ^ y),
    ];
    let operators: HashMap<&str, BinOp> = HashMap::from(operators);

    // Simulate gates/circuit
    let mut sim_gates = gates.clone();
    while !sim_gates.is_empty() {
        let mut to_remove = Vec::new();
        for gate in &sim_gates {
            let (op, inp1, inp2, wire) = gate;
            let val1_w = wires.get(inp1);
            let val2_w = wires.get(inp2);
            if let (Some(val1), Some(val2)) = (val1_w, val2_w) {
                wires.insert(wire, operators.get(op).unwrap()(*val1, *val2));
                to_remove.push(*gate);
            }
        }
        for gate in to_remove {
            sim_gates.retain(|g| *g != gate);
        }
    }

    // Collect answer from zXX bits
    let mut p1_ans = 0u64;
    for (gate, val) in &wires {
        if gate.as_bytes()[0] == b'z' {
            p1_ans |= (*val as u64)
                << String::from_utf8(gate.as_bytes()[1..].to_vec())
                    .unwrap()
                    .parse::<usize>()
                    .unwrap();
        }
    }
    println!("Problem 47: {}", p1_ans);

    // Compute the actual result of addition
    let mut x = 0;
    let mut y = 0;
    for (gate, val) in &wires {
        let fst = gate.as_bytes()[0];
        if fst == b'x' {
            x |= (*val as u64)
                << String::from_utf8(gate.as_bytes()[1..].to_vec())
                    .unwrap()
                    .parse::<usize>()
                    .unwrap();
        } else if fst == b'y' {
            y |= (*val as u64)
                << String::from_utf8(gate.as_bytes()[1..].to_vec())
                    .unwrap()
                    .parse::<usize>()
                    .unwrap();
        }
    }
    let target_ans = x + y;

    // Compute the MSB of the output
    let mut output_msb = 0;
    for wire in wires.keys() {
        if wire.as_bytes()[0] == b'z' {
            output_msb = output_msb.max(
                String::from_utf8(wire.as_bytes()[1..].to_vec())
                    .unwrap()
                    .parse::<usize>()
                    .unwrap(),
            );
        }
    }
    let output_msb_wire = format!("z{:02}", output_msb);

    // The input LSBs
    let mut x_lsb = "x0".to_string();
    let mut y_lsb = "y0".to_string();
    while !wires.contains_key(x_lsb.as_str()) || !wires.contains_key(y_lsb.as_str()) {
        x_lsb = format!("{}0", x_lsb);
        y_lsb = format!("{}0", y_lsb);
    }

    // Use the structure of the ripple adder to figure out which gates are wrong, and thus
    // need to be swapped. We don't actually need to figure out the pairs for swapping.
    let mut needs_swapping = HashSet::with_capacity(8);
    for &(op, inp1, inp2, wire) in &gates {
        // Check if the final carry is wrong; seems to never happen though
        if wire == output_msb_wire {
            if (p1_ans & output_msb as u64) != (target_ans & output_msb as u64) {
                needs_swapping.insert(wire);
            }
        // Check if the LSB is wrong; seems never to happen though
        } else if (inp1 == x_lsb || inp1 == y_lsb || inp2 == x_lsb || inp2 == y_lsb) && op == "AND"
        {
            if (p1_ans & 1) != (target_ans & 1) {
                needs_swapping.insert(wire);
            }
        // XOR is only used for output bits ("zxx") or on inputs. Intermediary gates are
        // all AND or OR, so something needs to swap if such cases are found.
        } else if op == "XOR"
            && [wire, inp1, inp2]
                .iter()
                .all(|w| ![b'x', b'y', b'z'].contains(&w.as_bytes()[0]))
        {
            needs_swapping.insert(wire);
        // All outputs (minus the MSB) appear to be XOR, so if not, something needs swapping
        } else if wire.as_bytes()[0] == b'z' && op != "XOR" {
            needs_swapping.insert(wire);
        /* For the final case, it appears that in the input ripple adder construction, we
        always have AND->OR, XOR->AND, or XOR->XOR (except for the input LSB, where we
        have a half adder). So, we just detect gate pairings - if we see any AND->AND,
        AND->XOR, or XOR->OR, we know the current wire (1st output) must be incorrect,
        and thus needs swapping. */
        } else {
            for &(next_op, next_inp1, next_inp2, _) in &gates {
                if (wire == next_inp1 || wire == next_inp2)
                    && (op == "AND" && next_op != "OR" || op == "XOR" && next_op == "OR")
                {
                    needs_swapping.insert(wire);
                }
            }
        }
    }
    println!("Problem 48: {}", needs_swapping.iter().sorted().join(","));
}
