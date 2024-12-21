"""
FILE: Day 21/Python/sol.py
Author: Alex Jones
Desc: Solution to day 21 problems (41 & 42) for Advent of Code 2024, solved in Python 3.
"""
data = open("Day 21/data.txt", "r").read().strip().replace("\r","").split("\n")
moves = {"^": (-1, 0), ">": (0, 1), "v": (1, 0), "<": (0, -1)}

from functools import cache
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
    vertical = "^" if delta[0] < 0 else "v"
    horizontal = "<" if delta[1] < 0 else ">"
    shortest_length = float('inf')
    # Optimisation: the cheapest path will never zig-zag (moving is expensive, repeatedly
    # pressing "A" is cheap). So no BFS/combinatorics solution is required - it is
    # sufficient to just check the below two paths, so long as they are valid (don't walk
    # through the area with no button)
    invalid = find_pos(" ", robot != robots)
    paths = []
    if not invalid_path(start, end, invalid, 0): 
        paths.append(vertical * abs_delta[0] + horizontal * abs_delta[1] + "A")
    if not invalid_path(start, end, invalid, 1): 
        paths.append(horizontal * abs_delta[1] + vertical * abs_delta[0] + "A")
    for path in paths:
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