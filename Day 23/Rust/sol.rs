/*
 * FILE: Day 23/Rust/sol.rs
 * Author: Alex Jones
 * Desc: Solution to day 23 problems (45 & 46) for Advent of Code 2024, solved in Rust.
 */
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;

pub fn solve() {
    let contents: String = fs::read_to_string("Day 23/data.txt")
        .expect("Could not read data file")
        .trim()
        .replace("\r", "");
    let data: Vec<(u16, u16)> = contents
        .split("\n")
        .map(|row| {
            let conn = row.split("-").collect::<Vec<_>>();
            let fst = conn[0].as_bytes();
            let snd = conn[1].as_bytes();
            (
                ((fst[0] as u16) << 8) | fst[1] as u16,
                ((snd[0] as u16) << 8) | snd[1] as u16,
            )
        })
        .collect();

    let mut graph = HashMap::<u16, HashSet<u16>>::new();
    for (fst, snd) in data {
        graph
            .entry(fst)
            .or_insert(HashSet::with_capacity(14))
            .insert(snd);
        graph
            .entry(snd)
            .or_insert(HashSet::with_capacity(14))
            .insert(fst);
    }
    let mut target_cliques = HashSet::with_capacity(1000);
    for (node1, ngbs) in &graph {
        if node1 >> 8u16 != 0x74u16 {
            continue;
        }
        for node2 in ngbs {
            for node3 in ngbs {
                if graph.get(node2).unwrap().contains(node3) {
                    let mut nodes = [node1, node2, node3];
                    nodes.sort();
                    target_cliques.insert(nodes);
                }
            }
        }
    }
    println!("Problem 45: {}", target_cliques.len());

    /* The input graph is simple enough that a greedy / brute force approach
    will work. For each node, start by considering the full set of neighbours.
    By assuming that the maximum clique is quite close to the maximum degree (
    it is, both appear to be around 13/14 nodes), we can just iteratively try
    removing all nodes from our initial sets of all nodes connected to one
    node, until we find a clgraphiquie. By processing in a FIFO queue, the first
    clique we find is necessarily maximal. We could also use a heap where the
    cost is the size, but it seems it is not needed here. */
    let mut queue = VecDeque::from_iter(graph.clone().into_iter().map(|(node, ngbs)| {
        let mut nodes = ngbs.into_iter().collect::<Vec<_>>();
        nodes.push(node);
        nodes.sort();
        nodes
    }));
    let mut p2_ans = None;
    while let Some(maybe_clique) = queue.pop_front() {
        if maybe_clique
            .iter()
            .enumerate()
            .all(|(i, n1)| (0..i).all(|j| graph.get(n1).unwrap().contains(&maybe_clique[j])))
        {
            let mut ans = Vec::from([(maybe_clique[0] >> 8) as u8, maybe_clique[0] as u8]);
            for &node in maybe_clique.iter().skip(1) {
                ans.push(b',');
                ans.push((node >> 8) as u8);
                ans.push(node as u8);
            }
            p2_ans = Some(ans);
            break;
        }
        queue.extend(maybe_clique.iter().map(|ngb| {
            maybe_clique
                .iter()
                .filter(|node| *node != ngb)
                .cloned()
                .collect()
        }));
    }
    println!(
        "Problem 46: {}",
        String::from_utf8(p2_ans.unwrap()).unwrap()
    );
}
