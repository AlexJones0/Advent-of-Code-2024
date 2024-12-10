"""
FILE: Day 10/Python/sol.py
Author: Alex Jones
Desc: Solution to day 10 problems (19 & 20) for Advent of Code 2024, solved in Python 3.
"""
data = open("Day 10/data.txt", "r").read().strip().replace("\r","").split("\n")
data = [list(map(int, row)) for row in data]
dirs = [(1, 0), (0, 1), (-1, 0), (0, -1)]

# Don't need to cache today, but could use functools.cache here
def trails_from(y, x):
    if data[y][x] == 9:
        return set([((y, x),)])
    trailhead_ends = set()
    for dy, dx in dirs:
        ny, nx = y + dy, x + dx
        if 0 > ny or ny >= len(data) or 0 > nx or nx >= len(data[0]):
            continue
        if data[ny][nx] == data[y][x] + 1:
            paths = (tuple([(y,x)] + list(path)) for path in trails_from(ny, nx))
            trailhead_ends |= set(paths)
    return trailhead_ends

trails = [trails_from(y, x) for y, row in enumerate(data) for x, v in enumerate(row) if v == 0]
print("Problem 19:", sum(len(set(path[-1] for path in trailhead)) for trailhead in trails))
print("Problem 20:", sum(len(trailhead) for trailhead in trails))
