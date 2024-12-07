"""
FILE: Day 06/Python/sol.py
Author: Alex Jones
Desc: Solution to day 6 problems (11 & 12) for Advent of Code 2024, solved in Python 3.
"""
NOT_IMPLEMENTED = "Not Yet Implemented"
data = open("Day 06/data.txt", "r").read().strip().replace("\r","").split("\n")

turn_right = {(-1,0): (0,1), (0,1): (1,0), (1,0): (0,-1), (0,-1): (-1,0)}
grid_map = {(i, j): char for i, row in enumerate(data) for j, char in enumerate(row)}
pos = [pos for pos in grid_map if grid_map[pos] == '^'][0]

def simulate(grid_map, pos, dir_, visited):
    seen = set(visited)
    while pos in grid_map and (pos, dir_) not in seen:
        # Maintain a `visited` path to allow shorter simulations in part 2
        visited.append((pos, dir_))
        seen.add((pos, dir_))
        next_pos = (pos[0] + dir_[0], pos[1] + dir_[1])
        if grid_map.get(next_pos) == "#":
            dir_ = turn_right[dir_]
            continue
        pos = next_pos
    return (pos, dir_) not in seen

path = []
simulate(grid_map, pos, (-1,0), path)
print("Problem 11:", len(set(p for (p, _) in path)))

ans = 0
visited = set([])
# Opt 1: Only need to check placing obstacles on our path
for i, ((prev_pos, prev_dir), (pos, _)) in enumerate(zip(path[:-1], path[1:])):
    # Only check each position once, the first time encountered
    if pos == prev_pos or pos in visited: continue
    visited.add(pos)
    # Opt 2: simulate restoring from just before the point of collision
    ans += not simulate(grid_map | {pos: '#'}, prev_pos, prev_dir, path[:i])
print("Problem 12:", ans)
