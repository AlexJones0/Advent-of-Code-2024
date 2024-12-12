"""
FILE: Day 12/Python/sol.py
Author: Alex Jones
Desc: Solution to day 12 problems (23 & 24) for Advent of Code 2024, solved in Python 3.
"""
data = open("Day 12/data.txt", "r").read().strip().replace("\r","").split("\n")
data = {(y + x * 1j): c for y, row in enumerate(data) for x, c in enumerate(row)}
UNIT_CARDINALS = [1, 1j, -1, -1j]

def contibuted_sides(pos: complex, dir_: complex, perimeter: set[tuple[complex,complex]]) -> int:
    adjacent = [(pos + dir_ * -1j, dir_), (pos + dir_ * 1j, dir_)]
    if all(side in perimeter for side in adjacent):
        return -1
    elif all(side not in perimeter for side in adjacent):
        return 1
    return 0

def get_region(data: dict[complex,str], start: complex) -> tuple[set[complex],int,int,int]:
    visited, perimeter = set([]), set([])
    area, sides = 0, 0
    plots = [start]
    while plots:
        if (pos := plots.pop()) in visited or data[start] != data[pos]:
            continue
        visited.add(pos)
        area += 1
        for dir_ in UNIT_CARDINALS:
            ngb = pos + dir_
            if ngb in visited:
                sides -= contibuted_sides(ngb, -dir_, perimeter)
                perimeter.remove((ngb, -dir_))
            else:
                sides += contibuted_sides(pos, dir_, perimeter)
                perimeter.add((pos, dir_))
                if ngb in data:
                    plots.append(ngb)
    return (visited, area, len(perimeter), sides)

def get_regions(data: dict[complex,str]) -> list[tuple[int,int,int]]:
    visited, regions = set(), []
    for pos in data.keys():
        if pos in visited:
            continue
        region = get_region(data, pos)
        regions.append(tuple(region[1:]))
        visited |= region[0]
    return regions

regions = get_regions(data)
print("Problem 23:", sum(a * p for (a, p, _) in regions))
print("Problem 24:", sum(a * s for (a, _, s) in regions))
