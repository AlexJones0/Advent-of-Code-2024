"""
FILE: Day 07/Python/sol.py
Author: Alex Jones
Desc: Solution to day 7 problems (13 & 14) for Advent of Code 2024, solved in Python 3.
"""
NOT_IMPLEMENTED = "Not Yet Implemented"
data = open("Day 07/data.txt", "r").read().strip().replace("\r","").split("\n")
data = [(int(row.split(": ")[0]), list(map(int, row.split(": ")[1].split()))) for row in data]

def satisfies(target, current, vals, start, operators):
    if current > target or start >= len(vals):
        return current == target
    return any(satisfies(target, op(current, vals[start]), vals, start+1, operators) for op in operators)
    
def solve(data, operators):
    return sum([target for (target, vals) in data if satisfies(target, vals[0], vals, 1, operators)])

operators = [lambda x,y: x+y, lambda x,y: x*y]
print("Problem 13:", solve(data, operators))
print("Problem 14:", solve(data, operators + [lambda x,y: int(str(x)+str(y))]))
