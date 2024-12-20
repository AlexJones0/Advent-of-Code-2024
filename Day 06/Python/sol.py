"""
FILE: Day 06/Python/sol.py
Author: Alex Jones
Desc: Solution to day 6 problems (11 & 12) for Advent of Code 2024, solved in Python 3.
"""
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

from functools import cache

@cache
def find_next_collision(pos, dir_):
    while pos in grid_map and grid_map[pos] != '#':
        pos = (pos[0] + dir_[0], pos[1] + dir_[1])
    return pos

def between(item, one, two):
    return one <= item < two or two <= item < one

def new_obstacle_blocks_path(current_pos, current_dir, new_pos, obstacle):
    if current_dir[0] != 0 and current_pos[1] == obstacle[1]:
        return between(obstacle[0], current_pos[0], new_pos[0])
    elif current_dir[1] != 0 and current_pos[0] == obstacle[0]:
        return between(obstacle[1], current_pos[1], new_pos[1])
    return False

ans = 0
visited_on_path = set()
# Optimisation 1: Only try placing obstacles on nodes explored in our original path
for i, ((prev_pos, prev_dir), (pos, _)) in enumerate(zip(path[:-1], path[1:])):
    # Only check each position once, the first time encountered
    if pos == prev_pos or pos in visited_on_path: continue
    visited_on_path.add(pos)
    # Optimisation 2: Simulate restoring from just before the point of collision
    current_pos, next_pos, current_dir = prev_pos, prev_pos, prev_dir
    seen = set()
    while next_pos in grid_map and (current_pos, current_dir) not in seen:
        seen.add((current_pos, current_dir))
        # Optimisation 3: Use a memoized global result to automatically find the next
        # collision given the current position and direction. Then, just check if the
        # newly inserted obstacle would have been on that collision path - if so, set
        # the new position to be just before the obstacle instead.
        next_pos = find_next_collision(current_pos, current_dir)
        if new_obstacle_blocks_path(current_pos, current_dir, next_pos, pos):
            next_pos = pos
        current_pos = (next_pos[0] - current_dir[0], next_pos[1] - current_dir[1])
        current_dir = turn_right[current_dir]
    # Only increment if we stopped searching because we found a cycle, instead of 
    # finding the grid border
    ans += next_pos in grid_map

print("Problem 12:", ans)
