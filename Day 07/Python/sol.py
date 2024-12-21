"""
FILE: Day 07/Python/sol.py
Author: Alex Jones
Desc: Solution to day 7 problems (13 & 14) for Advent of Code 2024, solved in Python 3.
"""
data = open("Day 07/data.txt", "r").read().strip().replace("\r","").split("\n")
data = [(int(row.split(": ")[0]), list(map(int, row.split(": ")[1].split()))) for row in data]

def satisfies(target, current, vals, start, operators):
    if current < target or start >= len(vals):
        return current == target
    return any(satisfies(target, op(current, vals[-start]), vals, start+1, operators) for op in operators)

def solve(data, operators):
    return sum([target for (target, vals) in data if satisfies(vals[0], target, vals, 1, operators)])

intdiv = lambda x,y: -1 if x % y != 0 else x // y
deconcat = lambda x,y: -1 if not str(x).endswith(str(y)) or x == y else int(str(x).removesuffix(str(y)))
operators = [lambda x,y: x-y, intdiv, deconcat]
print("Problem 13:", solve(data, operators[:-1]))
print("Problem 14:", solve(data, operators))