"""
FILE: Day 05/Python/sol.py
Author: Alex Jones
Desc: Solution to day 5 problems (9 & 10) for Advent of Code 2024, solved in Python 3.
"""
data = open("Day 05/data.txt", "r").read().strip().replace("\r","").split("\n\n")
orderings = [tuple(x.split("|")) for x in data[0].split("\n")]
updates = [x.split(",") for x in data[1].split("\n")]

from collections import defaultdict
from functools import cmp_to_key

comes_before = defaultdict(set)
for (pred, succ) in orderings:
    comes_before[pred].add(succ)
p1, p2 = 0, 0
for update in updates:
    ordered = sorted(update, key=cmp_to_key(lambda fst, snd: -1 * (snd in comes_before[fst])))
    p1 += int(update[len(update) // 2]) if update == ordered else 0
    p2 += int(ordered[len(ordered) // 2]) if update != ordered else 0
print("Problem 9:", p1)
print("Problem 10:", p2)
