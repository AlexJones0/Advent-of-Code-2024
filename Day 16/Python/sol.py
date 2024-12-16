"""
FILE: Day 16/Python/sol.py
Author: Alex Jones
Desc: Solution to day 16 problems (31 & 32) for Advent of Code 2024, solved in Python 3.
"""
NOT_IMPLEMENTED = "Not Yet Implemented"
data = open("Day 16/data.txt", "r").read().strip().replace("\r","").split("\n")

import heapq
from collections import defaultdict
from typing import Iterable

LEFT = {(-1, 0): (0, -1), (0, -1): (1, 0), (1, 0): (0, 1), (0, 1): (-1, 0)}
RIGHT = {(-1, 0): (0, 1), (0, 1): (1, 0), (1, 0): (0, -1), (0, -1): (-1, 0)}

vec_2 = tuple[int, int]       # (y, x)
state = tuple[vec_2, vec_2]   # (position, direction)
move_info = tuple[int, state] # (cost, state)

def find_loc(grid: list[str], target: str) -> vec_2:
    return min(((i, j)) for i, r in enumerate(grid) for j, c in enumerate(r) if c == target)

def in_bounds(grid: list[str], pos: vec_2) -> bool:
    return pos[0] < len(grid) and pos[1] < len(grid[0])

def moves(grid: list[str], pos: vec_2, dir_: vec_2) -> Iterable[move_info]:
    for next_dir in [dir_, LEFT[dir_], RIGHT[dir_]]:
        next = (pos[0] + next_dir[0], pos[1] + next_dir[1])
        if in_bounds(grid, next) and grid[next[0]][next[1]] != '#':
            yield (1 + (next_dir != dir_) * 1000, (next, next_dir))

def get_nodes_on_paths(preds: dict[state,list[state]], end: state) -> set[vec_2]:
    nodes, seen = [end], set()
    while nodes:
        next = []
        for node in (n for n in nodes if n not in seen):
            seen.add(node)
            next += preds[node]
        nodes = next
    return {pos for (pos, _) in seen}

def dijkstra(grid: list[str], start: vec_2, dir_: vec_2, end: vec_2) -> vec_2:
    queue = [(0, (start, dir_))]
    lowest_cost = defaultdict(lambda: float('inf'), {(start, dir_): 0})
    preds = defaultdict(list)
    on_end_path, explored = set(), set()
    best = float('inf')
    while queue:
        (cost, state) = heapq.heappop(queue)
        if cost > best:
            continue
        if state[0] == end:
            best = cost
            on_end_path |= get_nodes_on_paths(preds, state)
        if state in explored:
            continue
        explored.add(state)
        for (cost, ngb) in moves(grid, *state):
            new_cost = lowest_cost[state] + cost
            if lowest_cost[ngb] > new_cost:
                preds[ngb] = []
                lowest_cost[ngb] = new_cost
            if lowest_cost[ngb] >= new_cost:
                preds[ngb].append(state)
                heapq.heappush(queue, (new_cost, ngb))
    return best, len(on_end_path)
    
start, end = find_loc(data, 'S'), find_loc(data, 'E')
p1, p2 = dijkstra(data, start, (0, 1), end)
print("Problem 31:", p1)
print("Problem 32:", p2)
