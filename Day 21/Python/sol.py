"""
FILE: Day 21/Python/sol.py
Author: Alex Jones
Desc: Solution to day 21 problems (41 & 42) for Advent of Code 2024, solved in Python 3.
"""
data = open("Day 21/data.txt", "r").read().strip().replace("\r","").split("\n")
moves = {"^": (-1, 0), ">": (0, 1), "v": (1, 0), "<": (0, -1)}

from functools import cache
from itertools import combinations
Vec2 = tuple[int, int]

def find_pos(char: str, is_directional: bool) -> Vec2:
    idx = " ^A<v>".index(char) if is_directional else "789456123 0A".index(char)
    return (idx // 3, idx % 3)

def invalid_path(start: Vec2, end: Vec2, invalid: Vec2, dim: int) -> bool:
    return end[dim] == invalid[dim] and start[1-dim] == invalid[1-dim]

@cache
def find_shortest(start: Vec2, end: Vec2, robot: int, robots: int) -> int:
    delta = (end[0] - start[0], end[1] - start[1])
    abs_delta = (abs(delta[0]), abs(delta[1]))
    path_size = abs_delta[0] + abs_delta[1]
    # For robot 1, it just requires moving the taxicab distance plus 1 ('activate')
    if robot == 1:
        return path_size + 1
    invalid = find_pos(" ", robot != robots)
    vertical = "^" if delta[0] < 0 else "v"
    horizontal = "<" if delta[1] < 0 else ">"
    vert_hori_path = vertical * abs_delta[0] + horizontal * abs_delta[1] + "A"
    hori_vert_path = horizontal * abs_delta[1] + vertical * abs_delta[0] + "A"
    # Use combinatorics to find all paths from start -> end
    shortest_length = float('inf')
    for verts in combinations(range(path_size), abs_delta[0]):
        path = [vertical if i in verts else horizontal for i in range(path_size)]
        path = "".join(path) + "A"
        # Check it doesn't path through invalid
        if invalid_path(start, end, invalid, 0) and path == vert_hori_path or \
           invalid_path(start, end, invalid, 1) and path == hori_vert_path:
            continue
        # Find the cost of the path by recursively using the next robot to make
        # each button press on the path
        current_pos = find_pos("A", True)
        pathlen = 0
        for button in path:
            pos = find_pos(button, True)
            pathlen += find_shortest(current_pos, pos, robot - 1, robots)
            current_pos = pos
        shortest_length = min(shortest_length, pathlen)
    # The shortest length is the shortest length of all possible paths
    return shortest_length

def calc(code: str, robots: int) -> int:
    # For each code, we sum the cost of pressing each sequential button on the
    # first (inner-most) robot, starting from the previous position.
    result = 0
    start = find_pos("A", False)
    for button in code:
        pos = find_pos(button, False)
        result += find_shortest(start, pos, robots, robots)
        start = pos
    return result * int(code[:-1])

print("Problem 41:", sum(calc(code, 3) for code in data))
print("Problem 42:", sum(calc(code, 26) for code in data))