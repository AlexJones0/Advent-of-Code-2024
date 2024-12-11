"""
FILE: Day 11/Python/sol.py
Author: Alex Jones
Desc: Solution to day 11 problems (21 & 22) for Advent of Code 2024, solved in Python 3.
"""
data = list(map(int,open("Day 11/data.txt", "r").read().replace("\r","").strip().split()))

import functools

def blink(stone: int) -> list[int]:
    if stone == 0:
        return [1]
    elif len(str(stone)) % 2 == 0:
        mid = len(str(stone)) // 2
        return [int(str(stone)[:mid]), int(str(stone)[mid:])]
    return [stone * 2024]

@functools.cache
def stones_after(stone: int, blinks: int) -> int:
    if blinks == 0:
        return 1
    return sum(stones_after(st, blinks-1) for st in blink(stone))

print("Problem 21:", sum(stones_after(stone, 25) for stone in data))
print("Problem 22:", sum(stones_after(stone, 75) for stone in data))
