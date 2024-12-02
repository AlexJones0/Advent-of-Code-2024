"""
FILE: Day 02/Python/sol.py
Author: Alex Jones
Desc: Solution to day 2 problems (3 & 4) for Advent of Code 2024, solved in Python 3.
"""
data = open("Day 02/data.txt", "r").read().strip().replace("\r","").split("\n")

def is_safe(row: list[int]) -> bool:
    diffSet = set([row[i] - row[i-1] for i in range(1,len(row))])
    return diffSet.issubset({1,2,3}) or diffSet.issubset({-1,-2,-3})

data = [[int(x) for x in xs.split()] for xs in data]
print("Problem 3:", sum(is_safe(row) for row in data))

print("Problem 4:", sum(any(is_safe(row[:i]+row[i+1:]) for i in range(len(row)))
                        for row in data))
