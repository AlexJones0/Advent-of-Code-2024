"""
FILE: Day 20/Python/sol.py
Author: Alex Jones
Desc: Solution to day 20 problems (39 & 40) for Advent of Code 2024, solved in Python 3.

Comments:
Optimised for runtime today, takes around ~1.0 seconds in CPython (for Python 3.9) on my
system.
"""
data = open("Day 20/data.txt", "r").read().strip().replace("\r","").split("\n")

vec_2 = tuple[int, int] # (y, x)

def in_bounds(grid: list[str], pos: vec_2) -> bool:
    return 0 <= pos[0] < len(grid) and 0 <= pos[1] < len(grid[0])

def find_path_len_from(grid: list[str], start: vec_2) -> tuple[list[int], list[vec_2]]:
    height, width = len(grid), len(grid[0])
    path_len = 0
    dist_from = [-1 for _ in range(height * width)]
    nodes = []
    pred, node = None, start
    while node is not None:
        dist_from[node[0] * width + node[1]] = path_len
        nodes.append(node)
        ngb = None
        for dir_ in [(-1, 0), (0, 1), (1, 0), (0, -1)]:
            next = (node[0] + dir_[0], node[1] + dir_[1])
            if next != pred and in_bounds(grid, next) and grid[next[0]][next[1]] != '#':
                ngb = next
                break
        pred, node = node, ngb
        path_len += 1
    return dist_from, nodes

def find_num_cheats(grid: list[str], start: vec_2, cheat_length: int, cutoff: int) -> int:
    height, width = len(grid), len(grid[0])
    start_dist, nodes = find_path_len_from(grid, start)
    count = 0
    for (y1, x1) in nodes:
        dist_1 = start_dist[y1 * width + x1]
        for y2 in range(max(0, y1 - cheat_length), min(height, y1 + cheat_length + 1)):
            dy = abs(y2 - y1)
            min_x2 = max(0, x1 - cheat_length + dy)
            max_x2 = min(height, x1 + cheat_length + 1 - dy)
            for x2 in range(min_x2, max_x2):
                dist_2 = start_dist[y2 * width + x2]
                if dist_2 - cutoff <= dist_1:
                    continue
                taxicab = dy + abs(x2 - x1)
                moves_saved = dist_2 - dist_1 - taxicab
                count += moves_saved >= cutoff
    return count

start = min(((i, j)) for i, r in enumerate(data) for j, c in enumerate(r) if c == 'S')
print("Problem 39:", find_num_cheats(data, start, 2, 100))
print("Problem 40:", find_num_cheats(data, start, 20, 100))