"""
FILE: Day 15/Python/sol.py
Author: Alex Jones
Desc: Solution to day 15 problems (29 & 30) for Advent of Code 2024, solved in Python 3.
"""
data = open("Day 15/data.txt", "r").read().strip().replace("\r","").split("\n\n")
grid = [[c for c in row] for row in data[0].split("\n")]
moves = data[1].replace("\n","")

move_map = {"<": (0, -1), "^": (-1, 0), ">": (0, 1), "v": (1, 0)}
widen_map = {'@': "@.", 'O': "[]", '#': "##", '.': ".."}

def can_move(grid, pos, dir_) -> bool:
    (ny, nx) = (pos[0] + dir_[0], pos[1] + dir_[1])
    if grid[ny][nx] in '[]':
        left  = (ny, nx) if grid[ny][nx] == '[' else (ny, nx - 1)
        right = (ny, nx) if grid[ny][nx] == ']' else (ny, nx + 1)
        return (dir_ == (0,  1) or can_move(grid, left, dir_)) and \
               (dir_ == (0, -1) or can_move(grid, right, dir_))
    elif grid[ny][nx] == 'O':
        return can_move(grid, (ny, nx), dir_)
    return grid[ny][nx] != '#'

def make_move(grid, pos, dir_, partial=False):
    (ny, nx) = (pos[0] + dir_[0], pos[1] + dir_[1])
    if not partial and dir_[0] != 0:
        if grid[pos[0]][pos[1]] == '[':
            make_move(grid, (pos[0], pos[1] + 1), dir_, partial=True)
        elif grid[pos[0]][pos[1]] == ']':
            make_move(grid, (pos[0], pos[1] - 1), dir_, partial=True)
    if grid[ny][nx] != '.':
        make_move(grid, (ny, nx), dir_)
    grid[ny][nx] = grid[pos[0]][pos[1]]
    grid[pos[0]][pos[1]] = '.'

def simulate(grid, moves):
    robot = min(((i, j)) for i, r in enumerate(grid) for j, c in enumerate(r) if c == '@')
    for move in moves:
        dir_ = move_map[move]
        if can_move(grid, robot, dir_):
            make_move(grid, robot, dir_)
            robot = (robot[0] + dir_[0], robot[1] + dir_[1])

def get_total_gps(grid):
    return sum(100 * i + j for i, r in enumerate(grid) for j, c in enumerate(r) if c in 'O[')

p1 = [r.copy() for r in grid]
simulate(p1, moves)
print("Problem 29:", get_total_gps(p1))

p2 = [list("".join(widen_map[c] for c in r)) for r in grid]
simulate(p2, moves)
print("Problem 30:", get_total_gps(p2))
