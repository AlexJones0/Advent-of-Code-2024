"""
FILE: Day 18/Python/sol.py
Author: Alex Jones
Desc: Solution to day 18 problems (35 & 36) for Advent of Code 2024, solved in Python 3.
"""
data = open("Day 18/data.txt", "r").read().strip().replace("\r","").split("\n")
data = [tuple(int(v) for v in coord.split(",")) for coord in data]

GRID_HEIGHT = GRID_WIDTH = 71
START, END = (0, 0), (GRID_WIDTH - 1, GRID_HEIGHT - 1)

from heapq import heappush, heappop
from collections import defaultdict
from typing import Iterable, Optional
coord = tuple[int,int]

def neighbours(pos: coord, walls: set[coord]) -> Iterable[coord]:
    for dir_ in [(-1, 0), (0, 1), (1, 0), (0, -1)]:
        (nx, ny) = (pos[0] + dir_[0], pos[1] + dir_[1])
        if (nx, ny) in walls or nx < 0 or nx >= GRID_WIDTH or ny < 0 or ny >= GRID_HEIGHT:
            continue
        yield (nx, ny)

def h_taxicab(p1: coord, p2: coord) -> int:
    return abs(p2[0] - p1[0]) + abs(p2[1] - p1[1])

def min_steps_a_star(start: coord, end: coord, walls: set[coord]) -> Optional[int]:
    to_explore = [(0, start)]
    min_cost = defaultdict(lambda: float('inf'), {start: 0})
    explored = set()
    while to_explore:
        (_, pos) = heappop(to_explore)
        if pos in explored:
            continue
        explored.add(pos)
        if pos == end:
            return min_cost[pos]
        for ngb in neighbours(pos, walls):
            path_len = min_cost[pos] + 1
            if path_len < min_cost[ngb]:
                min_cost[ngb] = path_len
                heappush(to_explore, (path_len + h_taxicab(ngb, end), ngb)) 

print("Problem 35:", min_steps_a_star(START, END, set(data[:1024])))

def bin_search_for_first_blocking(start: coord, end: coord, data: list[coord]) -> str:
    left, right = 1024, len(data)
    while left < right:
        mid = (left + right) // 2
        if min_steps_a_star(start, end, set(data[:mid])) is None:
            right = mid
        else:
            left = mid + 1
    return ",".join(str(v) for v in data[right-1])

print("Problem 36:", bin_search_for_first_blocking(START, END, data))
