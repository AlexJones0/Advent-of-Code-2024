"""
FILE: Day 01/Python/sol.py
Author: Alex Jones
Desc: Solution to day 1 problems (1 & 2) for Advent of Code 2024, solved in Python 3.
"""
data = open("Day 01/data.txt", "r").read().replace("\r","").split("\n")

data = [[int(n) for n in row.split()] for row in data]
lists = list(map(sorted, [*zip(*data)]))
print("Problem 1:", sum([abs(i - j) for (i,j) in zip(*lists)]))

from collections import Counter
counts = Counter(lists[1])
print("Problem 2:", sum([i*counts[i] for i in lists[0]]))
