"""
FILE: Day 04/Python/sol.py
Author: Alex Jones
Desc: Solution to day 4 problems (7 & 8) for Advent of Code 2024, solved in Python 3.
"""
data = open("Day 04/data.txt", "r").read().strip().replace("\r","").split("\n")

unit_cardinals = [(0, 1), (0, -1), (1, 0), (-1, 0)]
unit_intercardinals = [(i,j) for i in (-1,1) for j in (-1,1)]
height, width = len(data), len(data[0])
matches = 0
for i in range(height):
    for j in range(width):
        for dir_ in (unit_cardinals + unit_intercardinals):
            if not(0 <= i + dir_[0]*3 < height and 0 <= j + dir_[1]*3 < width):
                continue
            for k in range(4):
                ni, nj = i + dir_[0]*k, j + dir_[1]*k
                if data[ni][nj] != "XMAS"[k]:
                    break
            if k == 3 and data[ni][nj] == "S":
                matches += 1
print("Problem 7:", matches)

matches = 0
for i in range(1,height-1):
    for j in range(1,width-1):
        if data[i][j] != "A":
            continue
        matches += sum([    data[i+dir_[0]][j+dir_[1]] == "M" 
                        and data[i-dir_[0]][j-dir_[1]] == "S" 
                        for dir_ in unit_intercardinals]) == 2
print("Problem 8:", matches)
