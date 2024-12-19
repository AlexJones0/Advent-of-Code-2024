"""
FILE: Day 19/Python/sol.py
Author: Alex Jones
Desc: Solution to day 19 problems (37 & 38) for Advent of Code 2024, solved in Python 3.
"""
data = open("Day 19/data.txt", "r").read().strip().replace("\r","").split("\n\n")
patterns = [p.strip() for p in data[0].split(",")]
designs = data[1].split("\n")

from functools import cache

@cache
def ways(d: str) -> int:
    return 1 if d == "" else sum(ways(d[len(p):]) for p in patterns if d.startswith(p))

print("Problem 37:", sum(ways(d) > 0 for d in designs))
print("Problem 38:", sum(ways(d) for d in designs))
