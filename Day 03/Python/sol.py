"""
FILE: Day 03/Python/sol.py
Author: Alex Jones
Desc: Solution to day 3 problems (5 & 6) for Advent of Code 2024, solved in Python 3.
"""
data = open("Day 03/data.txt", "r").read().strip().replace("\r","")

import re

def calc(muls: list[str]) -> int:
    operands = [list(map(int, re.findall(r"[0-9]{1,3}", stmt))) for stmt in muls]
    return sum([op[0] * op[1] for op in operands])

stmts = re.findall(r"(mul\([0-9]{1,3},[0-9]{1,3}\)|do\(\)|don't\(\))", data)
print("Problem 5:", calc(filter(lambda stmt: stmt[0] == "m", stmts)))

filtered = []
state = True
for stmt in stmts:
    if stmt in {"do()", "don't()"}:
        state = stmt == "do()"
    elif state:
        filtered.append(stmt)
print("Problem 6:", calc(filtered))
